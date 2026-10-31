# Async Rust

Read before writing or reviewing any `async fn`, `tokio::spawn`, `select!`,
channel or runtime setup. Sources: Tokio tutorial, Async Book, Alice Ryhl's
"Async: What is blocking?" and "Actors with Tokio", `tokio::select!` docs.

## Do you need async at all?

Async pays off for many concurrent I/O-bound operations (sockets, HTTP,
timers). A CLI that reads three files, a batch job, or CPU-heavy work is
simpler and faster with threads (`std::thread::scope`, `rayon`). Do not add
`tokio` to a crate that does not wait on I/O.

## Runtime

- Use the runtime the project already uses; do not mix `tokio` with
  `async-std`/`smol` in one binary. Libraries avoid depending on a runtime
  where possible, or gate it behind a feature.
- `#[tokio::main]` on the binary, `#[tokio::test]` on tests. Libraries never
  create a runtime internally (no `Runtime::new().block_on` in library code).
- Enable only the tokio features you use (`rt-multi-thread`, `macros`,
  `net`, `time`, `sync`, `fs`, `io-util`), matching the project's
  `default-features = false` style.

## Blocking is the cardinal sin

A task that blocks the thread stalls every other task on that worker. Rule
of thumb from Ryhl: anything that takes more than 10–100 µs without yielding
is blocking.

| Blocking | Use instead |
|---|---|
| `std::fs::*`, `std::net::*`, `std::io::stdin` | `tokio::fs`, `tokio::net`, `tokio::io` |
| `std::thread::sleep` | `tokio::time::sleep` |
| `reqwest::blocking`, `ureq`, `rusqlite` sync calls | async client, or `spawn_blocking` |
| CPU-bound loop, compression, hashing, parsing MBs | `tokio::task::spawn_blocking`, or `rayon` with a `oneshot` to return the result |
| `std::sync::Mutex` held across `.await` | Lock, copy what you need, drop the guard before awaiting; or `tokio::sync::Mutex` only when the critical section must contain an await |

`std::sync::Mutex` is the right lock for short critical sections in async
code; `tokio::sync::Mutex` is slower and only needed when holding across an
await.

## Spawning and structured concurrency

- `tokio::spawn` requires `Send + 'static`: move owned data in, clone `Arc`
  handles outside the closure.
- Collect handles; a dropped `JoinHandle` detaches the task and loses its
  error. Use `JoinSet` for dynamic groups, `tokio::join!`/`try_join!` for a
  fixed set, `futures::stream::iter(..).buffer_unordered(n)` or a
  `Semaphore` for bounded parallelism.
- Independent operations run concurrently, not in a `for` loop of awaits.

```rust
let mut set = JoinSet::new();
for job in jobs {
    let client = Arc::clone(&client);
    set.spawn(async move { client.run(job).await });
}
while let Some(res) = set.join_next().await {
    res??;  // JoinError, then the job's error
}
```

- Propagate panics: `JoinError::is_panic()`; do not swallow them.

## Cancellation

Dropping a future cancels it at its last `.await`. Consequences:

- `select!` drops the losing branches. A branch reading from a socket into a
  local buffer loses that data unless the future is cancellation safe. Read
  the "Cancellation safety" section of the `tokio::select!` docs for every
  call you put in a branch; `tokio::sync::mpsc::Receiver::recv` is safe,
  `AsyncReadExt::read_exact` is not.
- Keep the future in a variable outside the loop (`let fut = pin!(..)`) when
  it must survive multiple `select!` iterations.
- Use `tokio_util::sync::CancellationToken` or a `watch` channel for
  shutdown; propagate it to every long-running task; await all tasks before
  exiting so `Drop` code runs.
- `tokio::time::timeout(dur, fut)` for deadlines.

## Shared state: prefer an owning task

`Arc<Mutex<State>>` works for a counter. For anything with logic, give the
state to one task and send it messages: the actor pattern.

```rust
struct Handle { tx: mpsc::Sender<Msg> }
enum Msg { Get { reply: oneshot::Sender<u64> }, Inc }

async fn run(mut rx: mpsc::Receiver<Msg>) {
    let mut count = 0;
    while let Some(msg) = rx.recv().await {
        match msg {
            Msg::Inc => count += 1,
            Msg::Get { reply } => { let _ = reply.send(count); }
        }
    }
}
```

No locks, no deadlocks, back-pressure for free through the bounded channel.

Channel choice: `mpsc` (many producers, one consumer, bounded by default),
`oneshot` (single reply), `watch` (latest value, config/shutdown),
`broadcast` (fan-out, lossy under lag). Unbounded channels hide
back-pressure bugs; pick a bound.

## Traits

Native `async fn` in traits is stable (1.75). Use it. Reach for `async-trait`
only when you need `dyn Trait` with async methods; or return
`Pin<Box<dyn Future<Output = T> + Send + '_>>` explicitly. Add
`+ Send` bounds where the trait will be used from spawned tasks.

## Streams

`futures::Stream` plus `tokio_stream::StreamExt` for async iteration.
`async_stream::stream!` or `futures::stream::unfold` to build one. Do not
hand-roll `poll_next` unless implementing a primitive.

## Testing

- `#[tokio::test]` (single-threaded by default; `flavor = "multi_thread"`
  when the code relies on it).
- `tokio::time::pause()` and `advance()` to test timeouts without waiting.
- Test cancellation: drop the future mid-way and assert state is consistent.
- Deadlock suspects: `tokio-console` or `RUSTFLAGS="--cfg tokio_unstable"`
  tracing of task states.

## Observability

Use `tracing` with `#[instrument]` on async fns; spans survive across awaits,
`log` output does not correlate. `tracing::Instrument::instrument` for spawned
tasks.

## Checklist before you finish async code

- [ ] No `std` blocking calls or `thread::sleep` inside async fns.
- [ ] No `std::sync::MutexGuard` live across `.await`.
- [ ] Every spawned task's `JoinHandle` is awaited or deliberately detached
      with a comment.
- [ ] Each `select!` branch checked for cancellation safety.
- [ ] Shutdown path cancels and awaits all tasks.
- [ ] Channels are bounded.
- [ ] Runtime created only in the binary.
