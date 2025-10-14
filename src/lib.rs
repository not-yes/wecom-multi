// 核心库 - 跨平台 Mutex 管理和进程启动
// 支持 Windows 和 macOS

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpawnRequest {
    pub count: u8,
    pub app_path: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpawnResponse {
    pub pids: Vec<u32>,
    pub success: usize,
    pub failed: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub started_at: String,
}

// 平台特定实现
#[cfg(target_os = "windows")]
pub mod platform {
    use super::*;
    use std::{ffi::OsStr, mem, os::windows::ffi::OsStrExt, slice, time::Duration};
    use windows::{
        core::*, Win32::Foundation::*,
        Win32::System::Threading::*,
        Win32::System::WindowsProgramming::*,
    };

    const WECOM_MUTEX_NAME: &str = "Tencent.WeWork.ExclusiveObject";

    pub fn get_default_app_path() -> PathBuf {
        let possible_paths = vec![
            r"C:\Program Files (x86)\WXWork\WXWork.exe",
            r"C:\Program Files\WXWork\WXWork.exe",
            r"D:\Program Files (x86)\WXWork\WXWork.exe",
            r"D:\Program Files\WXWork\WXWork.exe",
        ];

        for path in possible_paths {
            let p = PathBuf::from(path);
            if p.exists() {
                return p;
            }
        }

        PathBuf::from(r"C:\Program Files (x86)\WXWork\WXWork.exe")
    }

    pub async fn spawn_multiple(req: SpawnRequest) -> std::result::Result<SpawnResponse, String> {
        let exe = req.app_path.unwrap_or(get_default_app_path());

        if !exe.exists() {
            return Err(format!("应用程序不存在: {:?}", exe));
        }

        let mut pids = vec![];
        let mut success = 0;
        let mut failed = 0;

        for i in 0..req.count {
            // 关闭 Mutex
            if let Err(e) = close_mutex(WECOM_MUTEX_NAME) {
                eprintln!("关闭 Mutex 失败: {}", e);
            }

            tokio::time::sleep(Duration::from_millis(100)).await;

            // 启动进程
            match launch_process(&exe) {
                Ok(pid) => {
                    pids.push(pid);
                    success += 1;
                }
                Err(e) => {
                    eprintln!("启动实例 {} 失败: {}", i + 1, e);
                    failed += 1;
                }
            }

            if i < req.count - 1 {
                tokio::time::sleep(Duration::from_millis(800)).await;
            }
        }

        Ok(SpawnResponse {
            pids,
            success,
            failed,
        })
    }

    fn close_mutex(_name: &str) -> std::result::Result<(), String> {
        unsafe {
            let h_current = GetCurrentProcess();
            let mut buf = vec![0u8; 64 * 1024];
            let mut ret_len = 0;

            let status = NtQuerySystemInformation(
                SystemExtendedHandleInformation,
                buf.as_mut_ptr() as _,
                buf.len() as u32,
                &mut ret_len,
            );

            if status != 0 {
                return Err("查询系统信息失败".to_string());
            }

            let info = &*(buf.as_ptr() as *const SYSTEM_HANDLE_INFORMATION_EX);
            let handles = slice::from_raw_parts(info.Handles.as_ptr(), info.NumberOfHandles as usize);

            for h in handles {
                if h.UniqueProcessId == GetCurrentProcessId() {
                    continue;
                }

                if let Ok(h_process) = OpenProcess(PROCESS_DUP_HANDLE, false, h.UniqueProcessId) {
                    let mut h_dup = HANDLE::default();

                    if DuplicateHandle(
                        h_process,
                        HANDLE(h.HandleValue as _),
                        h_current,
                        &mut h_dup,
                        0,
                        false,
                        DUPLICATE_CLOSE_SOURCE,
                    )
                    .is_ok()
                    {
                        let _ = CloseHandle(h_dup);
                    }

                    let _ = CloseHandle(h_process);
                }
            }
        }
        Ok(())
    }

    fn launch_process(exe: &PathBuf) -> std::result::Result<u32, String> {
        let wide: Vec<u16> = OsStr::new(exe).encode_wide().chain(Some(0)).collect();

        unsafe {
            let mut si = STARTUPINFOW::default();
            si.cb = mem::size_of::<STARTUPINFOW>() as u32;

            let mut pi = PROCESS_INFORMATION::default();

            CreateProcessW(
                PCWSTR::from_raw(wide.as_ptr()),
                PWSTR::null(),
                None,
                None,
                false,
                PROCESS_CREATION_FLAGS::default(),
                None,
                PCWSTR::null(),
                &si,
                &mut pi,
            )
            .map_err(|e| format!("启动进程失败: {}", e))?;

            let pid = pi.dwProcessId;
            let _ = CloseHandle(pi.hThread);
            let _ = CloseHandle(pi.hProcess);

            Ok(pid)
        }
    }

    pub fn kill_process(pid: u32) -> std::result::Result<(), String> {
        unsafe {
            match OpenProcess(PROCESS_TERMINATE, false, pid) {
                Ok(handle) => {
                    let result = TerminateProcess(handle, 0);
                    let _ = CloseHandle(handle);
                    result.map_err(|e| format!("终止进程失败: {}", e))
                }
                Err(e) => Err(format!("打开进程失败: {}", e)),
            }
        }
    }

    pub fn process_exists(pid: u32) -> bool {
        unsafe {
            match OpenProcess(PROCESS_QUERY_INFORMATION, false, pid) {
                Ok(handle) => {
                    let _ = CloseHandle(handle);
                    true
                }
                Err(_) => false,
            }
        }
    }

    #[repr(C)]
    struct SYSTEM_HANDLE_INFORMATION_EX {
        NumberOfHandles: usize,
        Reserved: usize,
        Handles: [SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX; 1],
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX {
        Object: *mut std::ffi::c_void,
        UniqueProcessId: u32,
        HandleValue: usize,
        GrantedAccess: u32,
        CreatorBackTraceIndex: u16,
        ObjectTypeIndex: u16,
        HandleAttributes: u32,
        Reserved: u32,
    }

    const SystemExtendedHandleInformation: i32 = 64;

    #[link(name = "ntdll")]
    extern "system" {
        fn NtQuerySystemInformation(
            SystemInformationClass: i32,
            SystemInformation: *mut std::ffi::c_void,
            SystemInformationLength: u32,
            ReturnLength: *mut u32,
        ) -> i32;
    }
}

#[cfg(target_os = "macos")]
pub mod platform {
    use super::*;
    use std::process::Command;

    pub fn get_default_app_path() -> PathBuf {
        PathBuf::from("/Applications/WeCom.app/Contents/MacOS/WeCom")
    }

    pub async fn spawn_multiple(req: SpawnRequest) -> std::result::Result<SpawnResponse, String> {
        let app_path = req.app_path.unwrap_or(get_default_app_path());

        if !app_path.exists() {
            return Err(format!("应用程序不存在: {:?}", app_path));
        }

        let mut pids = vec![];
        let mut success = 0;
        let mut failed = 0;

        for i in 0..req.count {
            // macOS 上使用 open 命令打开多个实例
            match Command::new("open")
                .arg("-n") // 新实例
                .arg("-a")
                .arg(&app_path)
                .spawn()
            {
                Ok(child) => {
                    pids.push(child.id());
                    success += 1;
                }
                Err(e) => {
                    eprintln!("启动实例 {} 失败: {}", i + 1, e);
                    failed += 1;
                }
            }

            if i < req.count - 1 {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        }

        Ok(SpawnResponse {
            pids,
            success,
            failed,
        })
    }

    pub fn kill_process(pid: u32) -> std::result::Result<(), String> {
        Command::new("kill")
            .arg("-9")
            .arg(pid.to_string())
            .output()
            .map(|_| ())
            .map_err(|e| format!("终止进程失败: {}", e))
    }

    pub fn process_exists(pid: u32) -> bool {
        Command::new("ps")
            .arg("-p")
            .arg(pid.to_string())
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub mod platform {
    use super::*;

    pub fn get_default_app_path() -> PathBuf {
        PathBuf::from("")
    }

    pub async fn spawn_multiple(_req: SpawnRequest) -> std::result::Result<SpawnResponse, String> {
        Err("此平台不支持".to_string())
    }

    pub fn kill_process(_pid: u32) -> std::result::Result<(), String> {
        Err("此平台不支持".to_string())
    }

    pub fn process_exists(_pid: u32) -> bool {
        false
    }
}
