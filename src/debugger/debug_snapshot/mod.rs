pub mod cpu_snapshot;
use self::cpu_snapshot::CPUSnapshot;


pub struct DebugSnapshot {
    pub cpu: CPUSnapshot,
}