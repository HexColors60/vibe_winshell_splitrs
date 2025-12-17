use crate::ws::SortColumn;
//! # ProcessManagerApp - sort_processes_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub fn sort_processes(&mut self) {
        let ascending = self.sort_ascending;
        match self.sort_column {
            SortColumn::Pid => {
                self.processes
                    .sort_by(|a, b| {
                        if ascending { a.pid.cmp(&b.pid) } else { b.pid.cmp(&a.pid) }
                    });
            }
            SortColumn::Name => {
                self.processes
                    .sort_by(|a, b| {
                        if ascending { a.name.cmp(&b.name) } else { b.name.cmp(&a.name) }
                    });
            }
            SortColumn::Memory => {
                self.processes
                    .sort_by(|a, b| {
                        if ascending {
                            a.memory.cmp(&b.memory)
                        } else {
                            b.memory.cmp(&a.memory)
                        }
                    });
            }
            SortColumn::Cpu => {
                self.processes
                    .sort_by(|a, b| {
                        if ascending {
                            a.cpu_usage
                                .partial_cmp(&b.cpu_usage)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        } else {
                            b.cpu_usage
                                .partial_cmp(&a.cpu_usage)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        }
                    });
            }
            SortColumn::ParentPid => {
                self.processes
                    .sort_by(|a, b| {
                        if ascending {
                            a.parent_pid.cmp(&b.parent_pid)
                        } else {
                            b.parent_pid.cmp(&a.parent_pid)
                        }
                    });
            }
            SortColumn::Status => {
                self.processes
                    .sort_by(|a, b| {
                        if ascending {
                            a.status.cmp(&b.status)
                        } else {
                            b.status.cmp(&a.status)
                        }
                    });
            }
            SortColumn::Runtime => {
                self.processes
                    .sort_by(|a, b| {
                        if ascending {
                            a.run_time.cmp(&b.run_time)
                        } else {
                            b.run_time.cmp(&a.run_time)
                        }
                    });
            }
            _ => {}
        }
    }
}
