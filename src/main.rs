use std::ptr;

use sysinfo::System;

use crate::config::{parameters::Parameters, types::Usage};
use crate::sys::tracker::ProcessTracker;
use crate::utility::conversions::BytesConversion;

pub mod config;
pub mod proc;
pub mod sys;
pub mod utility;

fn main() {
    let mut ptracker = ProcessTracker::new();
    let mut sys = System::new();
    let params = Parameters::new(
        100.0,
        BytesConversion::from_gb(0.5),
        BytesConversion::from_mb(20.0),
        BytesConversion::from_mb(20.0),
        0.0,
        None,
    );
    for _ in 0..10 {
        sys.refresh_all();
        ptracker.update(&sys);
        std::thread::sleep(params.get_update_time());
    }
    for (name, instances) in &ptracker.instances {
        println!("----------");
        println!("Process: {:?}", name);
        if instances.len() > 1 {
            println!("~process contains: {} instances", instances.len());
        }
        for instance in instances {
            println!("---");
            println!("id: {}", instance.get_pid());
            println!(
                "runtime: {:.2}s - {:.2}% total",
                instance.get_runtime().num_seconds(),
                ptracker.process_runtime_percentage(instance)
            );
            println!("cpu: {:.2}%", instance.get_stat_avg(Usage::Cpu));
            println!(
                "mem: {:.2}mb",
                BytesConversion::to_mb(instance.get_stat_avg(Usage::Memory))
            );
        }
    }
}
