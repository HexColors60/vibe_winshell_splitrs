use std::alloc::System;
use std::time::Instant;
use std::time::Duration;
use crate::ws::SortColumn;
use crate::ws::ViewMode;
use crate::ws::FontPickerState;
use crate::ws::FilepaneTab;
//! # ProcessManagerApp - restart_as_admin_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub fn restart_as_admin() -> Result<(), String> {
        #[cfg(windows)]
        {
            use windows::Win32::UI::Shell::ShellExecuteW;
            use windows::Win32::Foundation::HWND;
            use windows::core::PCWSTR;
            use std::os::windows::ffi::OsStrExt;
            use std::ffi::OsStr;
            use std::env;
            let exe_path = env::current_exe().map_err(|e| e.to_string())?;
            unsafe {
                let operation = windows::core::w!("runas");
                let file: Vec<u16> = OsStr::new(&exe_path)
                    .encode_wide()
                    .chain(std::iter::once(0))
                    .collect();
                let result = ShellExecuteW(
                    HWND(std::ptr::null_mut()),
                    PCWSTR::from_raw(operation.as_ptr()),
                    PCWSTR::from_raw(file.as_ptr()),
                    PCWSTR::null(),
                    PCWSTR::null(),
                    windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL,
                );
                if result.0 as i32 > 32 {
                    Ok(())
                } else {
                    Err("Failed to elevate".to_string())
                }
            }
        }
        #[cfg(not(windows))] { Err("Not supported on this platform".to_string()) }
    }
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        let start_time = Instant::now();
        let config = Self::load_config();
        let mut app = Self {
            system,
            processes: Vec::new(),
            file_handles: Vec::new(),
            network_connections: Vec::new(),
            windows: Vec::new(),
            last_update: Instant::now(),
            update_interval: Duration::from_secs(2),
            search_filter: String::new(),
            sort_column: SortColumn::Cpu,
            sort_ascending: false,
            selected_pid: None,
            auto_refresh: true,
            view_mode: ViewMode::Processes,
            theme: config.theme,
            show_graphs: false,
            cpu_history: Vec::new(),
            memory_history: Vec::new(),
            start_time,
            custom_refresh_input: String::from("2"),
            show_refresh_input: false,
            foreground_window_id: None,
            items_per_page: 50,
            current_page: 0,
            show_window_grid: false,
            program_path: String::new(),
            program_args: String::new(),
            logs: Vec::new(),
            selected_log_indices: HashSet::new(),
            run_as_admin: false,
            custom_programs: config.programs,
            taskbar_page: 0,
            live_page: 0,
            live_thumbnails: HashMap::new(),
            live_current_capture_index: 0,
            live_last_capture: Instant::now(),
            live_capture_interval: Duration::from_millis(200),
            live_paused: false,
            live_grid_size: config.live_grid_size,
            live_detail_percent: config.live_detail_percent,
            attempt_start_as_admin: config.attempt_start_as_admin,
            kill_confirm_pid: None,
            kill_confirm_thumbnail: None,
            show_settings: false,
            use_noto_font: config.use_noto,
            font_path: config.font_path,
            font_picker: FontPickerState::default(),
            filepane_tabs: vec![
                FilepaneTab::new("Tab 1".to_string(), std::env::current_dir()
                .unwrap_or_else(| _ | std::path::PathBuf::from("C:\\")).to_string_lossy()
                .to_string(), std::env::current_dir().unwrap_or_else(| _ |
                std::path::PathBuf::from("C:\\")).to_string_lossy().to_string(),)
            ],
            filepane_active_tab: 0,
            filepane_swap_columns: false,
            filepane_config_path: "filepane_config.ini".to_string(),
            context_menu_file: None,
            show_context_menu: false,
            conversation_history: Vec::new(),
            filepane_show_confirm: false,
            filepane_confirm_action: None,
            filepane_confirm_message: String::new(),
        };
        app.configure_fonts(&cc.egui_ctx, false);
        app.add_log("WinShell started".to_string());
        app.refresh_all_data();
        app
    }
}
