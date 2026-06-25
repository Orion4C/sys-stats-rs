use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::config::types::{self, Usage};
use sysinfo::Process;

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

    /// Folds the process's current readings into every tracked metric's running
    /// average — one new sample per [`Usage`] channel.
    pub fn update(&mut self, proc: &Process) {
        for (usage, stat) in &mut self.tracked {
            let value = usage.get_usage(proc);
            stat.update(value);
        }
    }

    /// Returns the running average for the given [`Usage`] metric.
    ///
    /// # Panics
    ///
    /// Panics if `usage` has no entry in the tracked metrics.
    pub fn get_stat_avg(&self, usage: Usage) -> f32 {
        self.tracked
            .get(&usage)
            .expect("get_stat_avg() - Usage does not exist within tracked Stats")
            .average
    }
}
