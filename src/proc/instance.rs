use std::ffi::OsString;
use strum::IntoEnumIterator;
use sysinfo::Pid;
use sysinfo::Process;

use crate::{config::parameters::Parameters, config::types::Usage, proc::stats::Stats};

pub struct Instance {
    pid: Pid,
    name: OsString,
    uptime: u64,

    metrics: Stats,
}

impl Instance {
    pub fn new(proc: &Process) -> Self {
        Self {
            pid: proc.pid(),
            name: proc.name().to_owned(),
            uptime: 1,
            metrics: Stats::new(proc),
        }
    }

    /// Records one observation of this process: increments the uptime tick
    /// counter and folds the latest metrics into the running averages.
    pub fn update(&mut self, proc: &Process) {
        self.uptime = self.uptime.saturating_add(1);
        self.metrics.update(proc);
    }

    pub fn get_pid(&self) -> &Pid {
        &self.pid
    }

    pub fn get_name(&self) -> &OsString {
        &self.name
    }

    pub fn get_uptime(&self) -> f32 {
        self.uptime as f32
    }

    /// Returns the running average for the given [`Usage`] metric.
    ///
    /// # Panics
    ///
    /// Panics if `usage` has no entry in the tracked metrics.
    pub fn get_stat_avg(&self, usage: Usage) -> f32 {
        self.metrics.get_stat_avg(usage)
    }

    /// Checks this instance's averaged metrics against the configured minimums.
    ///
    /// Returns `true` if *any* single metric's average is equal or greater than
    /// its corresponding minimum in `param`
    pub fn passes_min_parameters(&self, param: &Parameters) -> bool {
        for usage in Usage::iter() {
            if param.get_min_usage(usage) <= self.metrics.get_stat_avg(usage) {
                return true;
            }
        }
        false
    }
}
