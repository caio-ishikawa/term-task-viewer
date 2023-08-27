use std::error::Error;

use psutil::cpu;
use psutil::memory;

#[derive(Clone)]
pub struct SystemData {
    pub ram_total: u64,
    pub ram_used: u64,
    pub cpu_percent: f64,
}

pub fn generate_system_data() -> Result<SystemData, Box<dyn Error>> {
    let v_mem_info = memory::virtual_memory()?;
    let mem_mb = v_mem_info.total();
    let mem_used = v_mem_info.used();

    let cpu_times = cpu::cpu_times()?;
    let total_time = (cpu_times.user() + cpu_times.system() + cpu_times.idle()).as_secs_f64();
    let usage_time = total_time - cpu_times.idle().as_secs_f64();
    let cpu_percent = (usage_time / total_time) * 100.0;

    Ok(SystemData {
        ram_total: mem_mb,
        ram_used: mem_used,
        cpu_percent,
    })
}
