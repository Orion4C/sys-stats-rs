use std::collections::HashMap;
use std::ffi::OsString;
use strum::IntoEnumIterator;
use sysinfo::{Process, System};

use crate::config::parameters::Parameters;
use crate::config::types::Usage;
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

    pub fn add_instance(&mut self, proc: &Process) {
        let s = proc.name().to_owned();
        if self.instances.contains_key(&s) {
            self.instances.entry(s).and_modify(|k| k.update(proc));
        } else {
            self.instances.insert(s, Instance::new(proc));
        }
    }

    pub fn get_update_count(&self) -> f32 {
        return self.updates as f32;
    }

    pub fn get_instances_against_parameters(
        &self,
        parameters: Option<Parameters>,
    ) -> Vec<&Instance> {
        let mut res: Vec<&Instance> = Vec::new();
        if parameters.is_none() {
            println!("Printing All");
            res = self.instances.iter().map(|x| x.1).collect();
            return res;
        }
        let param = parameters.unwrap();
        for usage in Usage::iter() {
            let temp: Vec<&Instance> = self
                .instances
                .iter()
                .filter(|(_, instance)| {
                    param.is_greater_than_min(usage, instance.get_stat_avg(usage))
                })
                .map(|(_, instance)| instance)
                .collect();
            for instance in temp {
                if !res.contains(&instance) {
                    res.push(instance);
                }
            }
        }
        res
    }
}
