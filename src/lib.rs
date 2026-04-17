// Shared stuff

use std::io::{Error, ErrorKind};
use std::time::{SystemTime, UNIX_EPOCH};

pub const BUF_SIZE: usize = 4096;

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

#[inline(always)]
pub fn args_error(msg: String) -> Result<(), Error> {
    Err(Error::new(ErrorKind::InvalidInput, msg))
}

#[inline(always)]
pub fn mean(v: &[f64]) -> Option<f64> {
    if v.is_empty() { return None }
    Some(v.iter().sum::<f64>() / (v.len() as f64))
}

#[inline(always)]
pub fn stddev(v: &[f64], mean: f64) -> Option<f64> {
    if v.is_empty() { return None }
    let mut sum = 0.0;
    for val in v {
        sum += (val - mean).powi(2);
    }
    Some((sum / (v.len() as f64)).sqrt())
}