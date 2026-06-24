use std::ffi::OsString;
use sysinfo::{Pid, Process};

use crate::{config::types::Usage, proc::stats::Stats};

#[derive(PartialEq)]
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
            uptime: 0,
            metrics: Stats::new(proc),
        }
    }

    pub fn update(&mut self, proc: &Process) {
        self.uptime = self.uptime.saturating_add(1);
        self.metrics.update(proc);
    }

    pub fn get_name(&self) -> &OsString {
        return &self.name;
    }

    pub fn get_uptime(&self) -> f32 {
        return self.uptime as f32;
    }

    pub fn match_pid(&self, proc: &Process) -> bool {
        if self.pid != proc.pid() {
            return false;
        }
        true
    }

    pub fn match_name(&self, proc: &Process) -> bool {
        if self.name != proc.name() {
            return false;
        }
        true
    }

    pub fn get_stat_avg(&self, usage: Usage) -> f32 {
        return self.metrics.get_stat_avg(usage);
    }
}
