use crate::ws::WindowInfo;
use crate::ws::ViewMode;
//! # ProcessManagerApp - show_windows_view_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub(crate) fn show_windows_view(&mut self, ui: &mut egui::Ui) {
        let filter_lower = self.search_filter.to_lowercase();
        let mut window_to_focus: Option<u64> = None;
        let mut program_to_add: Option<(String, String)> = None;
        let pid_to_exe: std::collections::HashMap<u32, String> = self
            .processes
            .iter()
            .filter_map(|p| p.exe_path.as_ref().map(|path| (p.pid, path.clone())))
            .collect();
        let filtered_windows: Vec<WindowInfo> = self
            .windows
            .iter()
            .filter(|window| {
                if filter_lower.is_empty() {
                    true
                } else {
                    let title_match = window
                        .window_title
                        .to_lowercase()
                        .contains(&filter_lower);
                    let process_match = window
                        .process_name
                        .to_lowercase()
                        .contains(&filter_lower);
                    let pid_match = window.pid.to_string().contains(&filter_lower);
                    title_match || process_match || pid_match
                }
            })
            .cloned()
            .collect();
        if self.show_window_grid {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(
                    ui,
                    |ui| {
                        let available_width = ui.available_width();
                        let card_width = 220.0;
                        let columns = (available_width / card_width).floor().max(1.0)
                            as usize;
                        let mut row_index = 0;
                        let mut col_index = 0;
                        ui.horizontal(|ui| {
                            for window in &filtered_windows {
                                let card_color = if window.is_foreground {
                                    egui::Color32::from_rgb(60, 120, 180)
                                } else {
                                    egui::Color32::from_rgb(50, 50, 50)
                                };
                                egui::Frame::none()
                                    .fill(card_color)
                                    .rounding(5.0)
                                    .inner_margin(10.0)
                                    .show(
                                        ui,
                                        |ui| {
                                            ui.set_min_width(200.0);
                                            ui.set_max_width(200.0);
                                            ui.set_min_height(120.0);
                                            ui.vertical(|ui| {
                                                if window.is_foreground {
                                                    ui.colored_label(
                                                        egui::Color32::from_rgb(100, 200, 255),
                                                        "🔷 Foreground",
                                                    );
                                                }
                                                let title = if window.window_title.len() > 30 {
                                                    format!("{}...", & window.window_title[..27])
                                                } else {
                                                    window.window_title.clone()
                                                };
                                                ui.label(format!("📋 {}", title));
                                                let proc_name = if window.process_name.len() > 25 {
                                                    format!("{}...", & window.process_name[..22])
                                                } else {
                                                    window.process_name.clone()
                                                };
                                                ui.label(format!("🖥 {}", proc_name));
                                                ui.label(format!("PID: {}", window.pid));
                                                if ui.button("Focus Window").clicked() {
                                                    window_to_focus = Some(window.window_id);
                                                }
                                                if ui.button("📋 Copy Info").clicked() {
                                                    let info = format!(
                                                        "{} - {} (PID: {})", window.window_title, window
                                                        .process_name, window.pid
                                                    );
                                                    ui.output_mut(|o| o.copied_text = info);
                                                }
                                                if let Some(exe_path) = pid_to_exe.get(&window.pid) {
                                                    if ui.button("⭐ ToNew").clicked() {
                                                        program_to_add = Some((
                                                            window.process_name.clone(),
                                                            exe_path.clone(),
                                                        ));
                                                    }
                                                }
                                            });
                                        },
                                    );
                                col_index += 1;
                                if col_index >= columns {
                                    col_index = 0;
                                    row_index += 1;
                                    ui.end_row();
                                } else {
                                    ui.add_space(10.0);
                                }
                            }
                        });
                        if let Some(window_id) = window_to_focus {
                            self.bring_window_to_foreground(window_id);
                        }
                    },
                );
        } else {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(
                    ui,
                    |ui| {
                        ui.horizontal(|ui| {
                            ui.style_mut().spacing.item_spacing.x = 10.0;
                            ui.label("Status");
                            ui.separator();
                            ui.label("PID");
                            ui.separator();
                            ui.label("Process");
                            ui.separator();
                            ui.label("Window Title");
                            ui.separator();
                            ui.label("Actions");
                        });
                        ui.separator();
                        for window in &filtered_windows {
                            ui.horizontal(|ui| {
                                ui.style_mut().spacing.item_spacing.x = 10.0;
                                if window.is_foreground {
                                    ui.colored_label(
                                        egui::Color32::from_rgb(100, 200, 255),
                                        "🔷",
                                    );
                                } else {
                                    ui.label("  ");
                                }
                                ui.separator();
                                ui.label(format!("{}", window.pid));
                                ui.separator();
                                ui.label(&window.process_name);
                                ui.separator();
                                ui.label(&window.window_title);
                                ui.separator();
                                if ui.button("Focus").clicked() {
                                    window_to_focus = Some(window.window_id);
                                }
                                if ui.button("📋 Copy").clicked() {
                                    let info = format!(
                                        "{} - {} (PID: {})", window.window_title, window
                                        .process_name, window.pid
                                    );
                                    ui.output_mut(|o| o.copied_text = info);
                                }
                                if let Some(exe_path) = pid_to_exe.get(&window.pid) {
                                    if ui
                                        .button("⭐")
                                        .on_hover_text("Add to Custom Programs")
                                        .clicked()
                                    {
                                        program_to_add = Some((
                                            window.process_name.clone(),
                                            exe_path.clone(),
                                        ));
                                    }
                                }
                            });
                            ui.separator();
                        }
                        if let Some(window_id) = window_to_focus {
                            self.bring_window_to_foreground(window_id);
                        }
                    },
                );
        }
        if let Some((name, path)) = program_to_add {
            self.add_custom_program(name, path, String::new(), false);
            self.view_mode = ViewMode::New;
        }
    }
}
