use crate::{config::parameters::Parameters, sys::tracker::Tracker};

pub mod config;
pub mod proc;
pub mod sys;

fn main() {
    let mut tracker = Tracker::new();
    let params = Parameters::new(0.0, 0.0, 0.0, 0.0, 100.0, None);
    for _ in 0..100 {
        tracker.update();
        std::thread::sleep(params.get_update_time());
    }
    for instance in tracker.get_instances_against_parameters(Some(params)) {
        println!(
            "Process: {:?}\nCpu: {}\nMem: {}\nDiskRead: {}\nDiskWrite: {},\nUptime: {:.2}%",
            instance.get_name(),
            instance.get_stat_avg(config::types::Usage::Cpu),
            instance.get_stat_avg(config::types::Usage::Memory),
            instance.get_stat_avg(config::types::Usage::DiskRead),
            instance.get_stat_avg(config::types::Usage::DiskWrite),
            (instance.get_uptime() / tracker.get_update_count()) * 100.0
        )
    }
}
