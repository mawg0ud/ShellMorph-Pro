use sysinfo::{System, SystemExt, ProcessorExt};

pub struct PerformanceMonitor {
    system: System,
}

impl PerformanceMonitor {
    /// Creates a new performance monitor instance.
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }

    /// Logs the CPU usage for each processor.
    pub fn log_cpu_usage(&mut self) {
        self.system.refresh_cpu();
        for (i, cpu) in self.system.processors().iter().enumerate() {
            println!("CPU {}: {:.2}%", i, cpu.cpu_usage());
        }
    }

    /// Logs the total RAM and available memory.
    pub fn log_memory_usage(&mut self) {
        self.system.refresh_memory();
        let total_memory = self.system.total_memory() / 1024;
        let available_memory = self.system.available_memory() / 1024;

        println!(
            "Memory Usage: Total: {} MB, Available: {} MB",
            total_memory, available_memory
        );
    }
}

fn main() {
    let mut monitor = PerformanceMonitor::new();

    println!("Starting performance monitoring...");
    monitor.log_cpu_usage();
    monitor.log_memory_usage();
}
