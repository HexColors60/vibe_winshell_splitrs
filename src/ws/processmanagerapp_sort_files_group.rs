use crate::ws::SortColumn;
// # ProcessManagerApp - sort_files_group Methods
//
// This module contains method implementations for `ProcessManagerApp`.
//
// 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub fn sort_files(&mut self) {
        let ascending = self.sort_ascending;
        match self.sort_column {
            SortColumn::Pid => {
                self.file_handles
                    .sort_by(|a, b| {
                        if ascending { a.pid.cmp(&b.pid) } else { b.pid.cmp(&a.pid) }
                    });
            }
            SortColumn::Name => {
                self.file_handles
                    .sort_by(|a, b| {
                        if ascending {
                            a.process_name.cmp(&b.process_name)
                        } else {
                            b.process_name.cmp(&a.process_name)
                        }
                    });
            }
            SortColumn::FilePath => {
                self.file_handles
                    .sort_by(|a, b| {
                        if ascending { a.path.cmp(&b.path) } else { b.path.cmp(&a.path) }
                    });
            }
            SortColumn::FileSize => {
                self.file_handles
                    .sort_by(|a, b| {
                        if ascending { a.size.cmp(&b.size) } else { b.size.cmp(&a.size) }
                    });
            }
            SortColumn::FileAccess => {
                self.file_handles
                    .sort_by(|a, b| {
                        if ascending {
                            a.access_type.cmp(&b.access_type)
                        } else {
                            b.access_type.cmp(&a.access_type)
                        }
                    });
            }
            _ => {}
        }
    }
}
