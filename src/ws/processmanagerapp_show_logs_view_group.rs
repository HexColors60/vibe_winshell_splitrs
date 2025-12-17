//! # ProcessManagerApp - show_logs_view_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub(crate) fn show_logs_view(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .show(
                ui,
                |ui| {
                    ui.heading("📋 Application Log");
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("🗑 Clear Logs").clicked() {
                            self.logs.clear();
                            self.selected_log_indices.clear();
                            self.add_log("Logs cleared".to_string());
                        }
                        if ui.button("📋 Copy Selected").clicked() {
                            if !self.selected_log_indices.is_empty() {
                                let mut selected_logs: Vec<_> = self
                                    .selected_log_indices
                                    .iter()
                                    .filter_map(|&idx| self.logs.get(idx))
                                    .cloned()
                                    .collect();
                                selected_logs.sort();
                                let combined = selected_logs.join("\n");
                                ui.output_mut(|o| o.copied_text = combined);
                            }
                        }
                        if ui.button("📋 Copy All").clicked() {
                            let all_logs = self.logs.join("\n");
                            ui.output_mut(|o| o.copied_text = all_logs);
                        }
                        if ui.button("❌ Clear Selection").clicked() {
                            self.selected_log_indices.clear();
                        }
                        ui.label(
                            format!(
                                "Selected: {} / {}", self.selected_log_indices.len(), self
                                .logs.len()
                            ),
                        );
                    });
                    ui.add_space(5.0);
                    ui.label(
                        "💡 Tip: Click to select, Ctrl+Click to toggle selection",
                    );
                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);
                    for (idx, log) in self.logs.iter().enumerate().rev() {
                        let is_selected = self.selected_log_indices.contains(&idx);
                        ui.horizontal(|ui| {
                            let response = ui
                                .selectable_label(is_selected, format!("[{}]", idx + 1));
                            if response.clicked() {
                                if ui.input(|i| i.modifiers.ctrl || i.modifiers.command) {
                                    if is_selected {
                                        self.selected_log_indices.remove(&idx);
                                    } else {
                                        self.selected_log_indices.insert(idx);
                                    }
                                } else {
                                    self.selected_log_indices.clear();
                                    self.selected_log_indices.insert(idx);
                                }
                            }
                            ui.label(log);
                        });
                    }
                    if self.logs.is_empty() {
                        ui.add_space(20.0);
                        ui.label("No log entries yet");
                    }
                },
            );
    }
}
