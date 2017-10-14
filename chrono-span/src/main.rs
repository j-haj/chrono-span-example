#[macro_use]
extern crate chrono;
extern crate error_chain;

use chrono::prelude::*;

error_chain!{}

fn sum_less_than_n(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n {
        sum += i;
    }
    sum
}

fn run() -> Result<()> {
    Ok(())
}

quick_main!(run);
