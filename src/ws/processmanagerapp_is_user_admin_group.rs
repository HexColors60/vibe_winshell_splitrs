//! # ProcessManagerApp - is_user_admin_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub fn is_user_admin() -> bool {
        #[cfg(windows)]
        {
            use windows::Win32::UI::Shell::IsUserAnAdmin;
            unsafe { IsUserAnAdmin().as_bool() }
        }
        #[cfg(not(windows))] { false }
    }
}
