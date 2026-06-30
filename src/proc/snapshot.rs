use chrono::{DateTime, TimeDelta, Utc};
use strum::IntoEnumIterator;
use sysinfo::{Pid, Process};

use crate::{config::parameters::Parameters, config::types::Usage, proc::stats::Stats};

pub struct ProcessSnapshot {
    process_id: Pid,
    process_name: std::ffi::OsString,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
    metrics: Stats,
}

impl ProcessSnapshot {
    pub fn new(proc: &Process) -> Self {
        Self {
            process_id: proc.pid(),
            process_name: proc.name().to_owned(),
            start_time: Utc::now(),
            end_time: None,
            metrics: Stats::new(proc),
        }
    }

    pub fn set_endtime(&mut self) {
        self.end_time = Some(Utc::now());
    }

    pub fn has_endtime(&self) -> bool {
        !self.end_time.is_none()
    }

    pub fn update(&mut self, proc: &Process) {
        self.metrics.update(proc);
    }

    pub fn get_pid(&self) -> &Pid {
        &self.process_id
    }

    pub fn get_name(&self) -> &std::ffi::OsString {
        &self.process_name
    }

    /// Returns the running Timedelta of the given snapshot
    pub fn get_runtime(&self) -> TimeDelta {
        Utc::now() - self.start_time
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
