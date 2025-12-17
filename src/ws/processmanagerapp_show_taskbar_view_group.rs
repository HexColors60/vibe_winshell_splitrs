use crate::ws::WindowInfo;
use crate::ws::ViewMode;
//! # ProcessManagerApp - show_taskbar_view_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    fn show_taskbar_view(&mut self, ui: &mut egui::Ui) {
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
        let items_per_page = 100;
        let total_pages = (filtered_windows.len() + items_per_page - 1) / items_per_page;
        if self.taskbar_page >= total_pages && total_pages > 0 {
            self.taskbar_page = total_pages - 1;
        }
        let start_idx = self.taskbar_page * items_per_page;
        let end_idx = (start_idx + items_per_page).min(filtered_windows.len());
        let page_windows = if start_idx < filtered_windows.len() {
            &filtered_windows[start_idx..end_idx]
        } else {
            &[]
        };
        ui.horizontal(|ui| {
            ui.heading("🔲 Taskbar - Click to switch windows");
            ui.separator();
            if total_pages > 1 {
                if ui.button("◀ Prev").clicked() && self.taskbar_page > 0 {
                    self.taskbar_page -= 1;
                }
                ui.label(
                    format!("Page {}/{}", self.taskbar_page + 1, total_pages.max(1)),
                );
                if ui.button("Next ▶").clicked() && self.taskbar_page + 1 < total_pages
                {
                    self.taskbar_page += 1;
                }
            }
            ui.label(
                format!(
                    " | Showing {}-{} of {}", start_idx + 1, end_idx.min(filtered_windows
                    .len()), filtered_windows.len()
                ),
            );
        });
        ui.separator();
        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(
                ui,
                |ui| {
                    let icon_size = 80.0;
                    let spacing = 5.0;
                    for (_chunk_idx, row_windows) in page_windows.chunks(10).enumerate()
                    {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);
                            for window in row_windows {
                                let is_foreground = window.is_foreground;
                                let bg_color = if is_foreground {
                                    egui::Color32::from_rgb(60, 120, 180)
                                } else {
                                    egui::Color32::from_rgb(45, 45, 50)
                                };
                                let border_color = if is_foreground {
                                    egui::Color32::from_rgb(100, 180, 255)
                                } else {
                                    egui::Color32::from_rgb(80, 80, 90)
                                };
                                egui::Frame::none()
                                    .fill(bg_color)
                                    .stroke(egui::Stroke::new(2.0, border_color))
                                    .rounding(8.0)
                                    .inner_margin(5.0)
                                    .show(
                                        ui,
                                        |ui| {
                                            ui.set_min_size(egui::vec2(icon_size, icon_size));
                                            ui.set_max_size(egui::vec2(icon_size, icon_size));
                                            let icon = Self::get_process_icon(&window.process_name);
                                            ui.vertical_centered(|ui| {
                                                let icon_response = ui
                                                    .add(
                                                        egui::Label::new(egui::RichText::new(icon).size(24.0))
                                                            .sense(egui::Sense::click()),
                                                    );
                                                let title_display = if window.window_title.len() > 10 {
                                                    format!(
                                                        "{}...", & window.window_title.chars().take(7).collect::<
                                                        String > ()
                                                    )
                                                } else {
                                                    window.window_title.clone()
                                                };
                                                let title_response = ui
                                                    .add(
                                                        egui::Label::new(
                                                                egui::RichText::new(&title_display)
                                                                    .size(9.0)
                                                                    .color(egui::Color32::WHITE),
                                                            )
                                                            .sense(egui::Sense::click()),
                                                    );
                                                let proc_display = if window.process_name.len() > 10 {
                                                    format!(
                                                        "{}...", & window.process_name.chars().take(7).collect::<
                                                        String > ()
                                                    )
                                                } else {
                                                    window.process_name.clone()
                                                };
                                                let proc_response = ui
                                                    .add(
                                                        egui::Label::new(
                                                                egui::RichText::new(&proc_display)
                                                                    .size(8.0)
                                                                    .color(egui::Color32::GRAY),
                                                            )
                                                            .sense(egui::Sense::click()),
                                                    );
                                                if icon_response.clicked() || title_response.clicked()
                                                    || proc_response.clicked()
                                                {
                                                    window_to_focus = Some(window.window_id);
                                                }
                                                let any_hovered = icon_response.hovered()
                                                    || title_response.hovered() || proc_response.hovered();
                                                if any_hovered {
                                                    egui::show_tooltip(
                                                        ui.ctx(),
                                                        ui.layer_id(),
                                                        egui::Id::new(window.window_id),
                                                        |ui| {
                                                            ui.label(format!("📋 {}", window.window_title));
                                                            ui.label(format!("🖥 {}", window.process_name));
                                                            ui.label(format!("PID: {}", window.pid));
                                                            if window.is_foreground {
                                                                ui.colored_label(
                                                                    egui::Color32::from_rgb(100, 200, 255),
                                                                    "🔷 Active",
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                if let Some(exe_path) = pid_to_exe.get(&window.pid) {
                                                    if ui.small_button("⭐").on_hover_text("ToNew").clicked() {
                                                        program_to_add = Some((
                                                            window.process_name.clone(),
                                                            exe_path.clone(),
                                                        ));
                                                    }
                                                }
                                            });
                                        },
                                    );
                            }
                        });
                        ui.add_space(spacing);
                    }
                    if let Some(window_id) = window_to_focus {
                        self.bring_window_to_foreground(window_id);
                        self.add_log(format!("Switched to window ID: {}", window_id));
                    }
                },
            );
        if let Some((name, path)) = program_to_add {
            self.add_custom_program(name, path, String::new(), false);
            self.view_mode = ViewMode::New;
        }
    }
    pub fn get_process_icon(process_name: &str) -> &'static str {
        let name_lower = process_name.to_lowercase();
        if name_lower.contains("chrome") {
            return "🌐";
        }
        if name_lower.contains("firefox") {
            return "🦊";
        }
        if name_lower.contains("edge") {
            return "🔷";
        }
        if name_lower.contains("opera") {
            return "🔴";
        }
        if name_lower.contains("brave") {
            return "🦁";
        }
        if name_lower.contains("safari") {
            return "🧭";
        }
        if name_lower.contains("code") || name_lower.contains("vscode") {
            return "💻";
        }
        if name_lower.contains("visual studio") {
            return "🟣";
        }
        if name_lower.contains("sublime") {
            return "📝";
        }
        if name_lower.contains("atom") {
            return "⚛️";
        }
        if name_lower.contains("vim") || name_lower.contains("nvim") {
            return "📗";
        }
        if name_lower.contains("notepad") {
            return "📄";
        }
        if name_lower.contains("terminal") || name_lower.contains("cmd")
            || name_lower.contains("powershell")
        {
            return "⬛";
        }
        if name_lower.contains("wt") {
            return "⬛";
        }
        if name_lower.contains("word") {
            return "📘";
        }
        if name_lower.contains("excel") {
            return "📊";
        }
        if name_lower.contains("powerpoint") {
            return "📙";
        }
        if name_lower.contains("outlook") {
            return "📧";
        }
        if name_lower.contains("onenote") {
            return "📓";
        }
        if name_lower.contains("spotify") {
            return "🎵";
        }
        if name_lower.contains("vlc") {
            return "🎬";
        }
        if name_lower.contains("itunes") {
            return "🎶";
        }
        if name_lower.contains("music") {
            return "🎵";
        }
        if name_lower.contains("video") || name_lower.contains("player") {
            return "🎥";
        }
        if name_lower.contains("photo") || name_lower.contains("image") {
            return "🖼️";
        }
        if name_lower.contains("discord") {
            return "💬";
        }
        if name_lower.contains("slack") {
            return "💼";
        }
        if name_lower.contains("teams") {
            return "👥";
        }
        if name_lower.contains("zoom") {
            return "📹";
        }
        if name_lower.contains("skype") {
            return "📞";
        }
        if name_lower.contains("telegram") {
            return "✈️";
        }
        if name_lower.contains("whatsapp") {
            return "💚";
        }
        if name_lower.contains("explorer") {
            return "📁";
        }
        if name_lower.contains("finder") {
            return "📂";
        }
        if name_lower.contains("nautilus") {
            return "📂";
        }
        if name_lower.contains("steam") {
            return "🎮";
        }
        if name_lower.contains("game") {
            return "🎮";
        }
        if name_lower.contains("epic") {
            return "🎮";
        }
        if name_lower.contains("settings") {
            return "⚙️";
        }
        if name_lower.contains("control") {
            return "🎛️";
        }
        if name_lower.contains("task") {
            return "📊";
        }
        "🪟"
    }
}
