use std::time::Instant;
use crate::ws::WindowInfo;
use crate::ws::ProcessInfo;
use crate::ws::FileHandle;
use crate::ws::NetworkConnection;
//! # ProcessManagerApp - refresh_all_data_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub fn refresh_all_data(&mut self) {
        self.refresh_windows();
        self.refresh_processes();
        self.refresh_file_handles();
        self.refresh_network_connections();
        self.update_history();
        self.last_update = Instant::now();
    }
    fn update_history(&mut self) {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let total_cpu: f32 = self.processes.iter().map(|p| p.cpu_usage).sum();
        self.cpu_history.push((elapsed, total_cpu as f64));
        let total_memory: u64 = self.processes.iter().map(|p| p.memory).sum();
        self.memory_history
            .push((elapsed, total_memory as f64 / (1024.0 * 1024.0 * 1024.0)));
        if self.cpu_history.len() > 100 {
            self.cpu_history.remove(0);
        }
        if self.memory_history.len() > 100 {
            self.memory_history.remove(0);
        }
    }
    fn refresh_windows(&mut self) {
        self.windows.clear();
        #[cfg(windows)]
        {
            use windows::Win32::Foundation::{HWND, LPARAM, BOOL};
            use windows::Win32::UI::WindowsAndMessaging::{
                EnumWindows, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible,
                GetForegroundWindow,
            };
            use std::sync::Mutex;
            let foreground_hwnd = unsafe { GetForegroundWindow() };
            let mut foreground_window_id = 0u64;
            let mut pid_map: HashMap<u32, String> = HashMap::new();
            for process in &self.processes {
                pid_map.insert(process.pid, process.name.clone());
            }
            let windows_list: Mutex<Vec<(HWND, u32, String)>> = Mutex::new(Vec::new());
            unsafe extern "system" fn enum_window_callback(
                hwnd: HWND,
                lparam: LPARAM,
            ) -> BOOL {
                let windows_list = &*(lparam.0
                    as *const Mutex<Vec<(HWND, u32, String)>>);
                if IsWindowVisible(hwnd).as_bool() {
                    let mut title: [u16; 512] = [0; 512];
                    let len = GetWindowTextW(hwnd, &mut title);
                    if len > 0 {
                        let window_title = String::from_utf16_lossy(
                            &title[..len as usize],
                        );
                        if !window_title.is_empty() {
                            let mut pid: u32 = 0;
                            GetWindowThreadProcessId(hwnd, Some(&mut pid));
                            if pid > 0 {
                                if let Ok(mut list) = windows_list.lock() {
                                    list.push((hwnd, pid, window_title));
                                }
                            }
                        }
                    }
                }
                BOOL(1)
            }
            let windows_list_ptr = &windows_list as *const _ as isize;
            let _ = unsafe {
                EnumWindows(Some(enum_window_callback), LPARAM(windows_list_ptr))
            };
            if let Ok(list) = windows_list.lock() {
                for (hwnd, pid, window_title) in list.iter() {
                    let process_name = pid_map
                        .get(pid)
                        .cloned()
                        .unwrap_or_else(|| format!("pid-{}", pid));
                    let window_id = hwnd.0 as u64;
                    let is_foreground = hwnd.0 == foreground_hwnd.0;
                    if is_foreground {
                        foreground_window_id = window_id;
                    }
                    self.windows
                        .push(WindowInfo {
                            pid: *pid,
                            process_name,
                            window_title: window_title.clone(),
                            window_id,
                            is_foreground,
                        });
                }
            }
            self.foreground_window_id = if foreground_window_id > 0 {
                Some(foreground_window_id)
            } else {
                None
            };
        }
        #[cfg(target_os = "linux")]
        {
            for process in &self.processes {
                if process.name.contains("firefox") || process.name.contains("chrome")
                    || process.name.contains("code") || process.name.contains("terminal")
                    || process.name.contains("nautilus")
                    || process.name.contains("gedit")
                {
                    self.windows
                        .push(WindowInfo {
                            pid: process.pid,
                            process_name: process.name.clone(),
                            window_title: format!("{} - Window", process.name),
                            window_id: process.pid as u64,
                            is_foreground: false,
                        });
                }
            }
            if !self.windows.is_empty() {
                self.foreground_window_id = Some(self.windows[0].window_id);
                self.windows[0].is_foreground = true;
            }
        }
    }
    pub fn refresh_processes(&mut self) {
        self.system.refresh_cpu_all();
        self.system.refresh_memory();
        self.system.refresh_all();
        let foreground_pids: Vec<u32> = self
            .windows
            .iter()
            .filter(|w| w.is_foreground)
            .map(|w| w.pid)
            .collect();
        self.processes = self
            .system
            .processes()
            .iter()
            .map(|(pid, process)| {
                let pid_u32 = pid.as_u32();
                ProcessInfo {
                    pid: pid_u32,
                    name: process.name().to_string_lossy().to_string(),
                    memory: process.memory(),
                    cpu_usage: process.cpu_usage(),
                    parent_pid: process.parent().map(|p| p.as_u32()),
                    status: format!("{:?}", process.status()),
                    run_time: process.run_time(),
                    is_foreground: foreground_pids.contains(&pid_u32),
                    exe_path: process.exe().map(|p| p.to_string_lossy().to_string()),
                }
            })
            .collect();
        self.sort_processes();
    }
    fn refresh_file_handles(&mut self) {
        self.file_handles.clear();
        #[cfg(target_os = "linux")]
        {
            use procfs::process::all_processes;
            if let Ok(processes) = all_processes() {
                for process in processes.flatten() {
                    if let Ok(fds) = process.fd() {
                        let pid = process.pid() as u32;
                        let process_name = process
                            .stat()
                            .ok()
                            .and_then(|s| Some(s.comm))
                            .unwrap_or_else(|| format!("pid-{}", pid));
                        for fd in fds.flatten() {
                            if let procfs::process::FDTarget::Path(path) = &fd.target {
                                let size = std::fs::metadata(path)
                                    .ok()
                                    .map(|m| m.len())
                                    .unwrap_or(0);
                                if !path.to_string_lossy().starts_with("/dev")
                                    && !path.to_string_lossy().starts_with("/proc")
                                    && !path.to_string_lossy().starts_with("/sys")
                                {
                                    self.file_handles
                                        .push(FileHandle {
                                            pid,
                                            process_name: process_name.clone(),
                                            path: path.to_string_lossy().to_string(),
                                            size,
                                            access_type: "Open".to_string(),
                                        });
                                }
                            }
                        }
                    }
                }
            }
        }
        #[cfg(windows)]
        {
            for (pid, process) in self.system.processes() {
                let pid_u32 = pid.as_u32();
                if let Some(exe_path) = process.exe() {
                    let path_str = exe_path.to_string_lossy().to_string();
                    let size = std::fs::metadata(exe_path)
                        .ok()
                        .map(|m| m.len())
                        .unwrap_or(0);
                    self.file_handles
                        .push(FileHandle {
                            pid: pid_u32,
                            process_name: process.name().to_string_lossy().to_string(),
                            path: path_str,
                            size,
                            access_type: "Executable".to_string(),
                        });
                }
                if let Some(cwd) = process.cwd() {
                    if let Ok(entries) = std::fs::read_dir(cwd) {
                        for (idx, entry) in entries.flatten().take(5).enumerate() {
                            let path = entry.path();
                            if path.is_file() {
                                let size = std::fs::metadata(&path)
                                    .ok()
                                    .map(|m| m.len())
                                    .unwrap_or(0);
                                self.file_handles
                                    .push(FileHandle {
                                        pid: pid_u32,
                                        process_name: process.name().to_string_lossy().to_string(),
                                        path: path.to_string_lossy().to_string(),
                                        size,
                                        access_type: "Working Dir".to_string(),
                                    });
                            }
                            if idx >= 5 {
                                break;
                            }
                        }
                    }
                }
            }
        }
        #[cfg(target_os = "macos")] {}
    }
    pub fn refresh_network_connections(&mut self) {
        self.network_connections.clear();
        #[cfg(target_os = "linux")]
        {
            use procfs::net::{tcp, tcp6};
            let mut pid_map: HashMap<i32, String> = HashMap::new();
            for process in &self.processes {
                pid_map.insert(process.pid as i32, process.name.clone());
            }
            if let Ok(tcp_conns) = tcp() {
                for conn in tcp_conns {
                    let inode = conn.inode;
                    if let Ok(processes) = procfs::process::all_processes() {
                        for process in processes.flatten() {
                            if let Ok(fds) = process.fd() {
                                for fd in fds.flatten() {
                                    if let procfs::process::FDTarget::Socket(socket_inode) = fd
                                        .target
                                    {
                                        if socket_inode == inode {
                                            let pid = process.pid() as u32;
                                            let process_name = pid_map
                                                .get(&(pid as i32))
                                                .cloned()
                                                .unwrap_or_else(|| format!("pid-{}", pid));
                                            self.network_connections
                                                .push(NetworkConnection {
                                                    pid,
                                                    process_name,
                                                    protocol: "TCP".to_string(),
                                                    local_addr: format!("{}", conn.local_address),
                                                    remote_addr: format!("{}", conn.remote_address),
                                                    state: format!("{:?}", conn.state),
                                                    connection_id: format!("{}", inode),
                                                });
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if let Ok(tcp6_conns) = tcp6() {
                for conn in tcp6_conns {
                    let inode = conn.inode;
                    if let Ok(processes) = procfs::process::all_processes() {
                        for process in processes.flatten() {
                            if let Ok(fds) = process.fd() {
                                for fd in fds.flatten() {
                                    if let procfs::process::FDTarget::Socket(socket_inode) = fd
                                        .target
                                    {
                                        if socket_inode == inode {
                                            let pid = process.pid() as u32;
                                            let process_name = pid_map
                                                .get(&(pid as i32))
                                                .cloned()
                                                .unwrap_or_else(|| format!("pid-{}", pid));
                                            self.network_connections
                                                .push(NetworkConnection {
                                                    pid,
                                                    process_name,
                                                    protocol: "TCP6".to_string(),
                                                    local_addr: format!("{}", conn.local_address),
                                                    remote_addr: format!("{}", conn.remote_address),
                                                    state: format!("{:?}", conn.state),
                                                    connection_id: format!("{}", inode),
                                                });
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        #[cfg(windows)]
        {
            use windows::Win32::NetworkManagement::IpHelper::{
                GetTcpTable2, GetTcp6Table2, MIB_TCP_STATE_CLOSED, MIB_TCP_STATE_LISTEN,
                MIB_TCP_STATE_SYN_SENT, MIB_TCP_STATE_SYN_RCVD, MIB_TCP_STATE_ESTAB,
                MIB_TCP_STATE_FIN_WAIT1, MIB_TCP_STATE_FIN_WAIT2,
                MIB_TCP_STATE_CLOSE_WAIT, MIB_TCP_STATE_CLOSING, MIB_TCP_STATE_LAST_ACK,
                MIB_TCP_STATE_TIME_WAIT, MIB_TCP_STATE_DELETE_TCB,
            };
            let mut pid_map: HashMap<u32, String> = HashMap::new();
            for process in &self.processes {
                pid_map.insert(process.pid, process.name.clone());
            }
            let state_to_string = |state: u32| -> &'static str {
                match state {
                    x if x == MIB_TCP_STATE_CLOSED.0 as u32 => "CLOSED",
                    x if x == MIB_TCP_STATE_LISTEN.0 as u32 => "LISTEN",
                    x if x == MIB_TCP_STATE_SYN_SENT.0 as u32 => "SYN_SENT",
                    x if x == MIB_TCP_STATE_SYN_RCVD.0 as u32 => "SYN_RCVD",
                    x if x == MIB_TCP_STATE_ESTAB.0 as u32 => "ESTABLISHED",
                    x if x == MIB_TCP_STATE_FIN_WAIT1.0 as u32 => "FIN_WAIT1",
                    x if x == MIB_TCP_STATE_FIN_WAIT2.0 as u32 => "FIN_WAIT2",
                    x if x == MIB_TCP_STATE_CLOSE_WAIT.0 as u32 => "CLOSE_WAIT",
                    x if x == MIB_TCP_STATE_CLOSING.0 as u32 => "CLOSING",
                    x if x == MIB_TCP_STATE_LAST_ACK.0 as u32 => "LAST_ACK",
                    x if x == MIB_TCP_STATE_TIME_WAIT.0 as u32 => "TIME_WAIT",
                    x if x == MIB_TCP_STATE_DELETE_TCB.0 as u32 => "DELETE_TCB",
                    _ => "UNKNOWN",
                }
            };
            unsafe {
                let mut buffer_size = 0u32;
                let _ = GetTcpTable2(None, &mut buffer_size, false);
                if buffer_size > 0 {
                    let mut buffer = vec![0u8; buffer_size as usize];
                    let table_ptr = buffer.as_mut_ptr()
                        as *mut windows::Win32::NetworkManagement::IpHelper::MIB_TCPTABLE2;
                    if GetTcpTable2(Some(table_ptr), &mut buffer_size, false) == 0 {
                        let table = &*table_ptr;
                        let entries = std::slice::from_raw_parts(
                            table.table.as_ptr(),
                            table.dwNumEntries as usize,
                        );
                        for entry in entries {
                            let pid = entry.dwOwningPid;
                            let process_name = pid_map
                                .get(&pid)
                                .cloned()
                                .unwrap_or_else(|| format!("pid-{}", pid));
                            let local_addr = format!(
                                "{}.{}.{}.{}:{}", entry.dwLocalAddr & 0xFF, (entry
                                .dwLocalAddr >> 8) & 0xFF, (entry.dwLocalAddr >> 16) & 0xFF,
                                (entry.dwLocalAddr >> 24) & 0xFF, u16::from_be(entry
                                .dwLocalPort as u16)
                            );
                            let remote_addr = format!(
                                "{}.{}.{}.{}:{}", entry.dwRemoteAddr & 0xFF, (entry
                                .dwRemoteAddr >> 8) & 0xFF, (entry.dwRemoteAddr >> 16) &
                                0xFF, (entry.dwRemoteAddr >> 24) & 0xFF, u16::from_be(entry
                                .dwRemotePort as u16)
                            );
                            self.network_connections
                                .push(NetworkConnection {
                                    pid,
                                    process_name,
                                    protocol: "TCP".to_string(),
                                    local_addr,
                                    remote_addr,
                                    state: state_to_string(entry.dwState).to_string(),
                                    connection_id: format!(
                                        "{}-{}-{}", pid, entry.dwLocalPort, entry.dwRemotePort
                                    ),
                                });
                        }
                    }
                }
                let mut buffer_size6 = 0u32;
                let _ = GetTcp6Table2(std::ptr::null_mut(), &mut buffer_size6, false);
                if buffer_size6 > 0 {
                    let mut buffer6 = vec![0u8; buffer_size6 as usize];
                    let table6_ptr = buffer6.as_mut_ptr()
                        as *mut windows::Win32::NetworkManagement::IpHelper::MIB_TCP6TABLE2;
                    if GetTcp6Table2(table6_ptr, &mut buffer_size6, false) == 0 {
                        let table6 = &*table6_ptr;
                        let entries6 = std::slice::from_raw_parts(
                            table6.table.as_ptr(),
                            table6.dwNumEntries as usize,
                        );
                        for entry in entries6 {
                            let pid = entry.dwOwningPid;
                            let process_name = pid_map
                                .get(&pid)
                                .cloned()
                                .unwrap_or_else(|| format!("pid-{}", pid));
                            let local_addr = format!(
                                "[{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}]:{}",
                                entry.LocalAddr.u.Byte[0], entry.LocalAddr.u.Byte[1], entry
                                .LocalAddr.u.Byte[2], entry.LocalAddr.u.Byte[3], entry
                                .LocalAddr.u.Byte[4], entry.LocalAddr.u.Byte[5], entry
                                .LocalAddr.u.Byte[6], entry.LocalAddr.u.Byte[7], entry
                                .LocalAddr.u.Byte[8], entry.LocalAddr.u.Byte[9], entry
                                .LocalAddr.u.Byte[10], entry.LocalAddr.u.Byte[11], entry
                                .LocalAddr.u.Byte[12], entry.LocalAddr.u.Byte[13], entry
                                .LocalAddr.u.Byte[14], entry.LocalAddr.u.Byte[15],
                                u16::from_be(entry.dwLocalPort as u16)
                            );
                            let remote_addr = format!(
                                "[{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}]:{}",
                                entry.RemoteAddr.u.Byte[0], entry.RemoteAddr.u.Byte[1],
                                entry.RemoteAddr.u.Byte[2], entry.RemoteAddr.u.Byte[3],
                                entry.RemoteAddr.u.Byte[4], entry.RemoteAddr.u.Byte[5],
                                entry.RemoteAddr.u.Byte[6], entry.RemoteAddr.u.Byte[7],
                                entry.RemoteAddr.u.Byte[8], entry.RemoteAddr.u.Byte[9],
                                entry.RemoteAddr.u.Byte[10], entry.RemoteAddr.u.Byte[11],
                                entry.RemoteAddr.u.Byte[12], entry.RemoteAddr.u.Byte[13],
                                entry.RemoteAddr.u.Byte[14], entry.RemoteAddr.u.Byte[15],
                                u16::from_be(entry.dwRemotePort as u16)
                            );
                            self.network_connections
                                .push(NetworkConnection {
                                    pid,
                                    process_name,
                                    protocol: "TCP6".to_string(),
                                    local_addr,
                                    remote_addr,
                                    state: state_to_string(entry.State.0 as u32).to_string(),
                                    connection_id: format!(
                                        "{}-{}-{}", pid, entry.dwLocalPort, entry.dwRemotePort
                                    ),
                                });
                        }
                    }
                }
            }
        }
    }
}
