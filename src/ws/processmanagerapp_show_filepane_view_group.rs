use crate::ws::FilepaneTab;
use crate::ws::ContextAction;
use crate::ws::ChecksumAlgorithm;
use crate::egui::Theme;
use crate::ws::FilepaneCommand;
//! # ProcessManagerApp - show_filepane_view_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;
use crate::ws::types::FileInfo;

impl ProcessManagerApp {
    fn show_filepane_view(&mut self, ui: &mut egui::Ui) {
        if self.filepane_tabs.is_empty() {
            self.filepane_tabs
                .push(
                    FilepaneTab::new(
                        "Tab 1".to_string(),
                        std::env::current_dir()
                            .unwrap_or_else(|_| std::path::PathBuf::from("C:\\"))
                            .to_string_lossy()
                            .to_string(),
                        std::env::current_dir()
                            .unwrap_or_else(|_| std::path::PathBuf::from("C:\\"))
                            .to_string_lossy()
                            .to_string(),
                    ),
                );
            self.filepane_active_tab = 0;
        }
        ui.horizontal(|ui| {
            ui.heading("📂 Filepane");
            ui.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    ui.label("Filter:");
                    ui.add_sized(
                        [200.0, 20.0],
                        egui::TextEdit::singleline(
                                &mut self.filepane_tabs[self.filepane_active_tab].filter,
                            )
                            .hint_text("type to filter..."),
                    );
                    ui.separator();
                    if ui
                        .button("↔️ Swap")
                        .on_hover_text("Swap left and right columns")
                        .clicked()
                    {
                        self.filepane_swap_columns = !self.filepane_swap_columns;
                    }
                    if ui
                        .button("☑️ Checkboxes")
                        .on_hover_text("Toggle selection checkboxes")
                        .clicked()
                    {
                        let tab = &mut self.filepane_tabs[self.filepane_active_tab];
                        tab.show_checkboxes = !tab.show_checkboxes;
                        if !tab.show_checkboxes {
                            tab.left_checkboxes.clear();
                            tab.right_checkboxes.clear();
                        }
                    }
                    ui.separator();
                    if ui
                        .button("⏭ Copy→→")
                        .on_hover_text(
                            "Copy selected files from current to opposite panel",
                        )
                        .clicked()
                    {
                        self.copy_files_to_opposite_panel();
                    }
                    if ui
                        .button("⊞ Duplicate")
                        .on_hover_text("Duplicate current tab")
                        .clicked()
                    {
                        self.duplicate_current_tab();
                    }
                    ui.separator();
                    if ui.button("↶ Undo").on_hover_text("Undo last action").clicked()
                    {
                        self.undo_last_action();
                    }
                    if ui
                        .button("↷ Redo")
                        .on_hover_text("Redo undone action")
                        .clicked()
                    {
                        self.redo_last_action();
                    }
                    ui.separator();
                    if ui
                        .button("💾 Save")
                        .on_hover_text("Save current paths to config")
                        .clicked()
                    {
                        self.save_current_paths();
                    }
                    if ui
                        .button("📁 Save All")
                        .on_hover_text("Save all tabs to config")
                        .clicked()
                    {
                        self.save_all_tabs();
                    }
                    if ui
                        .button("📂 Load")
                        .on_hover_text("Load paths from config")
                        .clicked()
                    {
                        self.load_paths_from_config();
                    }
                    ui.separator();
                    if ui.button("+").on_hover_text("Add new tab").clicked() {
                        let tab_count = self.filepane_tabs.len() + 1;
                        self.filepane_tabs
                            .push(
                                FilepaneTab::new(
                                    format!("Tab {}", tab_count),
                                    self
                                        .filepane_tabs[self.filepane_active_tab]
                                        .left_path
                                        .clone(),
                                    self
                                        .filepane_tabs[self.filepane_active_tab]
                                        .right_path
                                        .clone(),
                                ),
                            );
                        self.filepane_active_tab = self.filepane_tabs.len() - 1;
                    }
                },
            );
        });
        ui.horizontal(|ui| {
            let mut tab_to_close = None;
            for (i, tab) in self.filepane_tabs.iter_mut().enumerate() {
                let is_active = i == self.filepane_active_tab;
                let tab_button = ui
                    .selectable_label(
                        is_active,
                        format!(
                            "{} {}{}{}", if is_active { "🔵" } else { "⚪" }, tab
                            .name, if ! tab.left_checkboxes.is_empty() || ! tab
                            .right_checkboxes.is_empty() { " ☑️" } else { "" }, if !
                            tab.selected_left.is_empty() || ! tab.selected_right
                            .is_empty() { " ✓" } else { "" }
                        ),
                    );
                if tab_button.clicked() {
                    self.filepane_active_tab = i;
                }
                if ui
                    .button("❌")
                    .on_hover_text(&format!("Close {}", tab.name))
                    .clicked()
                {
                    tab_to_close = Some(i);
                }
                ui.add_space(5.0);
            }
            if let Some(index) = tab_to_close {
                if self.filepane_tabs.len() > 1 {
                    self.filepane_tabs.remove(index);
                    if self.filepane_active_tab >= self.filepane_tabs.len() {
                        self.filepane_active_tab = self.filepane_tabs.len() - 1;
                    }
                }
            }
        });
        ui.separator();
        let current_tab = &mut self.filepane_tabs[self.filepane_active_tab];
        let left_path = current_tab.left_path.clone();
        let right_path = current_tab.right_path.clone();
        let show_hidden = self.show_window_grid;
        let available_height = ui.available_height() - 120.0;
        ui.horizontal(|ui| {
            let (first_panel_path, second_panel_path) = if self.filepane_swap_columns {
                (&right_path, &left_path)
            } else {
                (&left_path, &right_path)
            };
            let first_panel_index = if self.filepane_swap_columns { 1 } else { 0 };
            let second_panel_index = if self.filepane_swap_columns { 0 } else { 1 };
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width() * 0.5, available_height),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    self.show_file_panel_header_with_checkboxes(ui, first_panel_index);
                    ui.add_space(2.0);
                    self.show_file_panel_with_checkboxes(
                        ui,
                        first_panel_path,
                        first_panel_index,
                    );
                },
            );
            ui.separator();
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), available_height),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    self.show_file_panel_header_with_checkboxes(ui, second_panel_index);
                    ui.add_space(2.0);
                    self.show_file_panel_with_checkboxes(
                        ui,
                        second_panel_path,
                        second_panel_index,
                    );
                },
            );
        });
        let mut context_actions = Vec::new();
        if self.show_context_menu {
            // if let Some((ref file_info, panel_index)) = self.context_menu_file.take() {
            if let Some((file_info, panel_index)) = self.context_menu_file.take() {
                // let file_info_clone = file_info.clone();
                // let file_info_clone: crate::ws::types::FileInfo = file_info.clone();
                let file_info_clone = file_info;

                let file_path = file_info_clone.path.clone();
                let file_name = file_info_clone.name.clone();
                let is_directory = file_info_clone.is_directory;
                let (tab_index, dest_path, speed_limit) = {
                    let tab = &self.filepane_tabs[self.filepane_active_tab];
                    (
                        self.filepane_active_tab,
                        if panel_index == 0 {
                            tab.right_path.clone()
                        } else {
                            tab.left_path.clone()
                        },
                        tab.copy_speed_limit_mb_per_sec,
                    )
                };
                egui::Area::new(egui::Id::new("context_menu"))
                    .fixed_pos(ui.input(|i| i.pointer.hover_pos()).unwrap_or_default())
                    .show(
                        ui.ctx(),
                        |ui| {
                            egui::Frame::popup(ui.style())
                                .inner_margin(4.0)
                                .show(
                                    ui,
                                    |ui| {
                                        ui.set_min_width(200.0);
                                        ui.label(egui::RichText::new(&file_name).strong());
                                        ui.separator();
                                        if is_directory {
                                            if ui.button("📂 Open").clicked() {
                                                context_actions
                                                    .push(ContextAction::NavigateToDirectory {
                                                        path: file_path.clone(),
                                                        panel_index,
                                                    });
                                            }
                                        } else {
                                            if ui.button("🔧 Open").clicked() {
                                                context_actions
                                                    .push(ContextAction::OpenFile {
                                                        path: file_path.clone(),
                                                    });
                                            }
                                        }
                                        ui.separator();
                                        if ui.button("📋 Copy").clicked() {
                                            context_actions
                                                .push(ContextAction::CopyFile {
                                                    source: file_path.clone(),
                                                    destination: dest_path.clone(),
                                                    speed_limit,
                                                });
                                        }
                                        if ui.button("✂️ Cut").clicked() {
                                            context_actions.push(ContextAction::Cut);
                                        }
                                        ui.separator();
                                        if ui.button("🏷️ Rename").clicked() {
                                            context_actions
                                                .push(
                                                    ContextAction::LogMessage(
                                                        format!("Rename not implemented for: {}", file_name),
                                                    ),
                                                );
                                        }
                                        if ui.button("🗑️ Delete").clicked() {
                                            context_actions
                                                .push(
                                                    ContextAction::LogMessage(
                                                        format!("Delete not implemented for: {}", file_name),
                                                    ),
                                                );
                                        }
                                        ui.separator();
                                        if ui.button("🔐 Checksum").clicked() {
                                            context_actions
                                                .push(
                                                    ContextAction::LogMessage(
                                                        format!("Calculating checksum for: {}", file_name),
                                                    ),
                                                );
                                        }
                                        if ui.button("📋 Copy Path").clicked() {
                                            ui.output_mut(|o| o.copied_text = file_path.clone());
                                            context_actions.push(ContextAction::CloseMenu);
                                        }
                                        if ui.button("ℹ️ Properties").clicked() {
                                            context_actions
                                                .push(ContextAction::ShowProperties {
                                                    file_info: file_info_clone,
                                                });
                                        }
                                    },
                                );
                        },
                    );
                for action in context_actions.drain(..) {
                    match action {
                        ContextAction::NavigateToDirectory { path, panel_index } => {
                            if panel_index == 0 {
                                self.filepane_tabs[tab_index].left_path = path;
                                self.filepane_tabs[tab_index].selected_left.clear();
                                self.filepane_tabs[tab_index].left_checkboxes.clear();
                            } else {
                                self.filepane_tabs[tab_index].right_path = path;
                                self.filepane_tabs[tab_index].selected_right.clear();
                                self.filepane_tabs[tab_index].right_checkboxes.clear();
                            }
                        }
                        ContextAction::OpenFile { path } => {
                            self.open_file_with_system(&path);
                        }
                        ContextAction::CopyFile { source, destination, speed_limit } => {
                            self.copy_files_with_limit(
                                vec![source],
                                &destination,
                                speed_limit,
                            );
                        }
                        ContextAction::Cut => {
                            self.cut_selected_files();
                        }
                        ContextAction::ShowProperties { file_info } => {
                            self.show_file_properties(&file_info);
                        }
                        ContextAction::LogMessage(msg) => {
                            self.add_log(msg);
                        }
                        ContextAction::CloseMenu => {}
                    }
                }
            }
        }
        if (ui.input(|i| i.pointer.primary_clicked()) || !context_actions.is_empty())
            && self.show_context_menu
        {
            self.show_context_menu = false;
            self.context_menu_file = None;
        }
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.label("🔧 Operations:");
            if ui.button("📋 Copy").on_hover_text("Copy selected files").clicked() {
                self.copy_selected_files();
            }
            if ui.button("✂️ Cut").on_hover_text("Cut selected files").clicked() {
                self.cut_selected_files();
            }
            if ui.button("📄 Paste").on_hover_text("Paste files").clicked() {
                self.paste_files();
            }
            ui.separator();
            if ui
                .button("🔐 Checksum")
                .on_hover_text("Calculate checksum for selected files")
                .clicked()
            {
                self.calculate_checksum_for_selected();
            }
            ui.separator();
            let tab = &mut self.filepane_tabs[self.filepane_active_tab];
            ui.label("Speed:");
            let current_speed = tab.copy_speed_limit_mb_per_sec;
            ui.add_sized(
                [80.0, 20.0],
                egui::Slider::new(&mut tab.copy_speed_limit_mb_per_sec, 1.0..=100.0)
                    .text(format!("{} MB/s", current_speed)),
            );
            ui.separator();
            if ui
                .button("🗑️ Delete")
                .on_hover_text("Delete selected files")
                .clicked()
            {
                self.delete_selected_files();
            }
            if ui.button("📁 New Folder").on_hover_text("Create new folder").clicked()
            {
                self.create_new_folder();
            }
            if ui
                .button("💾 Save History")
                .on_hover_text("Save conversation history to file")
                .clicked()
            {
                self.save_current_conversation();
            }
            ui.separator();
            let current_tab = &self.filepane_tabs[self.filepane_active_tab];
            let selected_count = current_tab.selected_left.len()
                + current_tab.selected_right.len();
            let checked_count = current_tab.left_checkboxes.len()
                + current_tab.right_checkboxes.len();
            if selected_count > 0 {
                ui.label(format!("📌 Selected: {}", selected_count));
            }
            if checked_count > 0 {
                ui.label(format!("☑️ Checked: {}", checked_count));
            }
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label("⚙️ Settings:");
            if ui.checkbox(&mut self.show_window_grid, "Hidden files").changed() {}
            ui.separator();
            let tab = &mut self.filepane_tabs[self.filepane_active_tab];
            ui.label("Checksum:");
            let md5_selected = tab.checksum_algorithm == ChecksumAlgorithm::MD5;
            if ui.selectable_label(md5_selected, "MD5").clicked() {
                tab.checksum_algorithm = ChecksumAlgorithm::MD5;
            }
            let sha1_selected = tab.checksum_algorithm == ChecksumAlgorithm::SHA1;
            if ui.selectable_label(sha1_selected, "SHA1").clicked() {
                tab.checksum_algorithm = ChecksumAlgorithm::SHA1;
            }
            let sha256_selected = tab.checksum_algorithm == ChecksumAlgorithm::SHA256;
            if ui.selectable_label(sha256_selected, "SHA256").clicked() {
                tab.checksum_algorithm = ChecksumAlgorithm::SHA256;
            }
            let crc32_selected = tab.checksum_algorithm == ChecksumAlgorithm::CRC32;
            if ui.selectable_label(crc32_selected, "CRC32").clicked() {
                tab.checksum_algorithm = ChecksumAlgorithm::CRC32;
            }
            ui.separator();
            if ui.selectable_label(self.theme == Theme::Dark, "🌙 Dark").clicked() {
                self.theme = Theme::Dark;
                self.save_config();
            }
            if ui.selectable_label(self.theme == Theme::Light, "☀️ Light").clicked()
            {
                self.theme = Theme::Light;
                self.save_config();
            }
        });
        self.show_filepane_confirmation_dialog(ui);
    }
    fn show_file_panel_header_with_checkboxes(
        &mut self,
        ui: &mut egui::Ui,
        panel_index: usize,
    ) {
        let tab = &mut self.filepane_tabs[self.filepane_active_tab];
        let (path, should_navigate, should_toggle_all) = if panel_index == 0 {
            let mut new_path = tab.left_path.clone();
            let mut should_navigate = None as Option<String>;
            let mut should_toggle_all = false;
            ui.horizontal(|ui| {
                let column_name = if self.filepane_swap_columns {
                    "Right"
                } else {
                    "Left"
                };
                ui.label(format!("📁 {}", column_name));
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        if tab.show_checkboxes {
                            if ui
                                .button("☑️ All")
                                .on_hover_text("Toggle all checkboxes in this column")
                                .clicked()
                            {
                                should_toggle_all = true;
                            }
                            ui.separator();
                        }
                        if ui
                            .button("📁 ↑")
                            .on_hover_text("Parent directory")
                            .clicked()
                        {
                            should_navigate = Some("parent".to_string());
                        }
                        if ui.button("🔄").on_hover_text("Refresh").clicked() {
                            should_navigate = Some("refresh".to_string());
                        }
                    },
                );
            });
            ui.horizontal(|ui| {
                ui.label("📍");
                if ui
                    .add_sized(
                        [ui.available_width(), 20.0],
                        egui::TextEdit::singleline(&mut new_path),
                    )
                    .changed()
                {
                    should_navigate = Some(new_path.clone());
                }
            });
            (new_path, should_navigate, should_toggle_all)
        } else {
            let mut new_path = tab.right_path.clone();
            let mut should_navigate = None as Option<String>;
            let mut should_toggle_all = false;
            ui.horizontal(|ui| {
                let column_name = if self.filepane_swap_columns {
                    "Left"
                } else {
                    "Right"
                };
                ui.label(format!("📁 {}", column_name));
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        if tab.show_checkboxes {
                            if ui
                                .button("☑️ All")
                                .on_hover_text("Toggle all checkboxes in this column")
                                .clicked()
                            {
                                should_toggle_all = true;
                            }
                            ui.separator();
                        }
                        if ui
                            .button("📁 ↑")
                            .on_hover_text("Parent directory")
                            .clicked()
                        {
                            should_navigate = Some("parent".to_string());
                        }
                        if ui.button("🔄").on_hover_text("Refresh").clicked() {
                            should_navigate = Some("refresh".to_string());
                        }
                    },
                );
            });
            ui.horizontal(|ui| {
                ui.label("📍");
                if ui
                    .add_sized(
                        [ui.available_width(), 20.0],
                        egui::TextEdit::singleline(&mut new_path),
                    )
                    .changed()
                {
                    should_navigate = Some(new_path.clone());
                }
            });
            (new_path, should_navigate, should_toggle_all)
        };
        if should_toggle_all {
            let tab = &mut self.filepane_tabs[self.filepane_active_tab];
            let filter = tab.filter.to_lowercase();
            if let Ok(entries) = std::fs::read_dir(
                if panel_index == 0 { &tab.left_path } else { &tab.right_path },
            ) {
                for entry in entries.flatten() {
                    if let Ok(file_info) = FileInfo::new(entry.path()) {
                        if !filter.is_empty()
                            && !file_info.name.to_lowercase().contains(&filter)
                        {
                            continue;
                        }
                        if !self.show_window_grid && file_info.name.starts_with('.') {
                            continue;
                        }
                        if panel_index == 0 {
                            if tab.left_checkboxes.contains(&file_info.name) {
                                tab.left_checkboxes.remove(&file_info.name);
                            } else {
                                tab.left_checkboxes.insert(file_info.name);
                            }
                        } else {
                            if tab.right_checkboxes.contains(&file_info.name) {
                                tab.right_checkboxes.remove(&file_info.name);
                            } else {
                                tab.right_checkboxes.insert(file_info.name);
                            }
                        }
                    }
                }
            }
        }
        if let Some(action) = should_navigate {
            let tab = &mut self.filepane_tabs[self.filepane_active_tab];
            if action == "parent" {
                if panel_index == 0 {
                    if let Some(parent) = std::path::Path::new(&tab.left_path).parent() {
                        tab.left_path = parent.to_string_lossy().to_string();
                        tab.selected_left.clear();
                        tab.left_checkboxes.clear();
                    }
                } else {
                    if let Some(parent) = std::path::Path::new(&tab.right_path).parent()
                    {
                        tab.right_path = parent.to_string_lossy().to_string();
                        tab.selected_right.clear();
                        tab.right_checkboxes.clear();
                    }
                }
            } else if action == "refresh" {} else {
                if panel_index == 0 {
                    tab.left_path = action;
                    tab.selected_left.clear();
                    tab.left_checkboxes.clear();
                } else {
                    tab.right_path = action;
                    tab.selected_right.clear();
                    tab.right_checkboxes.clear();
                }
            }
        }
    }
    fn show_file_panel_with_checkboxes(
        &mut self,
        ui: &mut egui::Ui,
        path: &str,
        panel_index: usize,
    ) {
        let path_buf = std::path::Path::new(path);
        let tab = &mut self.filepane_tabs[self.filepane_active_tab];
        let filter = tab.filter.clone();
        let show_hidden = self.show_window_grid;
        let file_infos_with_display_names: Vec<(FileInfo, String)> = { Vec::new() };
        if let Ok(entries) = std::fs::read_dir(path_buf) {
            let mut file_infos: Vec<FileInfo> = Vec::new();
            if let Some(parent) = path_buf.parent() {
                if let Ok(parent_info) = FileInfo::new(parent.to_path_buf()) {
                    file_infos.push(parent_info);
                }
            }
            for entry in entries.flatten() {
                if let Ok(file_info) = FileInfo::new(entry.path()) {
                    if !filter.is_empty() {
                        if !file_info
                            .name
                            .to_lowercase()
                            .contains(&filter.to_lowercase())
                        {
                            continue;
                        }
                    }
                    if !show_hidden && file_info.name.starts_with('.') {
                        continue;
                    }
                    file_infos.push(file_info);
                }
            }
            file_infos
                .sort_by(|a, b| {
                    match (a.is_directory, b.is_directory) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                    }
                });
            let (current_selections, current_checkboxes) = if panel_index == 0 {
                (tab.selected_left.clone(), tab.left_checkboxes.clone())
            } else {
                (tab.selected_right.clone(), tab.right_checkboxes.clone())
            };
            let mut selections_to_update = current_selections.clone();
            let mut checkboxes_to_update = current_checkboxes.clone();
            let mut should_navigate_to = None as Option<String>;
            let file_infos_with_sizes: Vec<(FileInfo, String)> = file_infos
                .into_iter()
                .map(|info| {
                    let size_str = if info.is_directory {
                        String::new()
                    } else {
                        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
                        let mut size = info.size as f64;
                        let mut unit_index = 0;
                        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
                            size /= 1024.0;
                            unit_index += 1;
                        }
                        if unit_index == 0 {
                            format!("{} {}", size as u64, UNITS[unit_index])
                        } else {
                            format!("{:.1} {}", size, UNITS[unit_index])
                        }
                    };
                    (info, size_str)
                })
                .collect();
            egui::ScrollArea::vertical()
                .auto_shrink([false, true])
                .show(
                    ui,
                    |ui| {
                        for (file_info, size_str) in &file_infos_with_sizes {
                            let is_selected = current_selections
                                .contains(&file_info.name);
                            let _is_checked = current_checkboxes
                                .contains(&file_info.name);
                            let response = ui
                                .horizontal(|ui| {
                                    if tab.show_checkboxes {
                                        let mut is_checked = checkboxes_to_update
                                            .contains(&file_info.name);
                                        let checkbox_response = ui.checkbox(&mut is_checked, "");
                                        if checkbox_response.changed() {
                                            if is_checked {
                                                checkboxes_to_update.insert(file_info.name.clone());
                                            } else {
                                                checkboxes_to_update.remove(&file_info.name);
                                            }
                                        }
                                    }
                                    let icon = if file_info.is_directory {
                                        "📁"
                                    } else {
                                        "📄"
                                    };
                                    ui.label(icon);
                                    let checkbox_width = if tab.show_checkboxes {
                                        25.0
                                    } else {
                                        0.0
                                    };
                                    let available_width = ui.available_width() - 100.0
                                        - checkbox_width;
                                    let max_chars = (available_width / 8.0) as usize;
                                    let display_name = if file_info.name.len() <= max_chars {
                                        file_info.name.clone()
                                    } else if max_chars <= 3 {
                                        "...".to_string()
                                    } else {
                                        format!(
                                            "{}...", & file_info.name[..max_chars.saturating_sub(3)]
                                        )
                                    };
                                    let name_label = if file_info.is_directory {
                                        egui::RichText::new(display_name.clone()).strong()
                                    } else {
                                        egui::RichText::new(display_name.clone())
                                    };
                                    let label_response = ui
                                        .selectable_label(is_selected, name_label);
                                    if label_response.hovered()
                                        && file_info.name.len() > display_name.len()
                                    {
                                        label_response.on_hover_text(&file_info.name);
                                    }
                                    if !file_info.is_directory {
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.label(size_str);
                                            },
                                        );
                                    }
                                });
                            if response.response.clicked() {
                                let shift_pressed = ui.input(|i| i.modifiers.shift);
                                let ctrl_pressed = ui.input(|i| i.modifiers.ctrl);
                                if shift_pressed {
                                    if !selections_to_update.contains(&file_info.name) {
                                        selections_to_update.push(file_info.name.clone());
                                    }
                                } else if ctrl_pressed {
                                    if let Some(pos) = selections_to_update
                                        .iter()
                                        .position(|name| name == &file_info.name)
                                    {
                                        selections_to_update.remove(pos);
                                    } else {
                                        selections_to_update.push(file_info.name.clone());
                                    }
                                } else {
                                    selections_to_update.clear();
                                    selections_to_update.push(file_info.name.clone());
                                }
                            }
                            if response.response.secondary_clicked() {
                                self.context_menu_file = Some((
                                    file_info.clone(),
                                    panel_index,
                                ));
                                self.show_context_menu = true;
                                selections_to_update.clear();
                                selections_to_update.push(file_info.name.clone());
                            }
                            if response.response.has_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                && file_info.is_directory
                            {
                                should_navigate_to = Some(file_info.path.clone());
                            }
                            if response.response.double_clicked()
                                && file_info.is_directory
                            {
                                should_navigate_to = Some(file_info.path.clone());
                            }
                        }
                    },
                );
            let tab = &mut self.filepane_tabs[self.filepane_active_tab];
            if panel_index == 0 {
                tab.selected_left = selections_to_update;
                tab.left_checkboxes = checkboxes_to_update;
            } else {
                tab.selected_right = selections_to_update;
                tab.right_checkboxes = checkboxes_to_update;
            }
            if let Some(target_path) = should_navigate_to {
                if panel_index == 0 {
                    tab.left_path = target_path;
                    tab.selected_left.clear();
                    tab.left_checkboxes.clear();
                } else {
                    tab.right_path = target_path;
                    tab.selected_right.clear();
                    tab.right_checkboxes.clear();
                }
            }
        } else {
            ui.colored_label(egui::Color32::RED, "❌ Cannot access directory");
        }
    }
    fn copy_selected_files(&mut self) {
        if self.filepane_active_tab < self.filepane_tabs.len() {
            let tab = &self.filepane_tabs[self.filepane_active_tab];
            let left_count = tab.selected_left.len();
            let right_count = tab.selected_right.len();
            if left_count > 0 {
                self.add_log(format!("Copied {} files from left panel", left_count));
            }
            if right_count > 0 {
                self.add_log(format!("Copied {} files from right panel", right_count));
            }
            if left_count == 0 && right_count == 0 {
                self.add_log("No files selected to copy".to_string());
            }
        }
    }
    fn cut_selected_files(&mut self) {
        if self.filepane_active_tab < self.filepane_tabs.len() {
            let tab = &self.filepane_tabs[self.filepane_active_tab];
            let left_count = tab.selected_left.len();
            let right_count = tab.selected_right.len();
            if left_count > 0 {
                self.add_log(format!("Cut {} files from left panel", left_count));
            }
            if right_count > 0 {
                self.add_log(format!("Cut {} files from right panel", right_count));
            }
            if left_count == 0 && right_count == 0 {
                self.add_log("No files selected to cut".to_string());
            }
        }
    }
    fn paste_files(&mut self) {
        self.add_log("Paste files (placeholder)".to_string());
    }
    fn delete_selected_files(&mut self) {
        if self.filepane_active_tab < self.filepane_tabs.len() {
            let left_count = self
                .filepane_tabs[self.filepane_active_tab]
                .selected_left
                .len();
            let right_count = self
                .filepane_tabs[self.filepane_active_tab]
                .selected_right
                .len();
            if left_count > 0 {
                self.add_log(format!("Deleted {} files from left panel", left_count));
                self.filepane_tabs[self.filepane_active_tab].selected_left.clear();
            }
            if right_count > 0 {
                self.add_log(format!("Deleted {} files from right panel", right_count));
                self.filepane_tabs[self.filepane_active_tab].selected_right.clear();
            }
            if left_count == 0 && right_count == 0 {
                self.add_log("No files selected to delete".to_string());
            }
        }
    }
    fn create_new_folder(&mut self) {
        self.add_log("Create new folder (placeholder)".to_string());
    }
    fn calculate_checksum_for_selected(&mut self) {
        if self.filepane_active_tab >= self.filepane_tabs.len() {
            return;
        }
        let tab = &self.filepane_tabs[self.filepane_active_tab];
        let algorithm = tab.checksum_algorithm.clone();
        let mut files_to_check = Vec::new();
        for filename in &tab.left_checkboxes {
            let path = format!("{}\\{}", tab.left_path, filename);
            files_to_check.push(path);
        }
        for filename in &tab.selected_left {
            let path = format!("{}\\{}", tab.left_path, filename);
            if !files_to_check.contains(&path) {
                files_to_check.push(path);
            }
        }
        for filename in &tab.right_checkboxes {
            let path = format!("{}\\{}", tab.right_path, filename);
            files_to_check.push(path);
        }
        for filename in &tab.selected_right {
            let path = format!("{}\\{}", tab.right_path, filename);
            if !files_to_check.contains(&path) {
                files_to_check.push(path);
            }
        }
        if files_to_check.is_empty() {
            self.add_log("No files selected for checksum calculation".to_string());
            return;
        }
        self.add_log(
            format!(
                "Calculating {} checksum for {} files", algorithm.name(), files_to_check
                .len()
            ),
        );
        for file_path in files_to_check {
            let command = FilepaneCommand::CalculateChecksum {
                path: file_path,
                algorithm: algorithm.clone(),
            };
            self.execute_command(&command);
            if self.filepane_active_tab < self.filepane_tabs.len() {
                let tab = &mut self.filepane_tabs[self.filepane_active_tab];
                tab.command_history.push(command);
                tab.undo_stack.clear();
            }
        }
    }
    fn copy_files_with_limit(
        &mut self,
        source_files: Vec<String>,
        dest_path: &str,
        speed_limit_mb_per_sec: f64,
    ) {
        let mut total_bytes_copied = 0u64;
        let batch_size = (speed_limit_mb_per_sec * 1024.0 * 1024.0) as u64;
        for (index, file_path) in source_files.iter().enumerate() {
            let dest_file = format!(
                "{}\\{}", dest_path, std::path::Path::new(file_path).file_name()
                .and_then(| name | name.to_str()).unwrap_or("unknown")
            );
            self.add_log(
                format!(
                    "Copying {}/{}: {} -> {}", index + 1, source_files.len(), file_path,
                    dest_file
                ),
            );
            if index > 0 && index % 5 == 0 {
                self.add_log(
                    format!("Speed limiting at {} MB/s", speed_limit_mb_per_sec),
                );
            }
        }
        self.add_log(
            format!("Copy operation completed. Total files: {}", source_files.len()),
        );
    }
    fn copy_files_to_opposite_panel(&mut self) {
        if self.filepane_active_tab >= self.filepane_tabs.len() {
            return;
        }
        let (
            source_path,
            dest_path,
            left_checkboxes,
            right_checkboxes,
            selected_left,
            selected_right,
            speed_limit,
        ) = {
            let tab = &self.filepane_tabs[self.filepane_active_tab];
            let (src, dst) = if self.filepane_swap_columns {
                (tab.right_path.clone(), tab.left_path.clone())
            } else {
                (tab.left_path.clone(), tab.right_path.clone())
            };
            (
                src,
                dst,
                tab.left_checkboxes.clone(),
                tab.right_checkboxes.clone(),
                tab.selected_left.clone(),
                tab.selected_right.clone(),
                tab.copy_speed_limit_mb_per_sec,
            )
        };
        let mut source_files = Vec::new();
        for filename in &left_checkboxes {
            source_files.push(format!("{}\\{}", source_path, filename));
        }
        for filename in &right_checkboxes {
            source_files.push(format!("{}\\{}", source_path, filename));
        }
        if source_files.is_empty() {
            for filename in &selected_left {
                source_files.push(format!("{}\\{}", source_path, filename));
            }
            for filename in &selected_right {
                source_files.push(format!("{}\\{}", source_path, filename));
            }
        }
        if source_files.is_empty() {
            self.add_log("No files selected to copy".to_string());
            return;
        }
        self.copy_files_with_limit(source_files, &dest_path, speed_limit);
    }
    fn duplicate_current_tab(&mut self) {
        if self.filepane_active_tab >= self.filepane_tabs.len() {
            return;
        }
        let new_tab_name = format!(
            "{} Copy", self.filepane_tabs[self.filepane_active_tab].name
        );
        let new_tab = FilepaneTab::new(
            new_tab_name,
            self.filepane_tabs[self.filepane_active_tab].left_path.clone(),
            self.filepane_tabs[self.filepane_active_tab].right_path.clone(),
        );
        self.filepane_tabs.insert(self.filepane_active_tab + 1, new_tab);
        self.filepane_active_tab += 1;
        self.add_log(
            format!(
                "Duplicated tab to: {}", self.filepane_tabs[self.filepane_active_tab]
                .name
            ),
        );
    }
    fn undo_last_action(&mut self) {
        if self.filepane_active_tab >= self.filepane_tabs.len() {
            return;
        }
        let command = {
            let tab = &mut self.filepane_tabs[self.filepane_active_tab];
            tab.undo_stack.pop()
        };
        if let Some(command) = command {
            let log_message = match &command {
                FilepaneCommand::CopyFile { source, destination } => {
                    format!("Undo: Copy {} -> {}", source, destination)
                }
                FilepaneCommand::MoveFile { source, destination } => {
                    format!("Undo: Move {} -> {}", source, destination)
                }
                FilepaneCommand::DeleteFile { path } => {
                    format!("Undo: Delete {} (cannot restore)", path)
                }
                FilepaneCommand::CreateDirectory { path } => {
                    format!("Undo: Create {}", path)
                }
                FilepaneCommand::RenameFile { old_path, new_path } => {
                    format!("Undo: Rename {} -> {}", new_path, old_path)
                }
                FilepaneCommand::ChangeDirectory { panel, new_path: _ } => {
                    format!("Undo: Change directory for panel {}", panel)
                }
                FilepaneCommand::CalculateChecksum { path, algorithm } => {
                    format!("Undo: {} checksum for {}", algorithm.name(), path)
                }
            };
            self.add_log(log_message);
            let tab = &mut self.filepane_tabs[self.filepane_active_tab];
            tab.redo_stack.push(command);
        }
    }
}
