use sysinfo::System;
use std::{thread, time::Duration};
use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::{stdout};

fn main() {
    let mut system = System::new_all();
    extern crate num_cpus;

    loop {
        // Refresh system info.
        system.refresh_all();

        // Clear the terminal.
        execute!(stdout(), Clear(ClearType::All)).unwrap();

        // Print system information.
        println!("CPU Usage: {:.2}%", system.global_cpu_info().cpu_usage());
        println!("Total Memory: {} MB", system.total_memory() / 1024);
        println!("Used Memory:  {} MB", system.used_memory() / 1024);
        println!("Running Processes: {}", system.processes().len());
        println!("Number of CPU cores: {}", num_cpus::get());

        // Wait for 1 second.
        thread::sleep(Duration::from_secs(1));
    }
}
