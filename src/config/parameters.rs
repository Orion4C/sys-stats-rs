use std::time::Duration;

use crate::config::types::Usage;

pub struct Parameters {
    min_cpu: f32,
    min_mem: f32,
    min_disk_read: f32,
    min_disk_write: f32,
    min_uptime_percent: f32,

    update_time: Option<Duration>,

    runtime_iterations: u32,
}

impl Parameters {
    pub fn new(
        min_cpu: f32,
        min_mem: f32,
        min_disk_read: f32,
        min_disk_write: f32,
        min_uptime_percent: f32,
        update_time: Option<Duration>,
        runtime_iterations: u32,
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

    pub fn get_runtime_iterations(&self) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn params(update_time: Option<Duration>) -> Parameters {
        Parameters::new(1.0, 2.0, 3.0, 4.0, 50.0, update_time, 20)
    }

    #[test]
    fn min_usage_maps_each_variant() {
        let p = params(None);
        assert_eq!(p.get_min_usage(Usage::Cpu), 1.0);
        assert_eq!(p.get_min_usage(Usage::Memory), 2.0);
        assert_eq!(p.get_min_usage(Usage::DiskRead), 3.0);
        assert_eq!(p.get_min_usage(Usage::DiskWrite), 4.0);
    }

    #[test]
    fn passthrough_getters() {
        let p = params(None);
        assert_eq!(p.get_min_uptime_percentage(), 50.0);
        assert_eq!(p.get_runtime_iterations(), 20);
    }

    #[test]
    fn update_time_defaults_to_floor_when_none() {
        assert_eq!(
            params(None).get_update_time(),
            sysinfo::MINIMUM_CPU_UPDATE_INTERVAL
        );
    }

    #[test]
    fn update_time_clamps_below_floor() {
        let p = params(Some(Duration::from_nanos(1)));
        assert_eq!(p.get_update_time(), sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }

    #[test]
    fn update_time_keeps_value_above_floor() {
        let big = sysinfo::MINIMUM_CPU_UPDATE_INTERVAL * 10;
        assert_eq!(params(Some(big)).get_update_time(), big);
    }
}
