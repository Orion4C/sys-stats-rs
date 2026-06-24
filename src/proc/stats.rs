use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::config::types::{self, Usage};
use sysinfo::Process;

#[derive(PartialEq)]
struct Stat {
    current: f32,
    average: f32,
    counter: f32,
}

impl Stat {
    fn new(val: f32) -> Self {
        Self {
            current: val,
            average: val,
            counter: 1.0,
        }
    }

    fn update(&mut self, val: f32) {
        self.current = val;
        self.average = ((self.average * self.counter) + val) / (self.counter + 1.0);
        self.counter = self.counter + 1.0;
    }
}

#[derive(PartialEq)]
pub struct Stats {
    tracked: HashMap<types::Usage, Stat>,
}

impl Stats {
    pub fn new(proc: &Process) -> Self {
        let mut res: HashMap<types::Usage, Stat> = HashMap::new();
        for usage in types::Usage::iter() {
            res.insert(usage, Stat::new(usage.get_usage(proc)));
        }
        Self { tracked: res }
    }

    pub fn update(&mut self, proc: &Process) {
        for (usage, stat) in &mut self.tracked {
            let value = usage.get_usage(proc);
            stat.update(value);
        }
    }

    pub fn get_stat_avg(&self, usage: Usage) -> f32 {
        return self.tracked.get(&usage).unwrap().average;
    }
}
