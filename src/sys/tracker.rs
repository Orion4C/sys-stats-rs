use std::collections::HashMap;
use std::ffi::OsString;
use sysinfo::System;

use crate::config::parameters::Parameters;
use crate::proc::instance::Instance;

pub struct Tracker {
    sys: System,
    instances: HashMap<OsString, Instance>,
    updates: u64,
}

impl Tracker {
    pub fn new() -> Self {
        Self {
            sys: System::new(),
            instances: HashMap::new(),
            updates: 0,
        }
    }

    /// Advances the tracker by one tick: refreshes system data, increments the
    /// global update count, then for each live process updates the instance
    /// tracked under that process's name — creating one the first time a name
    /// is seen.
    pub fn update(&mut self) {
        self.updates = self.updates.saturating_add(1);
        self.sys.refresh_all();
        for (_, proc) in self.sys.processes() {
            let key = proc.name().to_os_string();
            self.instances
                .entry(key)
                .and_modify(|instance| instance.update(proc))
                .or_insert_with(|| Instance::new(proc));
        }
    }

    pub fn get_update_count(&self) -> f32 {
        self.updates as f32
    }

    /// Returns the tracked instances, optionally filtered by `parameters`.
    ///
    /// With `None`, returns every tracked instance unfiltered. With `Some`, returns
    /// only instances that both exceed at least one metric threshold (see
    /// [`Instance::passes_min_parameters`]) and have been seen for a large enough
    /// share of update ticks to clear the configured minimum uptime percentage.
    pub fn get_instances_against_parameters(
        &self,
        parameters: Option<Parameters>,
    ) -> Vec<&Instance> {
        if parameters.is_none() {
            return self.instances.iter().map(|x| x.1).collect();
        }
        let param = parameters.unwrap();
        self.instances
            .iter()
            .filter(|(_, instance)| {
                instance.passes_min_parameters(&param)
                    && ((instance.get_uptime() as f32 / self.updates as f32) * 100.00)
                        > param.get_min_uptime_percentage()
            })
            .map(|(_, instance)| instance)
            .collect()
    }
}
