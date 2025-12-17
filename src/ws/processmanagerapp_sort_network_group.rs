use crate::ws::SortColumn;
// # ProcessManagerApp - sort_network_group Methods
//
// This module contains method implementations for `ProcessManagerApp`.
//
// 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    fn sort_network(&mut self) {
        let ascending = self.sort_ascending;
        match self.sort_column {
            SortColumn::Pid => {
                self.network_connections
                    .sort_by(|a, b| {
                        if ascending { a.pid.cmp(&b.pid) } else { b.pid.cmp(&a.pid) }
                    });
            }
            SortColumn::Name => {
                self.network_connections
                    .sort_by(|a, b| {
                        if ascending {
                            a.process_name.cmp(&b.process_name)
                        } else {
                            b.process_name.cmp(&a.process_name)
                        }
                    });
            }
            SortColumn::Protocol => {
                self.network_connections
                    .sort_by(|a, b| {
                        if ascending {
                            a.protocol.cmp(&b.protocol)
                        } else {
                            b.protocol.cmp(&a.protocol)
                        }
                    });
            }
            SortColumn::LocalAddr => {
                self.network_connections
                    .sort_by(|a, b| {
                        if ascending {
                            a.local_addr.cmp(&b.local_addr)
                        } else {
                            b.local_addr.cmp(&a.local_addr)
                        }
                    });
            }
            SortColumn::RemoteAddr => {
                self.network_connections
                    .sort_by(|a, b| {
                        if ascending {
                            a.remote_addr.cmp(&b.remote_addr)
                        } else {
                            b.remote_addr.cmp(&a.remote_addr)
                        }
                    });
            }
            SortColumn::NetState => {
                self.network_connections
                    .sort_by(|a, b| {
                        if ascending {
                            a.state.cmp(&b.state)
                        } else {
                            b.state.cmp(&a.state)
                        }
                    });
            }
            _ => {}
        }
    }
}
