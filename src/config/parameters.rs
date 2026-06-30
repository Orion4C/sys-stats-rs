use std::time::Duration;

use crate::config::types::Usage;

pub struct Parameters {
    min_cpu: f32,
    min_mem: f32,
    min_disk_read: f32,
    min_disk_write: f32,
    min_uptime_percent: f32,

    update_time: Option<Duration>,

    runtime_iterations: u64,
}

impl Parameters {
    pub fn new(
        min_cpu: f32,
        min_mem: f32,
        min_disk_read: f32,
        min_disk_write: f32,
        min_uptime_percent: f32,
        update_time: Option<Duration>,
        runtime_iterations: u64,
    ) -> Self {
        Self {
            min_cpu,
            min_mem,
            min_disk_read,
            min_disk_write,
            min_uptime_percent,
            update_time,
            runtime_iterations,
        }
    }

    /// Returns the configured minimum threshold for the given [`Usage`] metric.
    pub fn get_min_usage(&self, usage: Usage) -> f32 {
        match usage {
            Usage::Cpu => self.min_cpu,
            Usage::Memory => self.min_mem,
            Usage::DiskRead => self.min_disk_read,
            Usage::DiskWrite => self.min_disk_write,
        }
    }

    pub fn get_min_uptime_percentage(&self) -> f32 {
        self.min_uptime_percent
    }

    pub fn get_runtime_iterations(&self) -> u64 {
        self.runtime_iterations
    }

    /// Returns the polling interval between metric updates.
    ///
    /// The result is always at least [`sysinfo::MINIMUM_CPU_UPDATE_INTERVAL`].
    /// If no interval was configured, or the configured one is shorter than that
    /// floor, the floor is returned instead
    pub fn get_update_time(&self) -> Duration {
        if self.update_time.is_none() {
            return sysinfo::MINIMUM_CPU_UPDATE_INTERVAL;
        }
        if self.update_time.unwrap() < sysinfo::MINIMUM_CPU_UPDATE_INTERVAL {
            return sysinfo::MINIMUM_CPU_UPDATE_INTERVAL;
        }
        self.update_time.unwrap()
    }
}
