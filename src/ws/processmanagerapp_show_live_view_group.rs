use crate::ws::WindowInfo;
use std::time::Instant;
use std::time::Duration;
// # ProcessManagerApp - show_live_view_group Methods
//
// This module contains method implementations for `ProcessManagerApp`.
//
// 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub(crate) fn show_live_view(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let filter_lower = self.search_filter.to_lowercase();
        let mut window_to_focus: Option<u64> = None;
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
        let available_size = ui.available_size();
        let safe_width = if available_size.x > 100.0 { available_size.x } else { 800.0 };
        let safe_height = if available_size.y > 100.0 {
            available_size.y
        } else {
            600.0
        };
        let num_cols = self.live_grid_size.max(1);
        let num_rows = self.live_grid_size.max(1);
        let spacing = 10.0;
        let cell_width = (safe_width - ((num_cols as f32 - 1.0) * spacing))
            / num_cols as f32;
        let cell_height = (safe_height - ((num_rows as f32 - 1.0) * spacing))
            / num_rows as f32;
        let target_w = (cell_width * self.live_detail_percent).max(32.0);
        let target_h = (cell_height * self.live_detail_percent).max(32.0);
        if !self.live_paused
            && self.live_last_capture.elapsed() >= self.live_capture_interval
            && !filtered_windows.is_empty()
        {
            let window_idx = self.live_current_capture_index % filtered_windows.len();
            let window = &filtered_windows[window_idx];
            if let Some(image_data) = self
                .capture_window_thumbnail(
                    window.window_id,
                    target_w as i32,
                    target_h as i32,
                )
            {
                let texture = ctx
                    .load_texture(
                        format!("window_thumb_{}", window.window_id),
                        image_data,
                        egui::TextureOptions::default(),
                    );
                self.live_thumbnails.insert(window.window_id, texture);
            }
            self.live_current_capture_index += 1;
            self.live_last_capture = Instant::now();
        }
        let items_per_page = (self.live_grid_size * self.live_grid_size).max(1);
        let total_pages = (filtered_windows.len() + items_per_page - 1) / items_per_page;
        if self.live_page >= total_pages && total_pages > 0 {
            self.live_page = total_pages - 1;
        }
        let start_idx = self.live_page * items_per_page;
        let end_idx = (start_idx + items_per_page).min(filtered_windows.len());
        let page_windows = if start_idx < filtered_windows.len() {
            &filtered_windows[start_idx..end_idx]
        } else {
            &[]
        };
        ui.horizontal(|ui| {
            ui.heading("📸 Live Preview - Real-time Window Thumbnails");
            ui.separator();
            if self.live_paused {
                if ui.button("▶️ Resume").clicked() {
                    self.live_paused = false;
                }
                ui.colored_label(egui::Color32::YELLOW, "⏸ PAUSED");
            } else {
                if ui.button("⏹️ Stop").clicked() {
                    self.live_paused = true;
                }
            }
            ui.separator();
            if total_pages > 1 {
                if ui.button("◀ Prev").clicked() && self.live_page > 0 {
                    self.live_page -= 1;
                }
                ui.label(format!("Page {}/{}", self.live_page + 1, total_pages.max(1)));
                if ui.button("Next ▶").clicked() && self.live_page + 1 < total_pages {
                    self.live_page += 1;
                }
            }
            ui.label(
                format!(
                    " | Showing {}-{} of {}", start_idx + 1, end_idx.min(filtered_windows
                    .len()), filtered_windows.len()
                ),
            );
            ui.separator();
            ui.label("Update speed:");
            if ui.button("🐢 Slow").clicked() {
                self.live_capture_interval = Duration::from_millis(500);
            }
            if ui.button("🐇 Fast").clicked() {
                self.live_capture_interval = Duration::from_millis(100);
            }
        });
        ui.separator();
        let spacing = 10.0;
        let available_size = ui.available_size();
        let num_cols = self.live_grid_size.max(1);
        let num_rows = self.live_grid_size.max(1);
        let thumb_width = (available_size.x - ((num_cols as f32 - 1.0) * spacing))
            / num_cols as f32;
        let cell_height = (available_size.y - ((num_rows as f32 - 1.0) * spacing))
            / num_rows as f32;
        let thumb_height = (cell_height - 30.0).max(10.0);
        for row_windows in page_windows.chunks(num_cols) {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);
                for window in row_windows {
                    let is_foreground = window.is_foreground;
                    let bg_color = if is_foreground {
                        egui::Color32::from_rgb(60, 120, 180)
                    } else {
                        egui::Color32::from_rgb(35, 35, 40)
                    };
                    let border_color = if is_foreground {
                        egui::Color32::from_rgb(100, 180, 255)
                    } else {
                        egui::Color32::from_rgb(70, 70, 80)
                    };
                    egui::Frame::none()
                        .fill(bg_color)
                        .stroke(egui::Stroke::new(2.0, border_color))
                        .rounding(6.0)
                        .inner_margin(4.0)
                        .show(
                            ui,
                            |ui| {
                                ui.set_min_size(
                                    egui::vec2(thumb_width, thumb_height + 30.0),
                                );
                                ui.set_max_size(
                                    egui::vec2(thumb_width, thumb_height + 30.0),
                                );
                                ui.vertical(|ui| {
                                    let (rect, response) = ui
                                        .allocate_exact_size(
                                            egui::vec2(thumb_width - 8.0, thumb_height),
                                            egui::Sense::click(),
                                        );
                                    if let Some(texture) = self
                                        .live_thumbnails
                                        .get(&window.window_id)
                                    {
                                        ui.painter()
                                            .image(
                                                texture.id(),
                                                rect,
                                                egui::Rect::from_min_max(
                                                    egui::pos2(0.0, 0.0),
                                                    egui::pos2(1.0, 1.0),
                                                ),
                                                egui::Color32::WHITE,
                                            );
                                    } else {
                                        ui.painter()
                                            .rect_filled(
                                                rect,
                                                4.0,
                                                egui::Color32::from_rgb(50, 50, 55),
                                            );
                                        let icon = Self::get_process_icon(&window.process_name);
                                        ui.painter()
                                            .text(
                                                rect.center(),
                                                egui::Align2::CENTER_CENTER,
                                                icon,
                                                egui::FontId::proportional(32.0),
                                                egui::Color32::from_gray(150),
                                            );
                                    }
                                    if response.clicked() {
                                        window_to_focus = Some(window.window_id);
                                    }
                                    if response.hovered() {
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
                                    let title_display = if window.window_title.len() > 22 {
                                        format!(
                                            "{}...", & window.window_title.chars().take(19).collect::<
                                            String > ()
                                        )
                                    } else {
                                        window.window_title.clone()
                                    };
                                    let title_response = ui
                                        .add(
                                            egui::Label::new(
                                                    egui::RichText::new(&title_display)
                                                        .size(10.0)
                                                        .color(egui::Color32::WHITE),
                                                )
                                                .sense(egui::Sense::click()),
                                        );
                                    if title_response.clicked() {
                                        window_to_focus = Some(window.window_id);
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
        if !self.live_paused {
            ctx.request_repaint();
        }
    }
}
