//! # ProcessManagerApp - show_new_program_launcher_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub(crate) fn show_new_program_launcher(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(
                ui,
                |ui| {
                    ui.heading("🚀 Launch New Program");
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        ui.label("Program Path:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.program_path)
                                .desired_width(350.0)
                                .hint_text(
                                    "e.g., C:\\Windows\\System32\\notepad.exe or /usr/bin/gedit",
                                ),
                        );
                        if ui.button("📂 Browse...").clicked() {
                            if let Some(path) = self.browse_for_program() {
                                self.program_path = path;
                            }
                        }
                    });
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label("Arguments:      ");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.program_args)
                                .desired_width(400.0)
                                .hint_text("Optional arguments (e.g., --help or file.txt)"),
                        );
                    });
                    ui.add_space(10.0);
                    #[cfg(windows)]
                    ui.checkbox(&mut self.run_as_admin, "⚡ Run as Administrator");
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        if ui.button("▶ Launch Program").clicked() {
                            if !self.program_path.is_empty() {
                                let program_path = self.program_path.clone();
                                let program_args = self.program_args.clone();
                                let run_as_admin = self.run_as_admin;
                                match self
                                    .launch_program(&program_path, &program_args, run_as_admin)
                                {
                                    Ok(pid) => {
                                        if pid > 0 {
                                            ui.label(
                                                format!("✓ Program launched successfully! PID: {}", pid),
                                            );
                                        } else {
                                            ui.label(
                                                "✓ Program launched with elevation (PID unavailable)",
                                            );
                                        }
                                        self.program_path.clear();
                                        self.program_args.clear();
                                        self.refresh_all_data();
                                    }
                                    Err(e) => {
                                        ui.colored_label(
                                            egui::Color32::RED,
                                            format!("✗ Failed to launch: {}", e),
                                        );
                                    }
                                }
                            }
                        }
                        if ui.button("🗑 Clear").clicked() {
                            self.program_path.clear();
                            self.program_args.clear();
                        }
                    });
                    ui.add_space(30.0);
                    ui.separator();
                    ui.add_space(10.0);
                    ui.heading("Quick Launch");
                    ui.add_space(10.0);
                    ui.label("Common Programs:");
                    ui.add_space(5.0);
                    #[cfg(windows)]
                    {
                        ui.horizontal_wrapped(|ui| {
                            if ui.button("📝 Notepad").clicked() {
                                let _ = self
                                    .launch_program(
                                        "C:\\Windows\\System32\\notepad.exe",
                                        "",
                                        false,
                                    );
                                self.refresh_all_data();
                            }
                            if ui.button("💻 CMD").clicked() {
                                let _ = self
                                    .launch_program(
                                        "C:\\Windows\\System32\\cmd.exe",
                                        "",
                                        false,
                                    );
                                self.refresh_all_data();
                            }
                            if ui.button("⚡ CMD (Admin)").clicked() {
                                let _ = self
                                    .launch_program("C:\\Windows\\System32\\cmd.exe", "", true);
                                self.refresh_all_data();
                            }
                            if ui.button("🔧 PowerShell").clicked() {
                                let _ = self
                                    .launch_program(
                                        "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
                                        "",
                                        false,
                                    );
                                self.refresh_all_data();
                            }
                            if ui.button("⚡ PowerShell (Admin)").clicked() {
                                let _ = self
                                    .launch_program(
                                        "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
                                        "",
                                        true,
                                    );
                                self.refresh_all_data();
                            }
                            if ui.button("📁 Explorer").clicked() {
                                let _ = self
                                    .launch_program("C:\\Windows\\explorer.exe", "", false);
                                self.refresh_all_data();
                            }
                            if ui.button("⚙️ Task Manager").clicked() {
                                let _ = self
                                    .launch_program(
                                        "C:\\Windows\\System32\\taskmgr.exe",
                                        "",
                                        false,
                                    );
                                self.refresh_all_data();
                            }
                            if ui.button("🖩 Calculator").clicked() {
                                let _ = self
                                    .launch_program(
                                        "C:\\Windows\\System32\\calc.exe",
                                        "",
                                        false,
                                    );
                                self.refresh_all_data();
                            }
                        });
                        ui.add_space(10.0);
                        ui.label("System Configuration:");
                        ui.add_space(5.0);
                        ui.horizontal_wrapped(|ui| {
                            if ui.button("🌐 Network Config").clicked() {
                                let _ = self
                                    .launch_program("control.exe", "ncpa.cpl", false);
                            }
                            if ui.button("💻 System Properties").clicked() {
                                let _ = self
                                    .launch_program("control.exe", "sysdm.cpl", false);
                            }
                            if ui.button("📦 Programs & Features").clicked() {
                                let _ = self
                                    .launch_program("control.exe", "appwiz.cpl", false);
                            }
                            if ui.button("🔌 Device Manager").clicked() {
                                let _ = self
                                    .launch_program("mmc.exe", "devmgmt.msc", false);
                            }
                            if ui.button("💾 Disk Management").clicked() {
                                let _ = self
                                    .launch_program("mmc.exe", "diskmgmt.msc", false);
                            }
                            if ui.button("⚙️ Services").clicked() {
                                let _ = self
                                    .launch_program("mmc.exe", "services.msc", false);
                            }
                            if ui.button("📝 Registry Editor").clicked() {
                                let _ = self.launch_program("regedit.exe", "", true);
                            }
                            if ui.button("🎛️ Control Panel").clicked() {
                                let _ = self.launch_program("control.exe", "", false);
                            }
                            if ui.button("🛡️ Firewall").clicked() {
                                let _ = self
                                    .launch_program("control.exe", "firewall.cpl", false);
                            }
                        });
                    }
                    #[cfg(target_os = "linux")]
                    {
                        ui.horizontal_wrapped(|ui| {
                            if ui.button("📝 Text Editor (gedit)").clicked() {
                                let _ = self.launch_program("/usr/bin/gedit", "", false);
                                self.refresh_all_data();
                            }
                            if ui.button("💻 Terminal").clicked() {
                                let _ = self
                                    .launch_program("/usr/bin/gnome-terminal", "", false);
                                self.refresh_all_data();
                            }
                            if ui.button("📁 Files").clicked() {
                                let _ = self.launch_program("/usr/bin/nautilus", "", false);
                                self.refresh_all_data();
                            }
                            if ui.button("🌐 Firefox").clicked() {
                                let _ = self.launch_program("/usr/bin/firefox", "", false);
                                self.refresh_all_data();
                            }
                            if ui.button("🖩 Calculator").clicked() {
                                let _ = self
                                    .launch_program("/usr/bin/gnome-calculator", "", false);
                                self.refresh_all_data();
                            }
                        });
                    }
                    #[cfg(target_os = "macos")]
                    {
                        ui.horizontal_wrapped(|ui| {
                            if ui.button("📝 TextEdit").clicked() {
                                let _ = self
                                    .launch_program(
                                        "/Applications/TextEdit.app/Contents/MacOS/TextEdit",
                                        "",
                                        false,
                                    );
                                self.refresh_all_data();
                            }
                            if ui.button("💻 Terminal").clicked() {
                                let _ = self
                                    .launch_program(
                                        "/Applications/Utilities/Terminal.app/Contents/MacOS/Terminal",
                                        "",
                                        false,
                                    );
                                self.refresh_all_data();
                            }
                            if ui.button("📁 Finder").clicked() {
                                let _ = self
                                    .launch_program(
                                        "/System/Library/CoreServices/Finder.app/Contents/MacOS/Finder",
                                        "",
                                        false,
                                    );
                                self.refresh_all_data();
                            }
                        });
                    }
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(10.0);
                    ui.heading("ℹ️ Instructions");
                    ui.add_space(5.0);
                    ui.label(
                        "• Enter the full path to the executable you want to launch",
                    );
                    ui.label(
                        "• Add any command-line arguments in the Arguments field (optional)",
                    );
                    ui.label("• Click 'Launch Program' to start the process");
                    ui.label("• The new process will appear in the Processes tab");
                    ui.add_space(10.0);
                    ui.label(
                        "💡 Tip: Use the Quick Launch buttons for common programs",
                    );
                    if !self.custom_programs.is_empty() {
                        ui.add_space(20.0);
                        ui.separator();
                        ui.add_space(10.0);
                        ui.heading("⭐ Custom Programs");
                        ui.add_space(10.0);
                        ui.horizontal_wrapped(|ui| {
                            let programs = self.custom_programs.clone();
                            for program in programs {
                                if ui
                                    .button(format!("🚀 {}", program.name))
                                    .on_hover_text(&program.path)
                                    .clicked()
                                {
                                    let _ = self
                                        .launch_program(
                                            &program.path,
                                            &program.args,
                                            program.admin,
                                        );
                                    self.refresh_all_data();
                                }
                            }
                        });
                    }
                },
            );
    }
    fn browse_for_program(&mut self) -> Option<String> {
        #[cfg(windows)]
        {
            use windows::Win32::UI::Shell::Common::COMDLG_FILTERSPEC;
            use windows::Win32::UI::Shell::{
                IFileOpenDialog, FOS_FILEMUSTEXIST, SIGDN_FILESYSPATH,
            };
            use windows::Win32::System::Com::{
                CoCreateInstance, CoInitializeEx, COINIT_APARTMENTTHREADED,
                CLSCTX_INPROC_SERVER,
            };
            use windows::core::PCWSTR;
            unsafe {
                let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
                let dialog: IFileOpenDialog = match CoCreateInstance(
                    &windows::Win32::UI::Shell::FileOpenDialog,
                    None,
                    CLSCTX_INPROC_SERVER,
                ) {
                    Ok(d) => d,
                    Err(e) => {
                        self.add_log(format!("Failed to create file dialog: {}", e));
                        return None;
                    }
                };
                if let Ok(mut options) = dialog.GetOptions() {
                    options |= FOS_FILEMUSTEXIST;
                    let _ = dialog.SetOptions(options);
                }
                let filter_name = windows::core::w!("Executable Files");
                let filter_spec = windows::core::w!("*.exe;*.bat;*.cmd;*.ps1");
                let all_files_name = windows::core::w!("All Files");
                let all_files_spec = windows::core::w!("*.*");
                let filters = [
                    COMDLG_FILTERSPEC {
                        pszName: PCWSTR::from_raw(filter_name.as_ptr()),
                        pszSpec: PCWSTR::from_raw(filter_spec.as_ptr()),
                    },
                    COMDLG_FILTERSPEC {
                        pszName: PCWSTR::from_raw(all_files_name.as_ptr()),
                        pszSpec: PCWSTR::from_raw(all_files_spec.as_ptr()),
                    },
                ];
                let _ = dialog.SetFileTypes(&filters);
                if dialog.Show(None).is_ok() {
                    if let Ok(result) = dialog.GetResult() {
                        if let Ok(path_pwstr) = result.GetDisplayName(SIGDN_FILESYSPATH)
                        {
                            let path_string = path_pwstr.to_string().ok()?;
                            self.add_log(format!("Selected program: {}", path_string));
                            return Some(path_string);
                        }
                    }
                }
            }
            None
        }
        #[cfg(not(windows))]
        {
            self.add_log(
                "File browser not yet implemented on this platform. Please enter the path manually."
                    .to_string(),
            );
            None
        }
    }
    fn launch_program(
        &mut self,
        program: &str,
        args: &str,
        run_as_admin: bool,
    ) -> Result<u32, String> {
        let launch_info = if args.is_empty() {
            if run_as_admin {
                format!("Launching with elevation: {}", program)
            } else {
                format!("Launching: {}", program)
            }
        } else {
            if run_as_admin {
                format!("Launching with elevation: {} {}", program, args)
            } else {
                format!("Launching: {} {}", program, args)
            }
        };
        self.add_log(launch_info);
        #[cfg(windows)]
        {
            if run_as_admin {
                use windows::Win32::UI::Shell::{ShellExecuteW, SE_ERR_ACCESSDENIED};
                use windows::Win32::Foundation::HWND;
                use windows::core::PCWSTR;
                use std::os::windows::ffi::OsStrExt;
                use std::ffi::OsStr;
                unsafe {
                    let operation = windows::core::w!("runas");
                    let file: Vec<u16> = OsStr::new(program)
                        .encode_wide()
                        .chain(std::iter::once(0))
                        .collect();
                    let parameters: Vec<u16> = if !args.is_empty() {
                        OsStr::new(args)
                            .encode_wide()
                            .chain(std::iter::once(0))
                            .collect()
                    } else {
                        vec![0]
                    };
                    let directory: Vec<u16> = if let Some(parent) = std::path::Path::new(
                            program,
                        )
                        .parent()
                    {
                        OsStr::new(&parent.to_string_lossy().to_string())
                            .encode_wide()
                            .chain(std::iter::once(0))
                            .collect()
                    } else {
                        vec![0]
                    };
                    let result = ShellExecuteW(
                        HWND(std::ptr::null_mut()),
                        PCWSTR::from_raw(operation.as_ptr()),
                        PCWSTR::from_raw(file.as_ptr()),
                        PCWSTR::from_raw(parameters.as_ptr()),
                        PCWSTR::from_raw(directory.as_ptr()),
                        windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL,
                    );
                    if result.0 as i32 <= 32 {
                        let error_msg = if result.0 as i32 == SE_ERR_ACCESSDENIED as i32
                        {
                            "Access denied. User canceled the elevation prompt."
                                .to_string()
                        } else {
                            format!("ShellExecute failed with code: {}", result.0 as i32)
                        };
                        self.add_log(format!("✗ Failed to launch: {}", error_msg));
                        return Err(error_msg);
                    }
                    self.add_log(
                        "✓ Successfully launched program with elevation".to_string(),
                    );
                    return Ok(0);
                }
            }
        }
        use std::process::Command;
        let mut cmd = Command::new(program);
        if !args.is_empty() {
            for arg in args.split_whitespace() {
                cmd.arg(arg);
            }
        }
        if let Some(parent) = std::path::Path::new(program).parent() {
            if parent.exists() {
                cmd.current_dir(parent);
            }
        }
        match cmd.spawn() {
            Ok(child) => {
                let pid = child.id();
                self.add_log(
                    format!("✓ Successfully launched program with PID: {}", pid),
                );
                Ok(pid)
            }
            Err(e) => {
                let error_msg = format!("✗ Failed to launch: {}", e);
                self.add_log(error_msg.clone());
                Err(error_msg)
            }
        }
    }
}
