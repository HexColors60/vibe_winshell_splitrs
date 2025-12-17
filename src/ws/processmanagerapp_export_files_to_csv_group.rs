use std::fs::File;
// # ProcessManagerApp - export_files_to_csv_group Methods
//
// This module contains method implementations for `ProcessManagerApp`.
//
// 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

use std::io::Write;
impl ProcessManagerApp {
    pub(crate) fn export_files_to_csv(&self) -> Result<String, String> {
        let filename = format!(
            "file_handles_{}.csv", chrono::Local::now().format("%Y%m%d_%H%M%S")
        );
        let mut file = File::create(&filename).map_err(|e| e.to_string())?;
        writeln!(file, "PID,Process Name,File Path,Size (bytes),Access Type")
            .map_err(|e| e.to_string())?;
        for fh in &self.file_handles {
            writeln!(
                file, "{},{},\"{}\",{},{}", fh.pid, fh.process_name, fh.path
                .replace("\"", "\"\""), fh.size, fh.access_type
            )
                .map_err(|e| e.to_string())?;
        }
        Ok(filename)
    }
}
