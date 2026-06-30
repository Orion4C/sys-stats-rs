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
        self.counter += 1.0;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_counts_as_first_sample() {
        let s = Stat::new(7.0);
        assert_eq!(s.average, 7.0);
        assert_eq!(s.counter, 1.0);
    }

    #[test]
    fn average_of_two_values() {
        let mut s = Stat::new(10.0);
        s.update(20.0);
        assert_eq!(s.average, 15.0);
    }

    #[test]
    fn average_of_known_sequence() {
        let mut s = Stat::new(2.0);
        s.update(4.0);
        s.update(6.0);
        assert_eq!(s.average, 4.0);
    }

    #[test]
    fn constant_series_does_not_drift() {
        let mut s = Stat::new(5.0);
        for _ in 0..100 {
            s.update(5.0);
        }
        assert_eq!(s.average, 5.0);
    }

    #[test]
    fn update_records_current_value() {
        let mut s = Stat::new(1.0);
        s.update(99.0);
        assert_eq!(s.current, 99.0);
    }
}
