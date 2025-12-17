use crate::ws::FilepaneCommand;
//! # ProcessManagerApp - redo_last_action_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub fn redo_last_action(&mut self) {
        if self.filepane_active_tab >= self.filepane_tabs.len() {
            return;
        }
        let command = {
            let tab = &mut self.filepane_tabs[self.filepane_active_tab];
            tab.redo_stack.pop()
        };
        if let Some(command) = command {
            let log_message = match &command {
                FilepaneCommand::CopyFile { source, destination } => {
                    format!("Redo: Copy {} -> {}", source, destination)
                }
                FilepaneCommand::MoveFile { source, destination } => {
                    format!("Redo: Move {} -> {}", source, destination)
                }
                FilepaneCommand::DeleteFile { path } => {
                    format!("Redo: Delete {} (cannot restore)", path)
                }
                FilepaneCommand::CreateDirectory { path } => {
                    format!("Redo: Create {}", path)
                }
                FilepaneCommand::RenameFile { old_path, new_path } => {
                    format!("Redo: Rename {} -> {}", old_path, new_path)
                }
                FilepaneCommand::ChangeDirectory { panel, new_path: _ } => {
                    format!("Redo: Change directory for panel {}", panel)
                }
                FilepaneCommand::CalculateChecksum { path, algorithm } => {
                    format!("Redo: {} checksum for {}", algorithm.name(), path)
                }
            };
            self.add_log(log_message);
            let tab = &mut self.filepane_tabs[self.filepane_active_tab];
            tab.undo_stack.push(command);
        }
    }
    pub fn save_current_paths(&mut self) {
        if self.filepane_active_tab >= self.filepane_tabs.len() {
            return;
        }
        let tab = &self.filepane_tabs[self.filepane_active_tab];
        let ini_content = format!(
            "[Filepane]\n\
            left_path={}\n\
            right_path={}\n\
            filter={}\n\
            show_checkboxes={}\n",
            tab.left_path.replace('\\', "/"), tab.right_path.replace('\\', "/"), tab
            .filter, tab.show_checkboxes
        );
        if let Err(e) = std::fs::write(&self.filepane_config_path, ini_content) {
            self.add_log(format!("Failed to save paths: {}", e));
        } else {
            self.add_log(
                format!("Saved current paths to {}", self.filepane_config_path),
            );
        }
    }
}
