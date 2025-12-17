use crate::ws::FileInfo;
// # ProcessManagerApp - save_all_tabs_group Methods
//
// This module contains method implementations for `ProcessManagerApp`.
//
// 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;
use crate::ws::FilepaneCommand;
use crate::ws::TrashItem;

// use windows::Win32::Shell::SHFILEOPSTRUCTW;
// use windows::Win32::Shell::FO_DELETE;
// use windows::Win32::Shell::FOF_ALLOWUNDO;
// use windows::Win32::Shell::SHFileOperationW;

use windows::Win32::UI::Shell::{
    SHFILEOPSTRUCTW, SHFileOperationW, FO_DELETE, FOF_ALLOWUNDO, FOF_NOCONFIRMATION,
};


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
        let is_second_confirm = self.filepane_second_confirm;

        if should_show && confirm_action.is_some() {
            let window_title = if is_second_confirm {
                "⚠️ FINAL CONFIRMATION - This action cannot be undone!"
            } else {
                "⚠️ Confirm File Operation"
            };

            egui::Window::new(window_title)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(
                    ui.ctx(),
                    |ui| {
                        ui.set_min_width(400.0);
                        ui.vertical_centered(|ui| {
                            ui.add_space(10.0);

                            if is_second_confirm {
                                ui.colored_label(egui::Color32::RED, "⚠️ WARNING: This is your final confirmation!");
                                ui.add_space(5.0);
                                ui.colored_label(egui::Color32::RED, "This action cannot be undone!");
                                ui.add_space(10.0);
                            }

                            ui.label(&confirm_message);

                            if is_second_confirm {
                                ui.add_space(10.0);
                                ui.label("Are you absolutely sure you want to proceed?");
                                ui.label("Type 'CONFIRM' to proceed:");
                                let mut confirm_input = String::new();
                                ui.add_sized([200.0, 25.0], egui::TextEdit::singleline(&mut confirm_input));

                                ui.add_space(15.0);
                                ui.horizontal(|ui| {
                                    if ui.button("🚫 Cancel").clicked() {
                                        self.add_log("Operation cancelled - user rejected final confirmation".to_string());
                                        self.filepane_show_confirm = false;
                                        self.filepane_second_confirm = false;
                                        self.filepane_confirm_action = None;
                                        self.filepane_confirm_message.clear();
                                        self.filepane_pending_operation = None;
                                    }

                                    let confirm_button = ui.add_enabled(
                                        confirm_input == "CONFIRM",
                                        egui::Button::new("⚠️ YES, EXECUTE OPERATION")
                                    );

                                    if confirm_button.clicked() && confirm_input == "CONFIRM" {
                                        if let Some(action) = confirm_action {
                                            self.add_log("🔥 Final confirmation received - executing operation".to_string());
                                            self.execute_real_command(&action);
                                        }
                                        self.filepane_show_confirm = false;
                                        self.filepane_second_confirm = false;
                                        self.filepane_confirm_action = None;
                                        self.filepane_confirm_message.clear();
                                        self.filepane_pending_operation = None;
                                    }
                                });
                            } else {
                                ui.add_space(20.0);
                                ui.horizontal(|ui| {
                                    if ui.button("❌ Cancel").clicked() {
                                        self.add_log("Action cancelled by user".to_string());
                                        self.filepane_show_confirm = false;
                                        self.filepane_confirm_action = None;
                                        self.filepane_confirm_message.clear();
                                        self.filepane_pending_operation = None;
                                    }
                                    if ui.button("✅ Confirm").clicked() {
                                        self.add_log("First confirmation received - requiring final confirmation".to_string());
                                        self.filepane_second_confirm = true;
                                    }
                                });
                            }
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
    pub fn execute_real_command(&mut self, command: &FilepaneCommand) {
        match command {
            FilepaneCommand::CopyFile { source, destination } => {
                self.real_copy_file(source, destination);
            }
            FilepaneCommand::MoveFile { source, destination } => {
                self.real_move_file(source, destination);
            }
            FilepaneCommand::DeleteFile { path } => {
                self.real_delete_file(path);
            }
            FilepaneCommand::CreateDirectory { path } => {
                self.real_create_directory(path);
            }
            FilepaneCommand::RenameFile { old_path, new_path } => {
                self.real_rename_file(old_path, new_path);
            }
            _ => {
                self.execute_command(command);
            }
        }

        // Add to operation history
        if let Some(ref operation) = self.filepane_pending_operation {
            self.filepane_operation_history.push(operation.clone());
        }
        self.filepane_pending_operation = None;
    }

    fn real_copy_file(&mut self, source: &str, destination: &str) {
        let source_path = std::path::Path::new(source);
        let dest_path = std::path::Path::new(destination).join(
            source_path.file_name().unwrap_or_default()
        );

        self.add_log(format!("📋 COPY: Starting copy operation"));
        self.add_log(format!("   Source: {}", source));
        self.add_log(format!("   Destination: {}", dest_path.display()));

        match if source_path.is_dir() {
            self.copy_directory(source_path, &dest_path)
        } else {
            self.copy_file(source_path, &dest_path)
        } {
            Ok(_) => {
                self.add_log(format!("✅ Successfully copied to {}", dest_path.display()));
            }
            Err(e) => {
                self.add_log(format!("❌ Copy failed: {}", e));
            }
        }
    }

    fn copy_file(&self, source: &std::path::Path, destination: &std::path::Path) -> std::io::Result<()> {
        std::fs::copy(source, destination)?;
        Ok(())
    }

    fn copy_directory(&self, source: &std::path::Path, destination: &std::path::Path) -> std::io::Result<()> {
        std::fs::create_dir_all(destination)?;
        for entry in std::fs::read_dir(source)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let source_path = entry.path();
            let dest_path = destination.join(entry.file_name());

            if file_type.is_dir() {
                self.copy_directory(&source_path, &dest_path)?;
            } else {
                std::fs::copy(&source_path, &dest_path)?;
            }
        }
        Ok(())
    }

    fn real_move_file(&mut self, source: &str, destination: &str) {
        let source_path = std::path::Path::new(source);
        let dest_path = std::path::Path::new(destination).join(
            source_path.file_name().unwrap_or_default()
        );

        self.add_log(format!("✂️ MOVE: Starting move operation"));
        self.add_log(format!("   Source: {}", source));
        self.add_log(format!("   Destination: {}", dest_path.display()));

        match std::fs::rename(source, &dest_path) {
            Ok(_) => {
                self.add_log(format!("✅ Successfully moved to {}", dest_path.display()));
            }
            Err(e) => {
                self.add_log(format!("❌ Move failed: {}", e));
            }
        }
    }

    fn real_delete_file(&mut self, path: &str) {
        self.add_log(format!("🗑️ DELETE: Starting delete operation"));
        self.add_log(format!("   Path: {}", path));

        // Move to trash instead of permanent delete
        match self.move_to_trash(path) {
            Ok(trash_path) => {
                let trash_item = TrashItem {
                    original_path: path.to_string(),
                    trash_path: trash_path.clone(),
                    deletion_time: std::time::SystemTime::now(),
                    file_type: crate::ws::FileOperationType::Delete,
                };
                self.filepane_trash_items.push(trash_item);
                self.add_log(format!("✅ Moved to trash: {}", trash_path));
            }
            Err(e) => {
                self.add_log(format!("❌ Delete failed: {}", e));
            }
        }
    }

    fn move_to_trash(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let source_path = std::path::Path::new(path);

        #[cfg(target_os = "windows")]
        {
            // Use Windows API to move to recycle bin
            use std::os::windows::ffi::OsStrExt;
            use std::ffi::OsString;
            use windows::Win32::Foundation::HWND;
            use windows::core::PCWSTR;
            use windows::Win32::Foundation::BOOL;

            let mut wide_path: Vec<u16> = OsString::from(path).encode_wide().collect();
            wide_path.push(0); // Null terminate with double null termination
            wide_path.push(0);

            // Create the SHFILEOPSTRUCT
            let mut file_op = windows::Win32::UI::Shell::SHFILEOPSTRUCTW {
                hwnd: HWND::default(),
                wFunc: windows::Win32::UI::Shell::FO_DELETE,
                pFrom: PCWSTR::from_raw(wide_path.as_ptr()),
                pTo: PCWSTR::null(),
                fFlags: (windows::Win32::UI::Shell::FOF_ALLOWUNDO | windows::Win32::UI::Shell::FOF_NOCONFIRMATION).0,
                fAnyOperationsAborted: BOOL::from(false),
                hNameMappings: std::ptr::null_mut(),
                lpszProgressTitle: PCWSTR::null(),
            };

            // Call SHFileOperationW
            let result = unsafe { windows::Win32::UI::Shell::SHFileOperationW(&mut file_op) };

            if result != 0 {
                return Err(format!("Failed to move to recycle bin: {}", result).into());
            }

            Ok("Windows Recycle Bin".to_string())
        }

        #[cfg(not(target_os = "windows"))]
        {
            // For non-Windows systems, create a trash directory
            let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            let trash_dir = format!("{}/.trash", home_dir);
            std::fs::create_dir_all(&trash_dir)?;

            let file_name = source_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            let trash_name = format!("{}_{}", timestamp, file_name);
            let trash_path = std::path::Path::new(&trash_dir).join(trash_name);

            std::fs::rename(path, &trash_path)?;
            Ok(trash_path.to_string_lossy().to_string())
        }
    }

    fn real_create_directory(&mut self, path: &str) {
        self.add_log(format!("📁 CREATE: Creating directory"));
        self.add_log(format!("   Path: {}", path));

        match std::fs::create_dir_all(path) {
            Ok(_) => {
                self.add_log(format!("✅ Successfully created directory: {}", path));
            }
            Err(e) => {
                self.add_log(format!("❌ Create directory failed: {}", e));
            }
        }
    }

    fn real_rename_file(&mut self, old_path: &str, new_path: &str) {
        self.add_log(format!("🏷️ RENAME: Starting rename operation"));
        self.add_log(format!("   From: {}", old_path));
        self.add_log(format!("   To: {}", new_path));

        match std::fs::rename(old_path, new_path) {
            Ok(_) => {
                self.add_log(format!("✅ Successfully renamed to {}", new_path));
            }
            Err(e) => {
                self.add_log(format!("❌ Rename failed: {}", e));
            }
        }
    }

    pub fn restore_from_trash(&mut self) -> bool {
        if let Some(trash_item) = self.filepane_trash_items.pop() {
            self.add_log(format!("♻️ RESTORE: Restoring from trash"));
            self.add_log(format!("   Original path: {}", trash_item.original_path));
            self.add_log(format!("   Trash path: {}", trash_item.trash_path));

            match std::fs::rename(&trash_item.trash_path, &trash_item.original_path) {
                Ok(_) => {
                    self.add_log(format!("✅ Successfully restored to {}", trash_item.original_path));
                    true
                }
                Err(e) => {
                    self.add_log(format!("❌ Restore failed: {}", e));
                    // Put it back in trash if restore failed
                    self.filepane_trash_items.push(trash_item);
                    false
                }
            }
        } else {
            self.add_log("❌ No items in trash to restore".to_string());
            false
        }
    }

    pub fn open_file_with_system(&mut self, path: &str) {
        self.add_log(format!("🔧 OPEN: Opening file with system default"));
        self.add_log(format!("   Path: {}", path));

        #[cfg(target_os = "windows")]
        {
            match std::process::Command::new("cmd")
                .args(&["/C", "start", "", path])
                .spawn() {
                Ok(_) => {
                    self.add_log("✅ File opened successfully".to_string());
                }
                Err(e) => {
                    self.add_log(format!("❌ Failed to open file: {}", e));
                }
            }
        }
        #[cfg(target_os = "linux")]
        {
            match std::process::Command::new("xdg-open").arg(path).spawn() {
                Ok(_) => {
                    self.add_log("✅ File opened successfully".to_string());
                }
                Err(e) => {
                    self.add_log(format!("❌ Failed to open file: {}", e));
                }
            }
        }
        #[cfg(target_os = "macos")]
        {
            match std::process::Command::new("open").arg(path).spawn() {
                Ok(_) => {
                    self.add_log("✅ File opened successfully".to_string());
                }
                Err(e) => {
                    self.add_log(format!("❌ Failed to open file: {}", e));
                }
            }
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
