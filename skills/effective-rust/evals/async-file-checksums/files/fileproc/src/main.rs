//! Prints a checksum for every file in a directory.

/// FNV-1a over the whole buffer. Deliberately simple and deliberately slow on
/// large inputs; do not change it.
#[must_use]
pub fn checksum(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dir = std::env::args_os()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("usage: fileproc <directory>"))?;
    let _ = dir;
    todo!("list the directory, checksum each file, handle Ctrl-C")
}
