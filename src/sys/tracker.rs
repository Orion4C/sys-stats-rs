use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use sysinfo::{Pid, System};

use crate::config::parameters::Parameters;
use crate::proc::snapshot::ProcessSnapshot;

pub struct ProcessTracker {
    pub instances: HashMap<OsString, Vec<ProcessSnapshot>>,
    start_time: DateTime<Utc>,
}

impl ProcessTracker {
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
            start_time: Utc::now(),
        }
    }

    pub fn process_runtime_percentage(&self, snapshot: &ProcessSnapshot) -> f32 {
        let tracker_runtime = (Utc::now() - self.start_time).num_milliseconds();
        if tracker_runtime <= 0 {
            return 0.0;
        }
        (snapshot.get_runtime().num_milliseconds() as f32 / tracker_runtime as f32) * 100.0
    }

    pub fn get_trimmed_list(
        &self,
        param: &Parameters,
    ) -> HashMap<&OsString, Vec<&ProcessSnapshot>> {
        let mut res: HashMap<&OsString, Vec<&ProcessSnapshot>> = HashMap::new();
        for (name, snaps) in &self.instances {
            let kept: Vec<&ProcessSnapshot> = snaps
                .iter()
                .filter(|snap| {
                    snap.passes_min_parameters(param)
                        && self.process_runtime_percentage(snap)
                            >= param.get_min_uptime_percentage()
                })
                .collect();
            if !kept.is_empty() {
                res.insert(name, kept);
            }
        }
        res
    }

    pub fn update(&mut self, sys: &System) {
        // update the live snapshot for each running process, or start one
        for (pid, proc) in sys.processes() {
            let vec = self.instances.entry(proc.name().to_owned()).or_default();
            match vec
                .iter_mut()
                .find(|p| p.get_pid() == pid && !p.has_endtime())
            {
                Some(live) => live.update(proc),
                None => vec.push(ProcessSnapshot::new(proc)),
            }
        }

        // anything still live but no longer in the OS gets an endtime
        let alive: HashSet<Pid> = sys.processes().keys().copied().collect();
        for snaps in self.instances.values_mut() {
            for snap in snaps.iter_mut() {
                if !snap.has_endtime() && !alive.contains(snap.get_pid()) {
                    snap.set_endtime();
                }
            }
        }
    }
}
