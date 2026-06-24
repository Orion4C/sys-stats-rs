use std::time::Duration;

use crate::config::types::Usage;

pub struct Parameters {
    min_cpu: f32,
    min_mem: f32,
    min_disk_read: f32,
    min_disk_write: f32,
    pub update_time: Option<Duration>,
}

impl Parameters {
    pub fn new(
        cpu: f32,
        mem: f32,
        disk_read: f32,
        disk_write: f32,
        update_time: Option<Duration>,
    ) -> Self {
        Self {
            min_cpu: cpu,
            min_mem: mem,
            min_disk_read: disk_read,
            min_disk_write: disk_write,
            update_time,
        }
    }

    pub fn is_greater_than_min(&self, usage: Usage, val: f32) -> bool {
        match usage {
            Usage::Cpu => {
                if self.min_cpu < val {
                    return true;
                }
                return false;
            }
            Usage::Memory => {
                if self.min_mem < val {
                    return true;
                }
                return false;
            }
            Usage::DiskRead => {
                if self.min_disk_read < val {
                    return true;
                }
                return false;
            }
            Usage::DiskWrite => {
                if self.min_disk_write < val {
                    return true;
                }
                return false;
            }
        }
    }
}
