use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use sysinfo::{Pid, System};

//use crate::config::parameters::Parameters;
use crate::proc::snapshot::ProcessSnapshot;

pub struct ProcessTracker {
    pub instances: HashMap<OsString, Vec<ProcessSnapshot>>,
}

impl ProcessTracker {
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
        }
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
