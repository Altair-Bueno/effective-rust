//! Tiny `key = value` configuration parser.

/// Service configuration loaded from a config file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}
