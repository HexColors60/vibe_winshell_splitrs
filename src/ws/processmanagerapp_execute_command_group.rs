use crate::ws::FilepaneCommand;
//! # ProcessManagerApp - execute_command_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub fn execute_command(&mut self, command: &FilepaneCommand) {
        match command {
            FilepaneCommand::CopyFile { source, destination } => {
                self.add_log(format!("Copying {} to {}", source, destination));
            }
            FilepaneCommand::MoveFile { source, destination } => {
                self.add_log(format!("Moving {} to {}", source, destination));
            }
            FilepaneCommand::DeleteFile { path } => {
                self.add_log(format!("Deleting {}", path));
            }
            FilepaneCommand::CreateDirectory { path } => {
                self.add_log(format!("Creating directory {}", path));
            }
            FilepaneCommand::RenameFile { old_path, new_path } => {
                self.add_log(format!("Renaming {} to {}", old_path, new_path));
            }
            FilepaneCommand::ChangeDirectory { panel, new_path } => {
                self.add_log(format!("Changing panel {} to {}", panel, new_path));
            }
            FilepaneCommand::CalculateChecksum { path, algorithm } => {
                self.add_log(
                    format!("Calculating {} checksum for {}", algorithm.name(), path),
                );
            }
        }
        if self.filepane_active_tab < self.filepane_tabs.len() {
            let tab = &mut self.filepane_tabs[self.filepane_active_tab];
            tab.command_history.push(command.clone());
            tab.undo_stack.clear();
        }
    }
}
