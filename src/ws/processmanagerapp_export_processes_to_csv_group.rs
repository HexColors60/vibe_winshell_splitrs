use std::fs::File;
//! # ProcessManagerApp - export_processes_to_csv_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    fn export_processes_to_csv(&self) -> Result<String, String> {
        let filename = format!(
            "processes_{}.csv", chrono::Local::now().format("%Y%m%d_%H%M%S")
        );
        let mut file = File::create(&filename).map_err(|e| e.to_string())?;
        writeln!(file, "PID,Name,Memory (bytes),CPU %,Parent PID,Status,Runtime (s)")
            .map_err(|e| e.to_string())?;
        for process in &self.processes {
            writeln!(
                file, "{},{},{},{:.2},{},{},{}", process.pid, process.name, process
                .memory, process.cpu_usage, process.parent_pid.map(| p | p.to_string())
                .unwrap_or_else(|| "-".to_string()), process.status, process.run_time
            )
                .map_err(|e| e.to_string())?;
        }
        Ok(filename)
    }
}
