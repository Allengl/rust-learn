use std::time::Instant;

use rayon::prelude::*;

fn main() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    let func = || println!("Hello");

    pool.join(func,func);
}
