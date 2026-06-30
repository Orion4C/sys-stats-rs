use clap::Parser;
use std::time::Duration;
use sysinfo::System;

use crate::config::{parameters::Parameters, types::Usage};
use crate::sys::tracker::ProcessTracker;
use crate::utility::conversions::BytesConversion;

pub mod config;
pub mod proc;
pub mod sys;
pub mod utility;

/// Track per-process system statistics and list resource-heavy processes.
#[derive(Parser)]
#[command(name = "sys-stats", version, about)]
struct Cli {
    /// Minimum average CPU usage (%) to list a process
    #[arg(long, default_value_t = 100.0)]
    min_cpu: f32,

    /// Minimum average memory (MB) to list a process
    #[arg(long, default_value_t = 500.0)]
    min_mem: f32,

    /// Minimum average disk read (MB) to list a process
    #[arg(long, default_value_t = 200.0)]
    min_disk_read: f32,

    /// Minimum average disk write (MB) to list a process
    #[arg(long, default_value_t = 200.0)]
    min_disk_write: f32,

    /// Minimum runtime percentage of the observation window
    #[arg(long, default_value_t = 0.0)]
    min_uptime: f32,

    /// Sampling interval in milliseconds (defaults to the sysinfo minimum if omitted)
    #[arg(long)]
    interval_ms: Option<u64>,

    /// Number of sampling iterations
    #[arg(long, default_value_t = 20)]
    iterations: u32,
}

impl Cli {
    fn into_parameters(self) -> Parameters {
        Parameters::new(
            self.min_cpu,
            BytesConversion::from_mb(self.min_mem),
            BytesConversion::from_mb(self.min_disk_read),
            BytesConversion::from_mb(self.min_disk_write),
            self.min_uptime,
            self.interval_ms.map(Duration::from_millis),
            self.iterations,
        )
    }
}

fn main() {
    let params = Cli::parse().into_parameters();

    let mut ptracker = ProcessTracker::new();
    let mut sys = System::new();

    for _ in 0..params.get_runtime_iterations() {
        sys.refresh_all();
        ptracker.update(&sys);
        std::thread::sleep(params.get_update_time());
    }

    let output = ptracker.get_trimmed_list(&params);
    for (name, instances) in &output {
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
