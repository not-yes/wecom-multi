// 核心库 - 跨平台 Mutex 管理和进程启动
// 支持 Windows 和 macOS

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Windows 特定模块
#[cfg(target_os = "windows")]
pub mod windows_sandbox;

#[cfg(target_os = "windows")]
pub mod wecom_manager;

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
    };

    const WECOM_MUTEX_NAME: &str = "Tencent.WeWork.Exclusive"; // 匹配所有相关 Mutex

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
            match close_mutex(WECOM_MUTEX_NAME) {
                Ok(_) => {
                    println!("✓ 成功关闭 Mutex,准备启动实例 {}", i + 1);
                }
                Err(e) => {
                    // 如果是第一个实例,mutex 不存在是正常的
                    if i == 0 {
                        println!("⚠ 未找到 Mutex (可能是首次启动): {}", e);
                    } else {
                        eprintln!("✗ 关闭 Mutex 失败: {}", e);
                    }
                }
            }

            tokio::time::sleep(Duration::from_millis(100)).await;

            // 启动进程
            match launch_process(&exe) {
                Ok(pid) => {
                    pids.push(pid);
                    success += 1;
                    println!("✓ 实例 {} 启动成功 (PID: {})", i + 1, pid);
                }
                Err(e) => {
                    eprintln!("✗ 启动实例 {} 失败: {}", i + 1, e);
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

    fn close_mutex(name: &str) -> std::result::Result<(), String> {
        unsafe {
            let h_current = GetCurrentProcess();

            // 使用足够大的固定缓冲区 (4MB,足以容纳大部分系统的句柄信息)
            let buf_size = 4 * 1024 * 1024; // 4MB
            let mut buf = vec![0u8; buf_size];
            let mut ret_len = 0u32;

            let status = NtQuerySystemInformation(
                SystemExtendedHandleInformation,
                buf.as_mut_ptr() as _,
                buf.len() as u32,
                &mut ret_len,
            );

            if status != 0 {
                return Err(format!("查询系统信息失败: status=0x{:X} (需要 {} 字节,分配了 {} 字节)",
                    status, ret_len, buf_size));
            }

            let info = &*(buf.as_ptr() as *const SYSTEM_HANDLE_INFORMATION_EX);
            let handles = slice::from_raw_parts(info.Handles.as_ptr(), info.NumberOfHandles as usize);

            let target_name = name.to_lowercase();
            let mut closed_count = 0;
            let mut mutex_count = 0;
            let mut checked_count = 0;

            eprintln!("[调试] 开始扫描系统句柄,总数: {}", info.NumberOfHandles);

            // 收集所有 ObjectTypeIndex 的统计
            let mut type_counts: std::collections::HashMap<u16, usize> = std::collections::HashMap::new();

            for h in handles {
                // 跳过当前进程的句柄
                if h.UniqueProcessId == GetCurrentProcessId() {
                    continue;
                }

                // 统计类型分布
                *type_counts.entry(h.ObjectTypeIndex).or_insert(0) += 1;

                // 尝试多个可能的 ObjectTypeIndex (不同 Windows 版本可能不同)
                // 17 = Mutant (常见), 18, 19 也可能是
                if h.ObjectTypeIndex < 15 || h.ObjectTypeIndex > 25 {
                    continue;
                }

                mutex_count += 1;

                if let Ok(h_process) = OpenProcess(PROCESS_DUP_HANDLE, false, h.UniqueProcessId) {
                    let mut h_dup = HANDLE::default();

                    // 复制句柄到当前进程
                    if DuplicateHandle(
                        h_process,
                        HANDLE(h.HandleValue as _),
                        h_current,
                        &mut h_dup,
                        0,
                        false,
                        DUPLICATE_HANDLE_OPTIONS(0),
                    )
                    .is_ok()
                    {
                        // 查询对象名称
                        if let Some(obj_name) = query_object_name(h_dup) {
                            checked_count += 1;

                            // 检查名称是否匹配目标 mutex
                            if obj_name.to_lowercase().contains(&target_name) {
                                eprintln!("[调试] 找到目标 Mutex: {} (PID: {})", obj_name, h.UniqueProcessId);

                                // 关闭源进程中的句柄
                                let mut h_temp = HANDLE::default();
                                if DuplicateHandle(
                                    h_process,
                                    HANDLE(h.HandleValue as _),
                                    h_current,
                                    &mut h_temp,
                                    0,
                                    false,
                                    DUPLICATE_CLOSE_SOURCE,
                                )
                                .is_ok()
                                {
                                    let _ = CloseHandle(h_temp);
                                    closed_count += 1;
                                    eprintln!("[调试] 已关闭 Mutex");
                                }
                            }
                        }

                        let _ = CloseHandle(h_dup);
                    }

                    let _ = CloseHandle(h_process);
                }
            }

            eprintln!("[调试] 扫描完成 - Mutex类型句柄: {}, 成功查询名称: {}, 关闭数量: {}",
                mutex_count, checked_count, closed_count);

            // 输出类型统计的前 10 个
            let mut sorted_types: Vec<_> = type_counts.iter().collect();
            sorted_types.sort_by(|a, b| b.1.cmp(a.1));
            eprintln!("[调试] ObjectTypeIndex 统计 (前10):");
            for (idx, (type_id, count)) in sorted_types.iter().take(10).enumerate() {
                eprintln!("  {}. Type {}: {} 个句柄", idx + 1, type_id, count);
            }

            if closed_count > 0 {
                Ok(())
            } else {
                Err(format!("未找到名为 '{}' 的 Mutex (扫描了 {} 个可能的 Mutex 句柄,成功查询 {} 个名称)",
                    name, mutex_count, checked_count))
            }
        }
    }

    // 查询对象名称
    fn query_object_name(handle: HANDLE) -> Option<String> {
        unsafe {
            let mut buffer = vec![0u8; 4096];
            let mut ret_len = 0u32;

            let status = NtQueryObject(
                handle,
                ObjectNameInformation,
                buffer.as_mut_ptr() as _,
                buffer.len() as u32,
                &mut ret_len,
            );

            if status != 0 {
                return None;
            }

            let name_info = &*(buffer.as_ptr() as *const UNICODE_STRING);
            if name_info.Length == 0 || name_info.Buffer.is_null() {
                return None;
            }

            let name_slice = slice::from_raw_parts(
                name_info.Buffer.as_ptr(),
                (name_info.Length / 2) as usize,
            );

            String::from_utf16(name_slice).ok()
        }
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

    /// 查找所有正在运行的企业微信进程
    pub fn find_wecom_processes() -> Vec<u32> {
        use windows::Win32::System::ProcessStatus::*;

        let mut pids = Vec::new();

        unsafe {
            // 枚举所有进程
            let mut process_ids = vec![0u32; 2048];
            let mut bytes_returned = 0u32;

            if EnumProcesses(
                process_ids.as_mut_ptr(),
                (process_ids.len() * std::mem::size_of::<u32>()) as u32,
                &mut bytes_returned,
            )
            .is_ok()
            {
                let count = bytes_returned as usize / std::mem::size_of::<u32>();

                for &pid in &process_ids[..count] {
                    if pid == 0 {
                        continue;
                    }

                    // 打开进程获取更多信息
                    if let Ok(h_process) = OpenProcess(
                        PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                        false,
                        pid,
                    ) {
                        // 获取进程可执行文件路径
                        let mut exe_path = vec![0u16; 260];
                        let mut size = exe_path.len() as u32;

                        if QueryFullProcessImageNameW(
                            h_process,
                            PROCESS_NAME_WIN32,
                            PWSTR(exe_path.as_mut_ptr()),
                            &mut size
                        )
                        .is_ok()
                        {
                            let path = String::from_utf16_lossy(&exe_path[..size as usize]);

                            // 检查是否是企业微信进程
                            if path.to_lowercase().contains("wxwork.exe") {
                                pids.push(pid);
                            }
                        }

                        let _ = CloseHandle(h_process);
                    }
                }
            }
        }

        pids
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

    #[repr(C)]
    struct UNICODE_STRING {
        Length: u16,
        MaximumLength: u16,
        Buffer: PWSTR,
    }

    const SystemExtendedHandleInformation: i32 = 64;
    const ObjectNameInformation: i32 = 1;

    #[link(name = "ntdll")]
    extern "system" {
        fn NtQuerySystemInformation(
            SystemInformationClass: i32,
            SystemInformation: *mut std::ffi::c_void,
            SystemInformationLength: u32,
            ReturnLength: *mut u32,
        ) -> i32;

        fn NtQueryObject(
            Handle: HANDLE,
            ObjectInformationClass: i32,
            ObjectInformation: *mut std::ffi::c_void,
            ObjectInformationLength: u32,
            ReturnLength: *mut u32,
        ) -> i32;
    }
}

#[cfg(target_os = "macos")]
pub mod platform {
    use super::*;
    use std::process::Command;
    use std::fs;

    pub fn get_default_app_path() -> PathBuf {
        // 尝试多个可能的路径
        let possible_paths = vec![
            "/Applications/企业微信.app",
            "/Applications/WeCom.app",
        ];

        for path in possible_paths {
            let p = PathBuf::from(path);
            if p.exists() {
                return p;
            }
        }

        // 默认返回中文版路径
        PathBuf::from("/Applications/企业微信.app")
    }

    fn get_instances_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        PathBuf::from(format!("{}/Applications/WeComMulti", home))
    }

    fn create_app_instance(source_app: &PathBuf, instance_id: u8) -> std::result::Result<PathBuf, String> {
        let instances_dir = get_instances_dir();

        // 创建实例目录
        fs::create_dir_all(&instances_dir)
            .map_err(|e| format!("创建实例目录失败: {}", e))?;

        // 获取应用名称
        let app_name = source_app
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("无法获取应用名称")?;

        // 创建实例路径
        let instance_path = instances_dir.join(format!("{}{}.app", app_name, instance_id));

        // 如果实例已存在,先删除
        if instance_path.exists() {
            fs::remove_dir_all(&instance_path)
                .map_err(|e| format!("删除旧实例失败: {}", e))?;
        }

        println!("正在克隆应用到: {}", instance_path.display());

        // 复制应用
        let status = Command::new("cp")
            .arg("-R")
            .arg(source_app)
            .arg(&instance_path)
            .status()
            .map_err(|e| format!("复制应用失败: {}", e))?;

        if !status.success() {
            return Err(format!("复制应用失败,退出码: {}", status.code().unwrap_or(-1)));
        }

        // 修改 Info.plist 中的 Bundle ID
        let plist_path = instance_path.join("Contents/Info.plist");
        let new_bundle_id = format!("com.tencent.WeWorkMac.instance{}", instance_id);

        println!("正在修改 Bundle ID: {}", new_bundle_id);

        let status = Command::new("/usr/libexec/PlistBuddy")
            .arg("-c")
            .arg(format!("Set :CFBundleIdentifier {}", new_bundle_id))
            .arg(&plist_path)
            .status()
            .map_err(|e| format!("修改 Bundle ID 失败: {}", e))?;

        if !status.success() {
            return Err("修改 Bundle ID 失败".to_string());
        }

        // 清除隔离属性
        println!("正在清除隔离属性...");
        let _ = Command::new("/usr/bin/xattr")
            .arg("-rc")
            .arg(&instance_path)
            .status();

        // 重新签名
        println!("正在重新签名...");
        let _ = Command::new("codesign")
            .arg("--force")
            .arg("--deep")
            .arg("--sign")
            .arg("-")
            .arg("--timestamp=none")
            .arg(&instance_path)
            .output();

        Ok(instance_path)
    }

    pub async fn spawn_multiple(req: SpawnRequest) -> std::result::Result<SpawnResponse, String> {
        let source_app = req.app_path.unwrap_or(get_default_app_path());

        if !source_app.exists() {
            return Err(format!("应用程序不存在: {:?}", source_app));
        }

        let mut pids = vec![];
        let mut success = 0;
        let mut failed = 0;

        for i in 0..req.count {
            // 为每个实例创建独立的应用副本
            match create_app_instance(&source_app, i + 1) {
                Ok(instance_path) => {
                    // 启动实例 - 使用直接启动可执行文件的方式,更稳定
                    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

                    // 为每个实例创建独立的数据目录
                    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
                    let instance_home = PathBuf::from(format!("{}/Library/Containers/WeComInstance{}", home, i + 1));

                    // 创建实例专用的数据目录
                    if !instance_home.exists() {
                        let _ = fs::create_dir_all(&instance_home);
                        println!("创建实例 {} 数据目录: {}", i + 1, instance_home.display());
                    }

                    // 获取可执行文件路径
                    let executable = instance_path.join("Contents/MacOS");
                    let app_name = source_app
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("WeChat");
                    let executable_path = executable.join(app_name);

                    // 尝试两种启动方式
                    // 方式1: 直接启动可执行文件 (推荐,更稳定)
                    // 设置多个环境变量,尽可能让每个实例使用独立的数据目录

                    // 创建 Documents 目录
                    let instance_documents = instance_home.join("Documents");
                    let _ = fs::create_dir_all(&instance_documents);

                    let launch_result = Command::new(&executable_path)
                        .env("HOME", &instance_home)
                        .env("TMPDIR", format!("{}/tmp", instance_home.display()))
                        .env("XDG_CONFIG_HOME", format!("{}/config", instance_home.display()))
                        .env("XDG_DATA_HOME", format!("{}/data", instance_home.display()))
                        .env("XDG_CACHE_HOME", format!("{}/cache", instance_home.display()))
                        .spawn();

                    match launch_result {
                        Ok(child) => {
                            let pid = child.id();
                            pids.push(pid);
                            success += 1;
                            println!("✓ 实例 {} 启动成功 (PID: {}, 数据目录: {})", i + 1, pid, instance_home.display());
                        }
                        Err(e) => {
                            // 方式2: 如果直接启动失败,尝试使用 open -n
                            eprintln!("直接启动失败,尝试使用 open 命令: {}", e);
                            match Command::new("open")
                                .arg("-n")
                                .arg(&instance_path)
                                .spawn()
                            {
                                Ok(child) => {
                                    pids.push(child.id());
                                    success += 1;
                                    println!("✓ 实例 {} 使用 open 命令启动成功: {}", i + 1, instance_path.display());
                                }
                                Err(e2) => {
                                    eprintln!("✗ 启动实例 {} 失败: {}", i + 1, e2);
                                    failed += 1;
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("✗ 创建实例 {} 失败: {}", i + 1, e);
                    failed += 1;
                }
            }

            if i < req.count - 1 {
                tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
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
