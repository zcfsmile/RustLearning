use std::time::Instant;
use rayon::prelude::*;

fn main() {
    let data: Vec<i64> = (0..=1000_000_000).collect();

    // 单线程求和
    let start = Instant::now();
    let sum1 = data.iter().sum::<i64>();
    let duration = start.elapsed();
    println!("Sum1: {}, duration: {:?}", sum1, duration);
    
    
    // 并行迭代求和
    let start2 = Instant::now();
    let sum2: i64 = data.par_iter().sum();
    let duration2 = start2.elapsed();
    println!("Sum2: {}, duration: {:?}", sum2, duration2);
}
// Sum1: 500000000500000000, duration: 7.616593083s
// Sum2: 500000000500000000, duration: 2.744974292s
