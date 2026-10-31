---
tags: [effective-rust]
max_turns: 40
timeout_seconds: 900
allowed_tools: [Read, Glob, Grep, Skill, Write, Edit, Bash]
---

Finish the `fileproc` tool. It takes a directory path as its only argument and must print `<file name> <checksum>` for every regular file directly inside that directory (no recursion). Requirements from the ops team:

- process files concurrently, but never more than 4 at a time (some files are several GB and `checksum` is CPU heavy, keep that function as it is)
- on Ctrl-C, stop starting new files, let the ones in flight finish, print what was computed, exit with status 0
- a file that cannot be read should be reported on stderr with its name and the reason, and must not abort the others

Keep it on tokio since the rest of our tooling is. Make sure it builds cleanly.

Input files: fileproc/Cargo.toml, fileproc/src/main.rs
