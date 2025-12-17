use crate::egui::Theme;
//! # ProcessManagerApp - show_settings_view_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub(crate) fn show_settings_view(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(
                ui,
                |ui| {
                    ui.heading("⚙️ Application Settings");
                    ui.add_space(20.0);
                    ui.group(|ui| {
                        ui.heading("Configuration Data");
                        ui.add_space(5.0);
                        let config_path = Self::get_config_path();
                        ui.label(format!("Path: {}", config_path.display()));
                        if ui.button("📂 Open Config Directory").clicked() {
                            if let Some(parent) = config_path.parent() {
                                #[cfg(target_os = "windows")]
                                let _ = std::process::Command::new("explorer")
                                    .arg(parent)
                                    .spawn();
                                #[cfg(target_os = "linux")]
                                let _ = std::process::Command::new("xdg-open")
                                    .arg(parent)
                                    .spawn();
                                #[cfg(target_os = "macos")]
                                let _ = std::process::Command::new("open")
                                    .arg(parent)
                                    .spawn();
                                self.add_log("Opened configuration directory".to_string());
                            }
                        }
                    });
                    ui.add_space(20.0);
                    ui.group(|ui| {
                        ui.heading("Appearance");
                        ui.add_space(10.0);
                        ui.label("Font Configuration:");
                        let fonts = vec![
                            ("Default", ""), ("Microsoft JhengHei (微軟正黑體)",
                            "C:\\Windows\\Fonts\\msjh.ttc"), ("PMingLiU (新細明體)",
                            "C:\\Windows\\Fonts\\mingliu.ttc"),
                        ];
                        let current_font_name = fonts
                            .iter()
                            .find(|(_, path)| *path == self.font_path)
                            .map(|(name, _)| *name)
                            .unwrap_or("Custom / Unknown");
                        egui::ComboBox::from_label("Select Font")
                            .selected_text(current_font_name)
                            .show_ui(
                                ui,
                                |ui| {
                                    for (name, path) in fonts {
                                        if ui
                                            .selectable_value(
                                                &mut self.font_path,
                                                path.to_string(),
                                                name,
                                            )
                                            .clicked()
                                        {
                                            self.save_config();
                                            self.configure_fonts(ui.ctx(), false);
                                        }
                                    }
                                },
                            );
                        if ui.button("📂 Browse System Fonts...").clicked() {
                            self.font_picker.is_open = true;
                            if self.font_picker.files.is_empty() {
                                let path = std::path::Path::new(
                                    &self.font_picker.directory,
                                );
                                if path.exists() && path.is_dir() {
                                    self.font_picker.files.clear();
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
                                }
                            }
                        }
                        if ui
                            .checkbox(
                                &mut self.use_noto_font,
                                "Use Noto Font (Legacy Option)",
                            )
                            .clicked()
                        {
                            self.save_config();
                        }
                        ui.label(
                            egui::RichText::new(
                                    "Example text: 你好, 世界! Hello World! 🌍",
                                )
                                .size(14.0),
                        );
                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);
                        ui.label("Theme:");
                        ui.horizontal(|ui| {
                            if ui
                                .selectable_label(self.theme == Theme::Dark, "🌙 Dark")
                                .clicked()
                            {
                                self.theme = Theme::Dark;
                                self.save_config();
                            }
                            if ui
                                .selectable_label(
                                    self.theme == Theme::Light,
                                    "☀️ Light",
                                )
                                .clicked()
                            {
                                self.theme = Theme::Light;
                                self.save_config();
                            }
                        });
                    });
                    ui.add_space(20.0);
                    ui.group(|ui| {
                        ui.heading("Behavior");
                        ui.add_space(10.0);
                        ui.checkbox(
                            &mut self.show_window_grid,
                            "Show window grid in Windows view",
                        );
                        ui.add_space(5.0);
                        ui.label("Live View Grid Size:");
                        ui.horizontal(|ui| {
                            let sizes = [2, 3, 4, 5, 6, 8];
                            for size in sizes {
                                if ui
                                    .selectable_label(
                                        self.live_grid_size == size,
                                        format!("{}x{}", size, size),
                                    )
                                    .clicked()
                                {
                                    self.live_grid_size = size;
                                    self.save_config();
                                }
                            }
                        });
                        ui.add_space(5.0);
                        ui.label("Live View Detail (%):");
                        ui.horizontal(|ui| {
                            let percents = [25, 50, 75, 100];
                            for p in percents {
                                let val = p as f32 / 100.0;
                                if ui
                                    .selectable_label(
                                        (self.live_detail_percent - val).abs() < 0.01,
                                        format!("{}%", p),
                                    )
                                    .clicked()
                                {
                                    self.live_detail_percent = val;
                                    self.save_config();
                                }
                            }
                        });
                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);
                        ui.label("Administrator Privileges:");
                        ui.horizontal(|ui| {
                            if ui
                                .checkbox(
                                    &mut self.attempt_start_as_admin,
                                    "Start WinShell as Administrator",
                                )
                                .on_hover_text(
                                    "When enabled, WinShell will attempt to restart with administrator privileges on startup\n\
                            This provides full system access but requires UAC confirmation",
                                )
                                .clicked()
                            {
                                self.save_config();
                            }
                            let is_admin = ProcessManagerApp::is_user_admin();
                            let status_color = if is_admin {
                                egui::Color32::from_rgb(100, 200, 100)
                            } else {
                                egui::Color32::from_rgb(200, 100, 100)
                            };
                            let status_text = if is_admin {
                                "✓ Running as Admin"
                            } else {
                                "○ Standard User"
                            };
                            ui.label(
                                egui::RichText::new(status_text)
                                    .color(status_color)
                                    .size(14.0),
                            );
                        });
                        #[cfg(windows)]
                        ui.checkbox(
                            &mut self.run_as_admin,
                            "Default to 'Run as Administrator' for new programs",
                        );
                    });
                },
            );
    }
    pub fn get_config_path() -> std::path::PathBuf {
        let current_dir_config = std::path::Path::new("vibe_winshell.ini");
        if current_dir_config.exists() {
            return current_dir_config.to_path_buf();
        }
        if let Some(home_dir) = dirs::home_dir() {
            let config_dir = home_dir.join(".config");
            if !config_dir.exists() {
                let _ = std::fs::create_dir_all(&config_dir);
            }
            return config_dir.join("vibe_winshell.ini");
        }
        std::path::PathBuf::from("vibe_winshell.ini")
    }
}
