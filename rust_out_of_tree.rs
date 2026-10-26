// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

use kernel::cpu;
use kernel::prelude::*;
use kernel::time::{ClockSource, Monotonic};

module! {
    type: RustOutOfTree,
    name: "rust_out_of_tree",
    authors: ["Rust for Linux Contributors"],
    description: "Rust out-of-tree sample",
    license: "GPL",
}

struct RustOutOfTree {
    numbers: KVec<i32>,
    start_time: i64,
}

impl kernel::Module for RustOutOfTree {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        let start_time = Monotonic::ktime_get();
        pr_info!("Rust out-of-tree sample (init)\n");
        pr_info!("Number of CPUS: {}\n", cpu::nr_cpu_ids());
        pr_info!("My start time is {}\n", start_time);

        let mut numbers = KVec::new();
        numbers.push(72, GFP_KERNEL)?;
        numbers.push(108, GFP_KERNEL)?;
        numbers.push(200, GFP_KERNEL)?;

        Ok(RustOutOfTree {
            numbers,
            start_time,
        })
    }
}

impl Drop for RustOutOfTree {
    fn drop(&mut self) {
        let end_time = Monotonic::ktime_get();
        let elapsed = end_time - self.start_time;
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("My start time was {}\n", self.start_time);
        pr_info!("My end time is {end_time}\n");
        pr_info!("I was running for {elapsed} seconds\n");
        pr_info!("Rust out-of-tree sample (exit)\n");
    }
}
