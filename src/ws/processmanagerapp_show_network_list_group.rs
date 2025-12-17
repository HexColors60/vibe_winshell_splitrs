use crate::ws::ViewMode;
//! # ProcessManagerApp - show_network_list_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub(crate) fn show_network_list(&mut self, ui: &mut egui::Ui) {
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
                        ui.label("PID");
                        ui.separator();
                        ui.label("Process");
                        ui.separator();
                        ui.label("Protocol");
                        ui.separator();
                        ui.label("Local Address");
                        ui.separator();
                        ui.label("Remote Address");
                        ui.separator();
                        ui.label("State");
                        ui.separator();
                        ui.label("Actions");
                    });
                    ui.separator();
                    let filter_lower = self.search_filter.to_lowercase();
                    let mut conn_to_close: Option<String> = None;
                    let mut program_to_add: Option<(String, String)> = None;
                    for conn in &self.network_connections {
                        if !filter_lower.is_empty() {
                            let process_match = conn
                                .process_name
                                .to_lowercase()
                                .contains(&filter_lower);
                            let pid_match = conn.pid.to_string().contains(&filter_lower);
                            let local_match = conn
                                .local_addr
                                .to_lowercase()
                                .contains(&filter_lower);
                            let remote_match = conn
                                .remote_addr
                                .to_lowercase()
                                .contains(&filter_lower);
                            if !process_match && !pid_match && !local_match
                                && !remote_match
                            {
                                continue;
                            }
                        }
                        ui.horizontal(|ui| {
                            ui.style_mut().spacing.item_spacing.x = 10.0;
                            ui.label(format!("{}", conn.pid));
                            ui.separator();
                            ui.label(&conn.process_name);
                            ui.separator();
                            ui.label(&conn.protocol);
                            ui.separator();
                            ui.label(&conn.local_addr);
                            ui.separator();
                            ui.label(&conn.remote_addr);
                            ui.separator();
                            let state_color = if conn.state == "Listen"
                                || conn.state.contains("LISTEN")
                            {
                                egui::Color32::BLUE
                            } else if conn.state == "Established"
                                || conn.state.contains("ESTABLISHED")
                            {
                                egui::Color32::GREEN
                            } else {
                                egui::Color32::GRAY
                            };
                            ui.colored_label(state_color, &conn.state);
                            ui.separator();
                            if ui.button("📋 Copy").clicked() {
                                let info = format!(
                                    "{} {} {} -> {} [{}]", conn.process_name, conn.protocol,
                                    conn.local_addr, conn.remote_addr, conn.state
                                );
                                ui.output_mut(|o| o.copied_text = info);
                            }
                            if ui.button("🔌 Close").clicked() {
                                conn_to_close = Some(conn.connection_id.clone());
                            }
                            if let Some(exe_path) = pid_to_exe.get(&conn.pid) {
                                if ui
                                    .button("⭐")
                                    .on_hover_text("Add Process to Custom Programs")
                                    .clicked()
                                {
                                    program_to_add = Some((
                                        conn.process_name.clone(),
                                        exe_path.clone(),
                                    ));
                                }
                            }
                        });
                        ui.separator();
                    }
                    if let Some(conn_id) = conn_to_close {
                        self.close_network_connection(&conn_id);
                    }
                    if let Some((name, path)) = program_to_add {
                        self.add_custom_program(name, path, String::new(), false);
                        self.view_mode = ViewMode::New;
                    }
                },
            );
    }
}
