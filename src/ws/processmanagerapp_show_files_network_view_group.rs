use crate::ws::FileHandle;
use crate::ws::NetworkConnection;
//! # ProcessManagerApp - show_files_network_view_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub(crate) fn show_files_network_view(&mut self, ui: &mut egui::Ui) {
        let filter_lower = self.search_filter.to_lowercase();
        let filtered_processes: Vec<(u32, String, u64, f32, Option<String>)> = self
            .processes
            .iter()
            .filter(|p| {
                if filter_lower.is_empty() {
                    return true;
                }
                p.name.to_lowercase().contains(&filter_lower)
                    || p.pid.to_string().contains(&filter_lower)
            })
            .map(|p| (p.pid, p.name.clone(), p.memory, p.cpu_usage, p.exe_path.clone()))
            .collect();
        let items_per_page = 20;
        let total_items = filtered_processes.len();
        let total_pages = (total_items + items_per_page - 1) / items_per_page;
        if self.current_page >= total_pages && total_pages > 0 {
            self.current_page = total_pages - 1;
        }
        let start_idx = self.current_page * items_per_page;
        let end_idx = (start_idx + items_per_page).min(total_items);
        let page_items = if start_idx < total_items {
            &filtered_processes[start_idx..end_idx]
        } else {
            &[]
        };
        ui.horizontal(|ui| {
            ui.heading("📂/🌐 Files & Network Manager");
            ui.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    if total_pages > 1 {
                        if ui.button("Next ▶").clicked()
                            && self.current_page + 1 < total_pages
                        {
                            self.current_page += 1;
                        }
                        ui.label(
                            format!("Page {}/{}", self.current_page + 1, total_pages),
                        );
                        if ui.button("◀ Prev").clicked() && self.current_page > 0 {
                            self.current_page -= 1;
                        }
                    }
                    ui.label(
                        format!(
                            "Showing {}-{} of {}", start_idx + 1, end_idx, total_items
                        ),
                    );
                },
            );
        });
        ui.separator();
        let mut path_to_open: Option<String> = None;
        let mut conn_to_close: Option<String> = None;
        let mut pid_to_kill: Option<u32> = None;
        let mut program_to_add: Option<(String, String)> = None;
        let mut process_to_copy: Option<u32> = None;
        egui::ScrollArea::vertical()
            .show(
                ui,
                |ui| {
                    for (pid, name, memory, cpu_usage, exe_path) in page_items {
                        ui.push_id(
                            *pid,
                            |ui| {
                                egui::Frame::group(ui.style())
                                    .inner_margin(8.0)
                                    .show(
                                        ui,
                                        |ui| {
                                            ui.horizontal(|ui| {
                                                ui.colored_label(
                                                    egui::Color32::from_rgb(100, 200, 255),
                                                    format!("{} ({})", name, pid),
                                                );
                                                ui.with_layout(
                                                    egui::Layout::right_to_left(egui::Align::Center),
                                                    |ui| {
                                                        if ui
                                                            .button("💀 Kill")
                                                            .on_hover_text("Kill Process")
                                                            .clicked()
                                                        {
                                                            pid_to_kill = Some(*pid);
                                                        }
                                                        if let Some(path) = exe_path {
                                                            if ui
                                                                .button("⭐ ToNew")
                                                                .on_hover_text("Add to Custom Programs")
                                                                .clicked()
                                                            {
                                                                program_to_add = Some((name.clone(), path.clone()));
                                                            }
                                                        }
                                                        if ui
                                                            .button("📋 Copy")
                                                            .on_hover_text("Copy All Process Data")
                                                            .clicked()
                                                        {
                                                            process_to_copy = Some(*pid);
                                                        }
                                                        ui.label(format!("Mem: {}", Self::format_memory(* memory)));
                                                        ui.label(format!("CPU: {:.1}%", cpu_usage));
                                                    },
                                                );
                                            });
                                            ui.add_space(4.0);
                                            let files: Vec<&FileHandle> = self
                                                .file_handles
                                                .iter()
                                                .filter(|f| f.pid == *pid)
                                                .collect();
                                            let conns: Vec<&NetworkConnection> = self
                                                .network_connections
                                                .iter()
                                                .filter(|c| c.pid == *pid)
                                                .collect();
                                            if !files.is_empty() {
                                                ui.label(
                                                    egui::RichText::new(
                                                            format!("📂 Open Files ({})", files.len()),
                                                        )
                                                        .strong()
                                                        .size(12.0),
                                                );
                                                for file in &files {
                                                    ui.horizontal(|ui| {
                                                        ui.label("  •");
                                                        ui.label(&file.path);
                                                        ui.weak(format!("({})", file.access_type));
                                                        if ui.button("📋").on_hover_text("Copy Path").clicked() {
                                                            ui.output_mut(|o| o.copied_text = file.path.clone());
                                                        }
                                                        if ui
                                                            .button("📂")
                                                            .on_hover_text("Open Location")
                                                            .clicked()
                                                        {
                                                            path_to_open = Some(file.path.clone());
                                                        }
                                                    });
                                                }
                                            }
                                            if !conns.is_empty() {
                                                if !files.is_empty() {
                                                    ui.add_space(4.0);
                                                }
                                                ui.label(
                                                    egui::RichText::new(
                                                            format!("🌐 Network ({})", conns.len()),
                                                        )
                                                        .strong()
                                                        .size(12.0),
                                                );
                                                for conn in &conns {
                                                    ui.horizontal(|ui| {
                                                        ui.label("  •");
                                                        ui.label(
                                                            format!(
                                                                "{} {} -> {}", conn.protocol, conn.local_addr, conn
                                                                .remote_addr
                                                            ),
                                                        );
                                                        ui.colored_label(
                                                            if conn.state.contains("ESTABLISHED") {
                                                                egui::Color32::GREEN
                                                            } else {
                                                                egui::Color32::GRAY
                                                            },
                                                            &conn.state,
                                                        );
                                                        if ui.button("📋").on_hover_text("Copy Info").clicked() {
                                                            let info = format!(
                                                                "{} {} {} -> {} [{}]", conn.process_name, conn.protocol,
                                                                conn.local_addr, conn.remote_addr, conn.state
                                                            );
                                                            ui.output_mut(|o| o.copied_text = info);
                                                        }
                                                        if ui
                                                            .button("🔌")
                                                            .on_hover_text("Close Connection")
                                                            .clicked()
                                                        {
                                                            conn_to_close = Some(conn.connection_id.clone());
                                                        }
                                                    });
                                                }
                                            }
                                            if files.is_empty() && conns.is_empty() {
                                                ui.weak("  No active resources");
                                            }
                                        },
                                    );
                            },
                        );
                        ui.add_space(8.0);
                    }
                },
            );
        if let Some(path) = path_to_open {
            self.open_file_path(&path);
        }
        if let Some(conn_id) = conn_to_close {
            self.close_network_connection(&conn_id);
        }
        if let Some((name, path)) = program_to_add {
            self.add_custom_program(name, path, String::new(), false);
        }
        if let Some(pid) = pid_to_kill {
            self.kill_confirm_pid = Some(pid);
            self.kill_confirm_thumbnail = None;
            if let Some(window) = self.windows.iter().find(|w| w.pid == pid) {
                if let Some(image) = self
                    .capture_window_thumbnail(window.window_id, 300, 200)
                {
                    self.kill_confirm_thumbnail = Some(
                        ui
                            .ctx()
                            .load_texture(
                                "kill_confirm_thumb",
                                image,
                                egui::TextureOptions::LINEAR,
                            ),
                    );
                }
            }
        }
        if let Some(pid) = process_to_copy {
            if let Some(process) = self.processes.iter().find(|p| p.pid == pid) {
                let files: Vec<&FileHandle> = self
                    .file_handles
                    .iter()
                    .filter(|f| f.pid == pid)
                    .collect();
                let conns: Vec<&NetworkConnection> = self
                    .network_connections
                    .iter()
                    .filter(|c| c.pid == pid)
                    .collect();
                let mut process_data = format!(
                    "Process Information\n\
                    ====================\n\
                    Name: {}\n\
                    PID: {}\n\
                    Memory: {}\n\
                    CPU: {:.1}%\n\
                    Status: {}\n\
                    Run Time: {}\n",
                    process.name, process.pid, Self::format_memory(process.memory),
                    process.cpu_usage, process.status, Self::format_time(process
                    .run_time)
                );
                if let Some(exe_path) = &process.exe_path {
                    process_data.push_str(&format!("Executable: {}\n", exe_path));
                }
                process_data
                    .push_str(
                        &format!(
                            "Foreground: {}\n", if process.is_foreground { "Yes" } else {
                            "No" }
                        ),
                    );
                if !files.is_empty() {
                    process_data.push_str(&format!("\nOpen Files ({})\n", files.len()));
                    process_data.push_str("-------------\n");
                    for (i, file) in files.iter().enumerate() {
                        process_data
                            .push_str(
                                &format!(
                                    "{}. {} ({})\n", i + 1, file.path, file.access_type
                                ),
                            );
                    }
                }
                if !conns.is_empty() {
                    process_data
                        .push_str(&format!("\nNetwork Connections ({})\n", conns.len()));
                    process_data.push_str("--------------------\n");
                    for (i, conn) in conns.iter().enumerate() {
                        process_data
                            .push_str(
                                &format!(
                                    "{}. {} {} -> {} [{}]\n", i + 1, conn.protocol, conn
                                    .local_addr, conn.remote_addr, conn.state
                                ),
                            );
                    }
                }
                if files.is_empty() && conns.is_empty() {
                    process_data.push_str("\nNo active resources\n");
                }
                ui.output_mut(|o| o.copied_text = process_data);
                self.add_log(
                    format!(
                        "Copied data for process {} ({}) to clipboard", process.name,
                        process.pid
                    ),
                );
            }
        }
    }
    pub fn add_log(&mut self, message: String) {
        use chrono::Local;
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!("{} - {}", timestamp, message);
        self.logs.push(log_entry);
        if self.logs.len() > 1000 {
            self.logs.remove(0);
        }
    }
}
