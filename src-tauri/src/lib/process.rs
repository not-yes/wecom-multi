/// 进程管理模块
///
/// 负责企业微信进程的启动、监控和关闭

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use std::collections::HashMap;

#[cfg(windows)]
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::Threading::*,
};

/// 进程信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub started_at: String,
}

/// 全局进程管理器
pub struct ProcessManager {
    processes: Mutex<HashMap<u32, ProcessInfo>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: Mutex::new(HashMap::new()),
        }
    }

    /// 添加进程到管理列表
    pub fn add_process(&self, pid: u32) {
        let mut procs = self.processes.lock().unwrap();
        procs.insert(pid, ProcessInfo {
            pid,
            started_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        });
    }

    /// 移除进程
    pub fn remove_process(&self, pid: u32) {
        let mut procs = self.processes.lock().unwrap();
        procs.remove(&pid);
    }

    /// 获取所有进程
    pub fn list_processes(&self) -> Vec<ProcessInfo> {
        let procs = self.processes.lock().unwrap();
        procs.values().cloned().collect()
    }

    /// 清空所有进程
    pub fn clear(&self) {
        let mut procs = self.processes.lock().unwrap();
        procs.clear();
    }
}

/// 获取企业微信默认安装路径
#[cfg(windows)]
pub fn get_default_wecom_path() -> Option<PathBuf> {
    let possible_paths = vec![
        r"C:\Program Files (x86)\WXWork\WXWork.exe",
        r"C:\Program Files\WXWork\WXWork.exe",
        r"D:\Program Files (x86)\WXWork\WXWork.exe",
        r"D:\Program Files\WXWork\WXWork.exe",
    ];

    for path in possible_paths {
        let p = PathBuf::from(path);
        if p.exists() {
            return Some(p);
        }
    }

    None
}

#[cfg(not(windows))]
pub fn get_default_wecom_path() -> Option<PathBuf> {
    None
}

/// 启动企业微信进程
#[cfg(windows)]
pub fn launch_wecom(path: Option<PathBuf>) -> Result<u32, String> {
    let exe_path = path.or_else(get_default_wecom_path)
        .ok_or_else(|| "未找到企业微信安装路径".to_string())?;

    if !exe_path.exists() {
        return Err(format!("企业微信程序不存在: {:?}", exe_path));
    }

    unsafe {
        let mut si: STARTUPINFOW = std::mem::zeroed();
        si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;

        let mut pi: PROCESS_INFORMATION = std::mem::zeroed();

        let exe_str = exe_path.to_string_lossy().to_string();
        let mut cmd_line: Vec<u16> = exe_str.encode_utf16().chain(std::iter::once(0)).collect();

        let result = CreateProcessW(
            None,
            PWSTR(cmd_line.as_mut_ptr()),
            None,
            None,
            false,
            PROCESS_CREATION_FLAGS(0),
            None,
            None,
            &si,
            &mut pi,
        );

        match result {
            Ok(_) => {
                let pid = pi.dwProcessId;
                let _ = CloseHandle(pi.hProcess);
                let _ = CloseHandle(pi.hThread);
                Ok(pid)
            }
            Err(e) => Err(format!("启动进程失败: {}", e)),
        }
    }
}

#[cfg(not(windows))]
pub fn launch_wecom(_path: Option<PathBuf>) -> Result<u32, String> {
    Err("此功能仅支持 Windows 平台".to_string())
}

/// 关闭指定 PID 的进程
#[cfg(windows)]
pub fn kill_process(pid: u32) -> Result<(), String> {
    unsafe {
        match OpenProcess(PROCESS_TERMINATE, false, pid) {
            Ok(handle) => {
                let result = TerminateProcess(handle, 0);
                let _ = CloseHandle(handle);

                if result.is_ok() {
                    Ok(())
                } else {
                    Err(format!("无法终止进程 {}", pid))
                }
            }
            Err(_) => Err(format!("无法打开进程 {}", pid)),
        }
    }
}

#[cfg(not(windows))]
pub fn kill_process(_pid: u32) -> Result<(), String> {
    Err("此功能仅支持 Windows 平台".to_string())
}

/// 检查进程是否存在
#[cfg(windows)]
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

#[cfg(not(windows))]
pub fn process_exists(_pid: u32) -> bool {
    false
}

// 添加 chrono 依赖到 Cargo.toml
