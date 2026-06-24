use strum_macros::EnumIter;
use sysinfo::Process;

#[derive(EnumIter, Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Usage {
    Memory = 1,
    Cpu,
    DiskRead,
    DiskWrite,
}

impl Usage {
    pub fn get_usage(self, proc: &Process) -> f32 {
        match self {
            Usage::Memory => proc.memory() as f32,
            Usage::Cpu => proc.cpu_usage(),
            Usage::DiskRead => proc.disk_usage().read_bytes as f32,
            Usage::DiskWrite => proc.disk_usage().written_bytes as f32,
        }
    }
}
