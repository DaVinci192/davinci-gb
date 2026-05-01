pub mod cpu_snapshot;
pub mod bus_snapshot;

use self::cpu_snapshot::CPUSnapshot;
use self::bus_snapshot::BusSnapshot;



pub struct DebugSnapshot {
    pub cpu: CPUSnapshot,
    pub bus: BusSnapshot,
}

