//! # ProcessManagerApp - show_kill_confirm_dialog_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub(crate) fn show_kill_confirm_dialog(&mut self, ctx: &egui::Context) {
        if let Some(pid) = self.kill_confirm_pid {
            let mut open = true;
            egui::Window::new("⚠️ Confirm Kill Process")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .open(&mut open)
                .show(
                    ctx,
                    |ui| {
                        ui.set_min_width(300.0);
                        ui.vertical_centered(|ui| {
                            ui.heading(
                                format!("Are you sure you want to kill process {}?", pid),
                            );
                            ui.add_space(10.0);
                            if let Some(texture) = &self.kill_confirm_thumbnail {
                                ui.add(
                                    egui::Image::new(texture)
                                        .fit_to_exact_size(egui::vec2(240.0, 160.0)),
                                );
                                ui.add_space(10.0);
                            } else {
                                if let Some(process) = self
                                    .processes
                                    .iter()
                                    .find(|p| p.pid == pid)
                                {
                                    ui.label(
                                        egui::RichText::new(&process.name).size(16.0).strong(),
                                    );
                                }
                                ui.label("(No window thumbnail available)");
                                ui.add_space(10.0);
                            }
                            ui.label(
                                "This action cannot be undone and may cause data loss.",
                            );
                            ui.add_space(20.0);
                            ui.horizontal(|ui| {
                                if ui.button("❌ Cancel").clicked() {
                                    self.kill_confirm_pid = None;
                                    self.kill_confirm_thumbnail = None;
                                }
                                if ui.button("⚠️ Yes, Kill Process").clicked() {
                                    if self.kill_process(pid) {
                                        self.add_log(
                                            format!("Successfully killed process {}", pid),
                                        );
                                        self.refresh_processes();
                                    } else {
                                        self.add_log(format!("Failed to kill process {}", pid));
                                    }
                                    self.kill_confirm_pid = None;
                                    self.kill_confirm_thumbnail = None;
                                }
                            });
                        });
                    },
                );
            if !open {
                self.kill_confirm_pid = None;
                self.kill_confirm_thumbnail = None;
            }
        }
    }
}
