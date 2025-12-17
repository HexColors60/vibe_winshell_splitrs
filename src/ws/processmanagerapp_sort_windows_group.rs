use crate::ws::SortColumn;
// # ProcessManagerApp - sort_windows_group Methods
//
// This module contains method implementations for `ProcessManagerApp`.
//
// 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    fn sort_windows(&mut self) {
        let ascending = self.sort_ascending;
        match self.sort_column {
            SortColumn::Pid => {
                self.windows
                    .sort_by(|a, b| {
                        if ascending { a.pid.cmp(&b.pid) } else { b.pid.cmp(&a.pid) }
                    });
            }
            SortColumn::Name => {
                self.windows
                    .sort_by(|a, b| {
                        if ascending {
                            a.process_name.cmp(&b.process_name)
                        } else {
                            b.process_name.cmp(&a.process_name)
                        }
                    });
            }
            SortColumn::WindowTitle => {
                self.windows
                    .sort_by(|a, b| {
                        if ascending {
                            a.window_title.cmp(&b.window_title)
                        } else {
                            b.window_title.cmp(&a.window_title)
                        }
                    });
            }
            _ => {}
        }
    }
}
