use crate::ws::ProcessManagerApp;
use crate::ws::ViewMode;
use crate::egui::Theme;
use std::time::Duration;
// # ProcessManagerApp - Trait Implementations
//
// This module contains trait implementations for `ProcessManagerApp`.
//
// ## Implemented Traits
//
// - `App`
//
// 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ProcessManagerApp;

impl eframe::App for ProcessManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_theme(ctx);
        if self.auto_refresh && self.last_update.elapsed() >= self.update_interval {
            self.refresh_all_data();
        }
        if self.auto_refresh {
            ctx.request_repaint();
        }
        egui::TopBottomPanel::top("top_panel")
            .show(
                ctx,
                |ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.heading("🔧 WinShell Monitor");
                            ui.separator();
                            ui.selectable_value(
                                &mut self.view_mode,
                                ViewMode::Processes,
                                "📊 Processes",
                            );
                            ui.selectable_value(
                                &mut self.view_mode,
                                ViewMode::Files,
                                "📁 Files",
                            );
                            ui.selectable_value(
                                &mut self.view_mode,
                                ViewMode::Network,
                                "🌐 Network",
                            );
                            ui.selectable_value(
                                &mut self.view_mode,
                                ViewMode::FilesNetwork,
                                "📂/🌐 F/N",
                            );
                            ui.selectable_value(
                                &mut self.view_mode,
                                ViewMode::Windows,
                                "🪟 Windows",
                            );
                            ui.selectable_value(
                                &mut self.view_mode,
                                ViewMode::Taskbar,
                                "🔲 Taskbar",
                            );
                            ui.selectable_value(
                                &mut self.view_mode,
                                ViewMode::Live,
                                "📸 Live",
                            );
                            ui.selectable_value(
                                &mut self.view_mode,
                                ViewMode::New,
                                "🚀 New",
                            );
                            ui.selectable_value(
                                &mut self.view_mode,
                                ViewMode::Logs,
                                "📋 Logs",
                            );
                            ui.selectable_value(
                                &mut self.view_mode,
                                ViewMode::Settings,
                                "⚙️ Settings",
                            );
                            ui.selectable_value(
                                &mut self.view_mode,
                                ViewMode::Filepane,
                                "📂 Filepane",
                            );
                            ui.separator();
                            if ui
                                .button(
                                    match self.theme {
                                        Theme::Dark => "☀️ Light",
                                        Theme::Light => "🌙 Dark",
                                    },
                                )
                                .clicked()
                            {
                                self.theme = match self.theme {
                                    Theme::Dark => Theme::Light,
                                    Theme::Light => Theme::Dark,
                                };
                            }
                            ui.separator();
                            ui.checkbox(&mut self.show_graphs, "📈 Graphs");
                            ui.separator();
                            if self.view_mode != ViewMode::Windows {
                                if ui.button("💾 Export CSV").clicked() {
                                    let result = match self.view_mode {
                                        ViewMode::Processes => self.export_processes_to_csv(),
                                        ViewMode::Files => self.export_files_to_csv(),
                                        ViewMode::Network => self.export_network_to_csv(),
                                        _ => Ok("".to_string()),
                                    };
                                    if let Ok(filename) = result {
                                        if !filename.is_empty() {
                                            println!("Exported to {}", filename);
                                        }
                                    }
                                }
                            }
                            if self.view_mode == ViewMode::Windows {
                                if ui
                                    .button(
                                        if self.show_window_grid {
                                            "📋 List"
                                        } else {
                                            "🔲 Grid"
                                        },
                                    )
                                    .clicked()
                                {
                                    self.show_window_grid = !self.show_window_grid;
                                }
                            }
                        });
                        ui.horizontal(|ui| {
                            if ui.button("🔄 Refresh").clicked() {
                                self.refresh_all_data();
                            }
                            ui.checkbox(&mut self.auto_refresh, "Auto-refresh");
                            if !self.show_refresh_input {
                                if ui.button("⚙️ Interval").clicked() {
                                    self.update_interval = match self.update_interval.as_secs()
                                    {
                                        1 => Duration::from_secs(2),
                                        2 => Duration::from_secs(5),
                                        5 => Duration::from_secs(10),
                                        _ => Duration::from_secs(1),
                                    };
                                    self.custom_refresh_input = self
                                        .update_interval
                                        .as_secs()
                                        .to_string();
                                }
                                ui.label(format!("({}s)", self.update_interval.as_secs()));
                                if ui.button("✏️").clicked() {
                                    self.show_refresh_input = true;
                                }
                            } else {
                                ui.label("Interval (s):");
                                let response = ui
                                    .text_edit_singleline(&mut self.custom_refresh_input);
                                if response.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                {
                                    if let Ok(seconds) = self
                                        .custom_refresh_input
                                        .parse::<u64>()
                                    {
                                        if seconds > 0 && seconds <= 60 {
                                            self.update_interval = Duration::from_secs(seconds);
                                        }
                                    }
                                    self.show_refresh_input = false;
                                }
                                if ui.button("✓").clicked() {
                                    if let Ok(seconds) = self
                                        .custom_refresh_input
                                        .parse::<u64>()
                                    {
                                        if seconds > 0 && seconds <= 60 {
                                            self.update_interval = Duration::from_secs(seconds);
                                        }
                                    }
                                    self.show_refresh_input = false;
                                }
                            }
                            ui.separator();
                            ui.label("🔍 Filter:");
                            ui.text_edit_singleline(&mut self.search_filter);
                            if ui.button("❌ Clear").clicked() {
                                self.search_filter.clear();
                            }
                            ui.separator();
                            let (_total_items, total_pages) = match self.view_mode {
                                ViewMode::Processes => {
                                    let total = self.processes.len();
                                    ui.label(format!("Total: {} processes", total));
                                    (total, self.total_pages(&self.processes))
                                }
                                ViewMode::Files => {
                                    let total = self.file_handles.len();
                                    ui.label(format!("Total: {} file handles", total));
                                    (total, self.total_pages(&self.file_handles))
                                }
                                ViewMode::Network => {
                                    let total = self.network_connections.len();
                                    ui.label(format!("Total: {} connections", total));
                                    (total, self.total_pages(&self.network_connections))
                                }
                                ViewMode::FilesNetwork => {
                                    let total = self.processes.len();
                                    ui.label(format!("F/N Manager: {} processes", total));
                                    (total, 0)
                                }
                                ViewMode::Windows => {
                                    let total = self.windows.len();
                                    ui.label(format!("Total: {} windows", total));
                                    (total, self.total_pages(&self.windows))
                                }
                                ViewMode::Taskbar => {
                                    let total = self.windows.len();
                                    let items_per_page = 100;
                                    let total_pages = (total + items_per_page - 1)
                                        / items_per_page;
                                    ui.label(format!("Taskbar: {} windows", total));
                                    (total, total_pages)
                                }
                                ViewMode::Live => {
                                    let total = self.windows.len();
                                    let items_per_page = 20;
                                    let total_pages = (total + items_per_page - 1)
                                        / items_per_page;
                                    ui.label(format!("Live: {} windows", total));
                                    (total, total_pages)
                                }
                                ViewMode::New => {
                                    ui.label("Program Launcher");
                                    (0, 0)
                                }
                                ViewMode::Logs => {
                                    let total = self.logs.len();
                                    ui.label(format!("Total: {} log entries", total));
                                    (total, 0)
                                }
                                ViewMode::Settings => {
                                    ui.label("⚙️ Application Settings");
                                    (0, 0)
                                }
                                ViewMode::Filepane => {
                                    ui.label("📂 Filepane File Manager");
                                    (0, 0)
                                }
                            };
                            ui.separator();
                            if total_pages > 1 {
                                if ui.button("◀ Prev").clicked() && self.current_page > 0
                                {
                                    self.current_page -= 1;
                                }
                                ui.label(
                                    format!("Page {}/{}", self.current_page + 1, total_pages),
                                );
                                if ui.button("Next ▶").clicked()
                                    && self.current_page + 1 < total_pages
                                {
                                    self.current_page += 1;
                                }
                            }
                        });
                    });
                },
            );
        egui::TopBottomPanel::bottom("bottom_panel")
            .show(
                ctx,
                |ui| {
                    ui.horizontal(|ui| {
                        match self.view_mode {
                            ViewMode::Processes => {
                                if let Some(pid) = self.selected_pid {
                                    let process_info = self
                                        .processes
                                        .iter()
                                        .find(|p| p.pid == pid)
                                        .cloned();
                                    if let Some(process) = process_info {
                                        ui.label(
                                            format!("Selected: {} (PID: {})", process.name, process.pid),
                                        );
                                        ui.separator();
                                        if ui.button("🗡️ Kill Process").clicked() {
                                            if self.kill_process(pid) {
                                                self.selected_pid = None;
                                                self.refresh_all_data();
                                            }
                                        }
                                        ui.separator();
                                        ui.label(
                                            format!("Memory: {}", Self::format_memory(process.memory)),
                                        );
                                        ui.label(format!("CPU: {:.2}%", process.cpu_usage));
                                        ui.label(
                                            format!("Runtime: {}", Self::format_time(process.run_time)),
                                        );
                                        if let Some(parent) = process.parent_pid {
                                            ui.label(format!("Parent PID: {}", parent));
                                        }
                                    }
                                } else {
                                    ui.label("No process selected");
                                }
                            }
                            ViewMode::Files
                            | ViewMode::Network
                            | ViewMode::FilesNetwork
                            | ViewMode::Windows => {
                                ui.label("Select an item from the list above");
                            }
                            ViewMode::Taskbar => {
                                ui.label("Click on a window icon to switch to it");
                            }
                            ViewMode::Live => {
                                ui.label("Live window thumbnails - click to switch");
                                ui.label(
                                    format!(
                                        "Update interval: {}ms", self.live_capture_interval
                                        .as_millis()
                                    ),
                                );
                            }
                            ViewMode::New => {
                                ui.label(
                                    "Enter a program path and arguments to launch a new process",
                                );
                            }
                            ViewMode::Logs => {
                                ui.label(
                                    "Application event log - tracks system activities and errors",
                                );
                            }
                            ViewMode::Settings => {
                                ui.label("Configure application settings and fonts");
                            }
                            ViewMode::Filepane => {
                                ui.label(
                                    "Two-column file manager with directory navigation and file operations",
                                );
                            }
                        }
                    });
                },
            );
        egui::CentralPanel::default()
            .show(
                ctx,
                |ui| {
                    if self.show_graphs && self.view_mode == ViewMode::Processes {
                        ui.vertical(|ui| {
                            ui.heading("System Resource History");
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label("CPU Usage (%)");
                                    use egui_plot::{Line, Plot, PlotPoints};
                                    let cpu_points: PlotPoints = self
                                        .cpu_history
                                        .iter()
                                        .map(|(x, y)| [*x, *y])
                                        .collect();
                                    let cpu_line = Line::new(cpu_points)
                                        .color(egui::Color32::from_rgb(255, 100, 100));
                                    Plot::new("cpu_plot")
                                        .height(150.0)
                                        .show(
                                            ui,
                                            |plot_ui| {
                                                plot_ui.line(cpu_line);
                                            },
                                        );
                                });
                                ui.separator();
                                ui.vertical(|ui| {
                                    ui.label("Memory Usage (GB)");
                                    use egui_plot::{Line, Plot, PlotPoints};
                                    let mem_points: PlotPoints = self
                                        .memory_history
                                        .iter()
                                        .map(|(x, y)| [*x, *y])
                                        .collect();
                                    let mem_line = Line::new(mem_points)
                                        .color(egui::Color32::from_rgb(100, 255, 100));
                                    Plot::new("memory_plot")
                                        .height(150.0)
                                        .show(
                                            ui,
                                            |plot_ui| {
                                                plot_ui.line(mem_line);
                                            },
                                        );
                                });
                            });
                            ui.separator();
                            self.show_process_list(ctx, ui);
                        });
                    } else {
                        match self.view_mode {
                            ViewMode::Processes => self.show_process_list(ctx, ui),
                            ViewMode::Files => self.show_file_list(ui),
                            ViewMode::Network => self.show_network_list(ui),
                            ViewMode::FilesNetwork => self.show_files_network_view(ui),
                            ViewMode::Windows => self.show_windows_view(ui),
                            ViewMode::Taskbar => self.show_taskbar_view(ui),
                            ViewMode::Live => self.show_live_view(ctx, ui),
                            ViewMode::New => self.show_new_program_launcher(ui),
                            ViewMode::Logs => self.show_logs_view(ui),
                            ViewMode::Settings => self.show_settings_view(ui),
                            ViewMode::Filepane => self.show_filepane_view(ui),
                        }
                    }
                    self.show_kill_confirm_dialog(ctx);
                    if self.font_picker.is_open {
                        self.show_font_picker(ctx);
                    }
                },
            );
    }
}

