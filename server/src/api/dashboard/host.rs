use super::*;

use sysinfo::System;

pub(super) fn collect_host_ops_summary() -> HostOpsSummary {
    let mut system = System::new_all();
    system.refresh_memory();

    let load_average = System::load_average();
    let available_memory = {
        let value = system.available_memory();
        if value == 0 {
            system.free_memory()
        } else {
            value
        }
    };

    HostOpsSummary {
        hostname: System::host_name(),
        os_name: System::name(),
        os_version: System::os_version(),
        kernel_version: System::kernel_version(),
        architecture: Some(std::env::consts::ARCH.to_string()),
        uptime_seconds: System::uptime(),
        cpu_cores: System::physical_core_count()
            .or_else(|| std::thread::available_parallelism().ok().map(usize::from)),
        load_average: HostLoadAverage {
            one: load_average.one,
            five: load_average.five,
            fifteen: load_average.fifteen,
        },
        memory: HostMemorySummary {
            total_bytes: system.total_memory(),
            used_bytes: system.used_memory(),
            available_bytes: available_memory,
        },
    }
}
