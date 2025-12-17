use crate::ws::SortColumn;
use crate::ws::ViewMode;
// # ProcessManagerApp - show_file_list_group Methods
//
// This module contains method implementations for `ProcessManagerApp`.
//
// 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub(crate) fn show_file_list(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(
                ui,
                |ui| {
                    let pid_to_exe: std::collections::HashMap<u32, String> = self
                        .processes
                        .iter()
                        .filter_map(|p| {
                            p.exe_path.as_ref().map(|path| (p.pid, path.clone()))
                        })
                        .collect();
                    ui.horizontal(|ui| {
                        ui.style_mut().spacing.item_spacing.x = 10.0;
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::Pid,
                                format!(
                                    "PID {}", if self.sort_column == SortColumn::Pid { if self
                                    .sort_ascending { "▲" } else { "▼" } } else { "" }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::Pid {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::Pid;
                                self.sort_ascending = true;
                            }
                            self.sort_files();
                        }
                        ui.separator();
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::Name,
                                format!(
                                    "Process {}", if self.sort_column == SortColumn::Name { if
                                    self.sort_ascending { "▲" } else { "▼" } } else { "" }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::Name {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::Name;
                                self.sort_ascending = true;
                            }
                            self.sort_files();
                        }
                        ui.separator();
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::FilePath,
                                format!(
                                    "File Path {}", if self.sort_column == SortColumn::FilePath
                                    { if self.sort_ascending { "▲" } else { "▼" } } else {
                                    "" }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::FilePath {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::FilePath;
                                self.sort_ascending = true;
                            }
                            self.sort_files();
                        }
                        ui.separator();
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::FileSize,
                                format!(
                                    "Size {}", if self.sort_column == SortColumn::FileSize { if
                                    self.sort_ascending { "▲" } else { "▼" } } else { "" }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::FileSize {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::FileSize;
                                self.sort_ascending = false;
                            }
                            self.sort_files();
                        }
                        ui.separator();
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::FileAccess,
                                format!(
                                    "Access {}", if self.sort_column == SortColumn::FileAccess {
                                    if self.sort_ascending { "▲" } else { "▼" } } else { ""
                                    }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::FileAccess {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::FileAccess;
                                self.sort_ascending = true;
                            }
                            self.sort_files();
                        }
                        ui.separator();
                        ui.label("Actions");
                    });
                    ui.separator();
                    let filter_lower = self.search_filter.to_lowercase();
                    let mut path_to_open: Option<String> = None;
                    let mut program_to_add: Option<(String, String)> = None;
                    for file in &self.file_handles {
                        if !filter_lower.is_empty() {
                            let path_match = file
                                .path
                                .to_lowercase()
                                .contains(&filter_lower);
                            let process_match = file
                                .process_name
                                .to_lowercase()
                                .contains(&filter_lower);
                            let pid_match = file.pid.to_string().contains(&filter_lower);
                            if !path_match && !process_match && !pid_match {
                                continue;
                            }
                        }
                        ui.horizontal(|ui| {
                            ui.style_mut().spacing.item_spacing.x = 10.0;
                            ui.label(format!("{}", file.pid));
                            ui.separator();
                            ui.label(&file.process_name);
                            ui.separator();
                            ui.label(&file.path);
                            ui.separator();
                            ui.label(Self::format_memory(file.size));
                            ui.separator();
                            ui.label(&file.access_type);
                            ui.separator();
                            if ui.button("📋 Copy").clicked() {
                                ui.ctx().copy_text(file.path.clone());}
                            if ui.button("📂 Open").clicked() {
                                path_to_open = Some(file.path.clone());
                            }
                            if let Some(exe_path) = pid_to_exe.get(&file.pid) {
                                if ui
                                    .button("⭐")
                                    .on_hover_text("Add Process to Custom Programs")
                                    .clicked()
                                {
                                    program_to_add = Some((
                                        file.process_name.clone(),
                                        exe_path.clone(),
                                    ));
                                }
                            }
                        });
                        ui.separator();
                    }
                    if let Some(path) = path_to_open {
                        self.open_file_path(&path);
                    }
                    if let Some((name, path)) = program_to_add {
                        self.add_custom_program(name, path, String::new(), false);
                        self.view_mode = ViewMode::New;
                    }
                },
            );
    }
    pub fn open_file_path(&mut self, path: &str) {
        #[cfg(windows)]
        {
            let _ = std::process::Command::new("explorer")
                .args(&["/select,", path])
                .spawn();
            self.add_log(format!("Opening path in Explorer: {}", path));
        }
        #[cfg(target_os = "linux")]
        {
            if let Some(parent) = std::path::Path::new(path).parent() {
                let _ = std::process::Command::new("xdg-open").arg(parent).spawn();
                self.add_log(format!("Opening parent directory: {}", parent.display()));
            }
        }
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("open").args(&["-R", path]).spawn();
            self.add_log(format!("Opening path in Finder: {}", path));
        }
    }
}
