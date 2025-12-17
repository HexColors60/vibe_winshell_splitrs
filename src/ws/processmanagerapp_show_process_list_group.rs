use crate::ws::SortColumn;
use crate::ws::ProcessInfo;
use crate::ws::ViewMode;
use crate::ws::CustomProgram;
//! # ProcessManagerApp - show_process_list_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub fn show_process_list(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(
                ui,
                |ui| {
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
                            self.sort_processes();
                        }
                        ui.separator();
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::Name,
                                format!(
                                    "Name {}", if self.sort_column == SortColumn::Name { if self
                                    .sort_ascending { "▲" } else { "▼" } } else { "" }
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
                            self.sort_processes();
                        }
                        ui.separator();
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::Memory,
                                format!(
                                    "Memory {}", if self.sort_column == SortColumn::Memory { if
                                    self.sort_ascending { "▲" } else { "▼" } } else { "" }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::Memory {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::Memory;
                                self.sort_ascending = false;
                            }
                            self.sort_processes();
                        }
                        ui.separator();
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::Cpu,
                                format!(
                                    "CPU % {}", if self.sort_column == SortColumn::Cpu { if self
                                    .sort_ascending { "▲" } else { "▼" } } else { "" }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::Cpu {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::Cpu;
                                self.sort_ascending = false;
                            }
                            self.sort_processes();
                        }
                        ui.separator();
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::ParentPid,
                                format!(
                                    "Parent {}", if self.sort_column == SortColumn::ParentPid {
                                    if self.sort_ascending { "▲" } else { "▼" } } else { ""
                                    }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::ParentPid {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::ParentPid;
                                self.sort_ascending = true;
                            }
                            self.sort_processes();
                        }
                        ui.separator();
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::Status,
                                format!(
                                    "Status {}", if self.sort_column == SortColumn::Status { if
                                    self.sort_ascending { "▲" } else { "▼" } } else { "" }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::Status {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::Status;
                                self.sort_ascending = true;
                            }
                            self.sort_processes();
                        }
                        ui.separator();
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::Runtime,
                                format!(
                                    "Runtime {}", if self.sort_column == SortColumn::Runtime {
                                    if self.sort_ascending { "▲" } else { "▼" } } else { ""
                                    }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::Runtime {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::Runtime;
                                self.sort_ascending = false;
                            }
                            self.sort_processes();
                        }
                    });
                    ui.separator();
                    let filter_lower = self.search_filter.to_lowercase();
                    let mut process_to_kill: Option<u32> = None;
                    let mut program_to_add: Option<(String, String)> = None;
                    let filtered_processes: Vec<&ProcessInfo> = self
                        .processes
                        .iter()
                        .filter(|process| {
                            if filter_lower.is_empty() {
                                true
                            } else {
                                let name_match = process
                                    .name
                                    .to_lowercase()
                                    .contains(&filter_lower);
                                let pid_match = process
                                    .pid
                                    .to_string()
                                    .contains(&filter_lower);
                                name_match || pid_match
                            }
                        })
                        .collect();
                    let start = self.current_page * self.items_per_page;
                    let end = (start + self.items_per_page)
                        .min(filtered_processes.len());
                    let paginated = if start >= filtered_processes.len() {
                        &[]
                    } else {
                        &filtered_processes[start..end]
                    };
                    for process in paginated {
                        let is_selected = self.selected_pid == Some(process.pid);
                        ui.horizontal(|ui| {
                            ui.style_mut().spacing.item_spacing.x = 10.0;
                            if process.is_foreground {
                                ui.colored_label(
                                    egui::Color32::from_rgb(100, 200, 255),
                                    "🔷",
                                );
                            } else {
                                ui.label("  ");
                            }
                            let response = ui
                                .selectable_label(is_selected, format!("{}", process.pid));
                            if response.clicked() {
                                self.selected_pid = Some(process.pid);
                            }
                            ui.separator();
                            ui.label(&process.name);
                            ui.separator();
                            ui.label(Self::format_memory(process.memory));
                            ui.separator();
                            let cpu_color = if process.cpu_usage > 50.0 {
                                egui::Color32::RED
                            } else if process.cpu_usage > 20.0 {
                                egui::Color32::YELLOW
                            } else {
                                egui::Color32::GREEN
                            };
                            ui.colored_label(
                                cpu_color,
                                format!("{:.2}%", process.cpu_usage),
                            );
                            ui.separator();
                            if let Some(parent) = process.parent_pid {
                                ui.label(format!("{}", parent));
                            } else {
                                ui.label("-");
                            }
                            ui.separator();
                            ui.label(&process.status);
                            ui.separator();
                            ui.label(Self::format_time(process.run_time));
                            ui.separator();
                            if ui.button("❌ Kill").clicked() {
                                process_to_kill = Some(process.pid);
                            }
                            if let Some(path) = &process.exe_path {
                                if ui
                                    .button("⭐")
                                    .on_hover_text("Add to Custom Programs")
                                    .clicked()
                                {
                                    program_to_add = Some((process.name.clone(), path.clone()));
                                }
                            }
                        });
                        ui.separator();
                    }
                    if let Some(pid) = process_to_kill {
                        if let Some(window) = self.windows.iter().find(|w| w.pid == pid)
                        {
                            if let Some(image_data) = self
                                .capture_window_thumbnail(window.window_id, 300, 200)
                            {
                                let texture = ctx
                                    .load_texture(
                                        format!("kill_confirm_{}", pid),
                                        image_data,
                                        egui::TextureOptions::default(),
                                    );
                                self.kill_confirm_thumbnail = Some(texture);
                            }
                        }
                        self.kill_confirm_pid = Some(pid);
                    }
                    if let Some((name, path)) = program_to_add {
                        self.add_custom_program(name, path, String::new(), false);
                        self.view_mode = ViewMode::New;
                    }
                },
            );
    }
    #[cfg(not(windows))]
    pub fn capture_window_thumbnail(
        &self,
        _window_id: u64,
        _max_width: i32,
        _max_height: i32,
    ) -> Option<egui::ColorImage> {
        None
    }
    pub fn add_custom_program(
        &mut self,
        name: String,
        path: String,
        args: String,
        admin: bool,
    ) {
        if !self.custom_programs.iter().any(|p| p.path == path && p.args == args) {
            self.custom_programs
                .push(CustomProgram {
                    name,
                    path,
                    args,
                    admin,
                });
            self.save_config();
            self.add_log("Added new custom program".to_string());
        } else {
            self.add_log("Program already exists in custom list".to_string());
        }
    }
}
