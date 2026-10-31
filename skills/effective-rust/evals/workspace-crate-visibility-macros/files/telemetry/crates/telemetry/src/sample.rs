/// One reading from a device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sample {
    pub device_id: u32,
    pub timestamp_ms: u64,
    pub channel: String,
    pub values: Vec<i16>,
    pub valid: bool,
}
