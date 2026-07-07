// Shared stuff

use std::io::{Error, ErrorKind};
use std::arch::x86_64::__rdtscp;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub const BUF_SIZE: usize = 4096;

#[inline(always)]
pub fn to_microsecs(nanosecs: u64) -> f64 {
    nanosecs as f64 / 1_000.0
}

#[inline(always)]
pub fn read_tsc() -> u64 {
    let mut aux = 0u32; // just needs initiation, value doesnt matter
    unsafe { __rdtscp(&mut aux) }
}

#[inline(always)]
pub fn args_error(msg: String) -> Result<(), Error> {
    Err(Error::new(ErrorKind::InvalidInput, msg))
}

pub fn mean(v: &[u64]) -> Option<f64> {
    if v.is_empty() { return None }
    Some(v.iter().sum::<u64>() as f64 / (v.len() as f64))
}

pub fn stddev(v: &[u64], mean: f64) -> Option<f64> {
    if v.len() < 2 { return None }
    let mut sum = 0.0;
    for val in v {
        sum += (*val as f64 - mean).powi(2);
    }
    Some( (sum / (v.len()-1) as f64).sqrt() )
}

pub fn measure_cycles_per_nanosecond() -> f64 {
    let start_time = Instant::now();
    let start_cycles = read_tsc();
    
    sleep(Duration::from_millis(100));
    
    let end_cycles = read_tsc();
    let end_time = Instant::now();
    
    let elapsed_ns = end_time.duration_since(start_time).as_nanos() as f64;
    let elapsed_cycles = (end_cycles - start_cycles) as f64;
    
    elapsed_cycles / elapsed_ns
}