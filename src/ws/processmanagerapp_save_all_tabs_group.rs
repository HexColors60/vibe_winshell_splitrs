use crate::ws::FileInfo;
// # ProcessManagerApp - save_all_tabs_group Methods
//
// This module contains method implementations for `ProcessManagerApp`.
//
// 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub fn save_all_tabs(&mut self) {
        let mut ini_content = String::from("[FilepaneTabs]\n");
        for (i, tab) in self.filepane_tabs.iter().enumerate() {
            ini_content
                .push_str(
                    &format!(
                        "[Tab{}]\n\
                name={}\n\
                left_path={}\n\
                right_path={}\n\
                filter={}\n\
                show_checkboxes={}\n",
                        i, tab.name, tab.left_path.replace('\\', "/"), tab.right_path
                        .replace('\\', "/"), tab.filter, tab.show_checkboxes
                    ),
                );
        }
        ini_content
            .push_str(
                &format!(
                    "\n[Settings]\n\
            active_tab={}\n\
            swap_columns={}\n",
                    self.filepane_active_tab, self.filepane_swap_columns
                ),
            );
        let filename = format!("filepane_all_tabs.ini");
        if let Err(e) = std::fs::write(&filename, ini_content) {
            self.add_log(format!("Failed to save all tabs: {}", e));
        } else {
            self.add_log(
                format!("Saved {} tabs to {}", self.filepane_tabs.len(), filename),
            );
        }
    }
    pub fn load_paths_from_config(&mut self) {
        if !std::path::Path::new(&self.filepane_config_path).exists() {
            self.add_log("No config file found".to_string());
            return;
        }
        if let Ok(content) = std::fs::read_to_string(&self.filepane_config_path) {
            let mut left_path = String::new();
            let mut right_path = String::new();
            let mut filter = String::new();
            let mut show_checkboxes = false;
            for line in content.lines() {
                if line.starts_with("left_path=") {
                    left_path = line.split('=').nth(1).unwrap_or("").replace('/', "\\");
                } else if line.starts_with("right_path=") {
                    right_path = line.split('=').nth(1).unwrap_or("").replace('/', "\\");
                } else if line.starts_with("filter=") {
                    filter = line.split('=').nth(1).unwrap_or("").to_string();
                } else if line.starts_with("show_checkboxes=") {
                    show_checkboxes = line.split('=').nth(1).unwrap_or("false")
                        == "true";
                }
            }
            if self.filepane_active_tab < self.filepane_tabs.len() {
                let tab = &mut self.filepane_tabs[self.filepane_active_tab];
                tab.left_path = left_path;
                tab.right_path = right_path;
                tab.filter = filter;
                tab.show_checkboxes = show_checkboxes;
                self.add_log(format!("Loaded paths from {}", self.filepane_config_path));
            }
        }
    }
    pub fn show_filepane_confirmation_dialog(&mut self, ui: &mut egui::Ui) {
        let should_show = self.filepane_show_confirm;
        let confirm_message = self.filepane_confirm_message.clone();
        let confirm_action = self.filepane_confirm_action.clone();
        if should_show && confirm_action.is_some() {
            egui::Window::new("⚠️ Confirm Action")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(
                    ui.ctx(),
                    |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(10.0);
                            ui.label(&confirm_message);
                            ui.add_space(20.0);
                            ui.horizontal(|ui| {
                                if ui.button("✅ Yes").clicked() {
                                    if let Some(action) = confirm_action {
                                        self.add_log("Action confirmed".to_string());
                                        self.execute_command(&action);
                                    }
                                    self.filepane_show_confirm = false;
                                    self.filepane_confirm_action = None;
                                    self.filepane_confirm_message.clear();
                                }
                                if ui.button("❌ No").clicked() {
                                    self.add_log("Action cancelled".to_string());
                                    self.filepane_show_confirm = false;
                                    self.filepane_confirm_action = None;
                                    self.filepane_confirm_message.clear();
                                }
                            });
                            ui.add_space(10.0);
                        });
                    },
                );
        }
    }
    pub fn show_file_properties(&mut self, file_info: &FileInfo) {
        let file_type = if file_info.is_directory { "Directory" } else { "File" };
        let modified_time = file_info
            .modified
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let formatted_time = std::time::UNIX_EPOCH
            + std::time::Duration::from_secs(modified_time);
        self.add_log(format!("=== Properties for {} ===", file_info.name));
        self.add_log(format!("Type: {}", file_type));
        self.add_log(format!("Size: {} bytes", file_info.size));
        self.add_log(format!("Modified: {:?}", formatted_time));
        self.add_log(format!("Path: {}", file_info.path));
        if let Some(ref ext) = file_info.extension {
            self.add_log(format!("Extension: {}", ext));
        }
        self.add_log("=================================".to_string());
    }
    pub fn open_file_with_system(&self, path: &str) {
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("cmd")
                .args(&["/C", "start", "", path])
                .spawn();
        }
        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("xdg-open").arg(path).spawn();
        }
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("open").arg(path).spawn();
        }
    }
    pub fn save_current_conversation(&mut self) {
        let mut conversation_text = String::new();
        if !self.conversation_history.is_empty() {
            conversation_text.push_str("=== Previous Conversation ===\n");
            for entry in &self.conversation_history {
                conversation_text.push_str(&format!("{}\n", entry));
            }
            conversation_text.push_str("\n");
        }
        conversation_text.push_str("=== Current Session Logs ===\n");
        for (i, log_entry) in self.logs.iter().enumerate() {
            conversation_text.push_str(&format!("[{:03}] {}\n", i + 1, log_entry));
        }
        match self.save_conversation_history(&conversation_text) {
            Ok(message) => {
                self.add_log(format!("✅ {}", message));
            }
            Err(error) => {
                self.add_log(format!("❌ {}", error));
            }
        }
    }
}
