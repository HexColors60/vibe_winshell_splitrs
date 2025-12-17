use crate::egui::Theme;
//! # ProcessManagerApp - show_font_picker_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    fn show_font_picker(&mut self, ctx: &egui::Context) {
        let mut open = self.font_picker.is_open;
        egui::Window::new("🔤 Select Font")
            .open(&mut open)
            .resize(|r| r.fixed_size([600.0, 500.0]))
            .min_width(500.0)
            .min_height(400.0)
            .show(
                ctx,
                |ui| {
                    ui.heading("Browse Fonts");
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label("Directory:");
                        let response = ui
                            .add(
                                egui::TextEdit::singleline(&mut self.font_picker.directory)
                                    .desired_width(300.0),
                            );
                        if ui.button("📂 Scan").clicked()
                            || (response.lost_focus()
                                && response.ctx.input(|i| i.key_pressed(egui::Key::Enter)))
                        {
                            let path = std::path::Path::new(&self.font_picker.directory);
                            if path.exists() && path.is_dir() {
                                self.font_picker.files.clear();
                                self.font_picker.error_msg = None;
                                if let Ok(entries) = std::fs::read_dir(path) {
                                    for entry in entries.flatten() {
                                        if let Ok(file_type) = entry.file_type() {
                                            if file_type.is_file() {
                                                if let Some(name) = entry.file_name().to_str() {
                                                    let lower = name.to_lowercase();
                                                    if lower.ends_with(".ttf") || lower.ends_with(".ttc")
                                                        || lower.ends_with(".otf")
                                                    {
                                                        self.font_picker.files.push(name.to_string());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self.font_picker.files.sort();
                                }
                            } else {
                                self.font_picker.error_msg = Some(
                                    "Directory does not exist".to_string(),
                                );
                            }
                        }
                    });
                    if let Some(err) = &self.font_picker.error_msg {
                        ui.colored_label(egui::Color32::RED, err);
                    }
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label("Filter:");
                        ui.text_edit_singleline(&mut self.font_picker.filter);
                    });
                    ui.separator();
                    ui.columns(
                        2,
                        |columns| {
                            columns[0]
                                .vertical(|ui| {
                                    ui.label("Files:");
                                    egui::ScrollArea::vertical()
                                        .max_height(300.0)
                                        .show(
                                            ui,
                                            |ui| {
                                                let filter = self.font_picker.filter.to_lowercase();
                                                let files = self.font_picker.files.clone();
                                                for file in files {
                                                    if !filter.is_empty()
                                                        && !file.to_lowercase().contains(&filter)
                                                    {
                                                        continue;
                                                    }
                                                    let is_selected = self.font_picker.selected_file.as_ref()
                                                        == Some(&file);
                                                    if ui.selectable_label(is_selected, &file).clicked() {
                                                        self.font_picker.selected_file = Some(file.clone());
                                                        self.configure_fonts(ui.ctx(), true);
                                                    }
                                                }
                                            },
                                        );
                                });
                            columns[1]
                                .vertical(|ui| {
                                    ui.label("Preview:");
                                    let preview_family = egui::FontFamily::Name(
                                        "Preview".into(),
                                    );
                                    let has_preview = ctx
                                        .fonts(|f| f.families().contains(&preview_family));
                                    ui.add(
                                        egui::TextEdit::multiline(
                                                &mut self.font_picker.preview_text,
                                            )
                                            .font(
                                                egui::FontId::new(
                                                    24.0,
                                                    if has_preview {
                                                        preview_family
                                                    } else {
                                                        egui::FontFamily::Proportional
                                                    },
                                                ),
                                            )
                                            .desired_width(f32::INFINITY)
                                            .desired_rows(8),
                                    );
                                    ui.add_space(10.0);
                                    if let Some(selected) = &self.font_picker.selected_file {
                                        ui.label(format!("Selected: {}", selected));
                                        ui.add_space(5.0);
                                        if ui.button("✅ Apply Font").clicked() {
                                            let path = std::path::Path::new(&self.font_picker.directory)
                                                .join(selected);
                                            self.font_path = path.to_string_lossy().to_string();
                                            self.save_config();
                                            self.configure_fonts(ui.ctx(), false);
                                            self.font_picker.is_open = false;
                                        }
                                    } else {
                                        ui.label("Select a font to preview");
                                    }
                                });
                        },
                    );
                },
            );
        self.font_picker.is_open = open;
    }
    pub fn save_config(&self) {
        let config_path = Self::get_config_path();
        let mut content = String::new();
        content.push_str("[Settings]\n");
        content.push_str(&format!("FontPath={}\n", self.font_path));
        content.push_str(&format!("UseNoto={}\n", self.use_noto_font));
        content
            .push_str(
                &format!(
                    "Theme={}\n", if self.theme == Theme::Light { "Light" } else { "Dark"
                    }
                ),
            );
        content.push_str(&format!("LiveGridSize={}\n", self.live_grid_size));
        content.push_str(&format!("LiveDetailPercent={}\n", self.live_detail_percent));
        content
            .push_str(&format!("AttemptStartAsAdmin={}\n", self.attempt_start_as_admin));
        content.push_str("\n");
        for program in &self.custom_programs {
            content.push_str("[Program]\n");
            content.push_str(&format!("Name={}\n", program.name));
            content.push_str(&format!("Path={}\n", program.path));
            content.push_str(&format!("Args={}\n", program.args));
            content.push_str(&format!("Admin={}\n", program.admin));
            content.push_str("\n");
        }
        if let Err(e) = std::fs::write(&config_path, content) {
            eprintln!("Failed to save config: {}", e);
        }
    }
}
