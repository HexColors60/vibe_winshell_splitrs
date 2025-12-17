use crate::ws::AppConfig;
use crate::egui::Theme;
use crate::ws::CustomProgram;
//! # ProcessManagerApp - load_config_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub fn load_config() -> AppConfig {
        let config_path = Self::get_config_path();
        let mut config = AppConfig {
            programs: Vec::new(),
            font_path: String::new(),
            use_noto: false,
            theme: Theme::Dark,
            live_grid_size: 3,
            live_detail_percent: 0.5,
            attempt_start_as_admin: true,
        };
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            let mut current_program = CustomProgram {
                name: String::new(),
                path: String::new(),
                args: String::new(),
                admin: false,
            };
            let mut in_program = false;
            let mut in_settings = false;
            for line in content.lines() {
                let line = line.trim();
                if line.starts_with("[Program]") {
                    if in_program && !current_program.name.is_empty()
                        && !current_program.path.is_empty()
                    {
                        config.programs.push(current_program.clone());
                    }
                    current_program = CustomProgram {
                        name: String::new(),
                        path: String::new(),
                        args: String::new(),
                        admin: false,
                    };
                    in_program = true;
                    in_settings = false;
                } else if line.starts_with("[Settings]") {
                    if in_program && !current_program.name.is_empty()
                        && !current_program.path.is_empty()
                    {
                        config.programs.push(current_program.clone());
                    }
                    in_settings = true;
                    in_program = false;
                } else if in_program {
                    if let Some((key, value)) = line.split_once('=') {
                        let key = key.trim();
                        let value = value.trim();
                        match key {
                            "Name" => current_program.name = value.to_string(),
                            "Path" => current_program.path = value.to_string(),
                            "Args" => current_program.args = value.to_string(),
                            "Admin" => current_program.admin = value == "true",
                            _ => {}
                        }
                    }
                } else if in_settings {
                    if let Some((key, value)) = line.split_once('=') {
                        let key = key.trim();
                        let value = value.trim();
                        match key {
                            "FontPath" => config.font_path = value.to_string(),
                            "UseNoto" => config.use_noto = value == "true",
                            "Theme" => {
                                if value == "Light" {
                                    config.theme = Theme::Light;
                                } else {
                                    config.theme = Theme::Dark;
                                }
                            }
                            "LiveGridSize" => {
                                if let Ok(size) = value.parse() {
                                    config.live_grid_size = size;
                                }
                            }
                            "LiveDetailPercent" => {
                                if let Ok(p) = value.parse() {
                                    config.live_detail_percent = p;
                                }
                            }
                            "AttemptStartAsAdmin" => {
                                config.attempt_start_as_admin = value == "true";
                            }
                            _ => {}
                        }
                    }
                }
            }
            if in_program && !current_program.name.is_empty()
                && !current_program.path.is_empty()
            {
                config.programs.push(current_program);
            }
        }
        config
    }
}
