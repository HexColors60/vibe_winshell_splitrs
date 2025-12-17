use crate::egui::Theme;
//! # ProcessManagerApp - apply_theme_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    fn apply_theme(&self, ctx: &egui::Context) {
        let visuals = match self.theme {
            Theme::Dark => egui::Visuals::dark(),
            Theme::Light => egui::Visuals::light(),
        };
        ctx.set_visuals(visuals);
    }
}
