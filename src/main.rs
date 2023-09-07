use rand::RngCore;
use std::time::{Instant, Duration};

use md2::digest;

fn benchmark(iterations: i32) -> Duration {
    let start_time = Instant::now();
    for _ in 0..iterations {
        let mut data = [0u8; 64];
        rand::thread_rng().fill_bytes(&mut data);
        digest(&data);
    }
    return start_time.elapsed();
}

fn main() {
    // warm up
    for _ in 0..5 {
        benchmark(100);
    }
    for i in [100, 1_000, 5_000].iter() {
        let duration = benchmark(*i);
        println!("{} iterations: {} ms", i, duration.as_millis());
    }
}