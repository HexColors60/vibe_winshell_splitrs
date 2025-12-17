use std::fs::File;
//! # ProcessManagerApp - export_network_to_csv_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    fn export_network_to_csv(&self) -> Result<String, String> {
        let filename = format!(
            "network_{}.csv", chrono::Local::now().format("%Y%m%d_%H%M%S")
        );
        let mut file = File::create(&filename).map_err(|e| e.to_string())?;
        writeln!(
            file,
            "PID,Process Name,Protocol,Local Address,Remote Address,State,Connection ID"
        )
            .map_err(|e| e.to_string())?;
        for conn in &self.network_connections {
            writeln!(
                file, "{},{},{},{},{},{},{}", conn.pid, conn.process_name, conn.protocol,
                conn.local_addr, conn.remote_addr, conn.state, conn.connection_id
            )
                .map_err(|e| e.to_string())?;
        }
        Ok(filename)
    }
}
