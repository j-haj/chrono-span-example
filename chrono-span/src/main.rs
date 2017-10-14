#[macro_use]
extern crate error_chain;
extern crate chrono;

use chrono::Duration;
use chrono::prelude::*;

error_chain!{}

fn sum_less_than_n(n: i64) {
    let mut sum: i64 = 0;
    for i in 1..n {
        sum = sum + i;
    }
}

fn run() -> Result<()> {
    let d = Duration::span(|| sum_less_than_n(100000));
    println!("{:?}", d);
    Ok(())
}

quick_main!(run);
