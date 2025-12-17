use sysinfo::System;
use crate::ws::ProcessInfo;
use crate::ws::FileHandle;
use crate::ws::NetworkConnection;
use crate::ws::WindowInfo;
use std::time::Instant;
use std::time::Duration;
use crate::ws::SortColumn;
use crate::ws::ViewMode;
use crate::egui::Theme;
use crate::ws::CustomProgram;
use crate::ws::FontPickerState;
use crate::ws::FilepaneTab;
use crate::ws::FileInfo;
use crate::ws::FilepaneCommand;
// Auto-generated module
//
// 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashSet, HashMap};

pub struct ProcessManagerApp {
    pub(super) system: System,
    pub(super) processes: Vec<ProcessInfo>,
    pub(super) file_handles: Vec<FileHandle>,
    pub(super) network_connections: Vec<NetworkConnection>,
    pub(super) windows: Vec<WindowInfo>,
    pub(super) last_update: Instant,
    pub(super) update_interval: Duration,
    pub(super) search_filter: String,
    pub(super) sort_column: SortColumn,
    pub(super) sort_ascending: bool,
    pub(super) selected_pid: Option<u32>,
    pub(super) auto_refresh: bool,
    pub(super) view_mode: ViewMode,
    pub(super) theme: Theme,
    pub(super) show_graphs: bool,
    pub(super) cpu_history: Vec<(f64, f64)>,
    pub(super) memory_history: Vec<(f64, f64)>,
    pub(super) start_time: Instant,
    pub(super) custom_refresh_input: String,
    pub(super) show_refresh_input: bool,
    pub(super) foreground_window_id: Option<u64>,
    pub(super) items_per_page: usize,
    pub(super) current_page: usize,
    pub(super) show_window_grid: bool,
    pub(super) program_path: String,
    pub(super) program_args: String,
    pub(super) logs: Vec<String>,
    pub(super) selected_log_indices: HashSet<usize>,
    pub(super) run_as_admin: bool,
    pub(super) custom_programs: Vec<CustomProgram>,
    pub(super) taskbar_page: usize,
    pub(super) live_page: usize,
    pub(super) live_thumbnails: HashMap<u64, egui::TextureHandle>,
    pub(super) live_current_capture_index: usize,
    pub(super) live_last_capture: Instant,
    pub(super) live_capture_interval: Duration,
    pub(super) live_paused: bool,
    pub(super) kill_confirm_pid: Option<u32>,
    pub(super) kill_confirm_thumbnail: Option<egui::TextureHandle>,
    pub(super) show_settings: bool,
    pub(super) use_noto_font: bool,
    pub(super) font_path: String,
    pub(super) font_picker: FontPickerState,
    pub(super) live_grid_size: usize,
    pub(super) live_detail_percent: f32,
    pub(super) attempt_start_as_admin: bool,
    pub(super) filepane_tabs: Vec<FilepaneTab>,
    pub(super) filepane_active_tab: usize,
    pub(super) filepane_swap_columns: bool,
    pub(super) filepane_config_path: String,
    pub(super) context_menu_file: Option<(FileInfo, usize)>,
    pub(super) show_context_menu: bool,
    pub(super) conversation_history: Vec<String>,
    pub(super) filepane_show_confirm: bool,
    pub(super) filepane_confirm_action: Option<FilepaneCommand>,
    pub(super) filepane_confirm_message: String,
}
