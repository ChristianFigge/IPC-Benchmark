// Shared stuff

use std::time::{SystemTime, UNIX_EPOCH};

pub const BUF_SIZE_BYTES: usize = 4096; // Better be a multiple of 8

#[inline(always)]
pub fn to_microsecs(nanosecs: u64) -> f64 {
    nanosecs as f64 / 1_000.0
}

#[inline(always)]
pub fn get_timestamp_ns() -> u64 {
    SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_nanos()
    as u64
}