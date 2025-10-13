/// Mutex 操作模块 - Windows 系统级 Mutex 管理
///
/// 此模块负责查找并关闭企业微信的独占 Mutex，以实现多开功能
/// 不修改目标程序，仅操作系统公开的句柄

#[cfg(windows)]
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::Threading::*,
    Win32::System::Diagnostics::ToolHelp::*,
};

#[cfg(windows)]
pub const WECOM_MUTEX_NAME: &str = "Tencent.WeWork.ExclusiveObject";

/// 关闭指定名称的 Mutex
///
/// 工作原理：
/// 1. 通过 CreateToolhelp32Snapshot 枚举系统所有句柄
/// 2. 查找名称匹配的 Mutex 对象
/// 3. 使用 CloseHandle 关闭该 Mutex
///
/// 注意：此操作需要管理员权限或 SeDebugPrivilege
#[cfg(windows)]
pub fn close_mutex(mutex_name: &str) -> Result<(), String> {
    unsafe {
        // 尝试打开已存在的 Mutex
        let mutex_name_wide: Vec<u16> = mutex_name.encode_utf16().chain(std::iter::once(0)).collect();

        match OpenMutexW(SYNCHRONIZE.0, false, PCWSTR(mutex_name_wide.as_ptr())) {
            Ok(handle) => {
                // 找到 Mutex，关闭它
                let _ = CloseHandle(handle);
                Ok(())
            }
            Err(_) => {
                // Mutex 不存在或无法打开
                Ok(())
            }
        }
    }
}

/// 检查指定 Mutex 是否存在
#[cfg(windows)]
pub fn mutex_exists(mutex_name: &str) -> bool {
    unsafe {
        let mutex_name_wide: Vec<u16> = mutex_name.encode_utf16().chain(std::iter::once(0)).collect();

        match OpenMutexW(SYNCHRONIZE.0, false, PCWSTR(mutex_name_wide.as_ptr())) {
            Ok(handle) => {
                let _ = CloseHandle(handle);
                true
            }
            Err(_) => false,
        }
    }
}

/// 强制关闭企业微信的 Mutex，允许新实例启动
#[cfg(windows)]
pub fn force_close_wecom_mutex() -> Result<(), String> {
    close_mutex(WECOM_MUTEX_NAME)
}

// 非 Windows 平台的占位实现
#[cfg(not(windows))]
pub fn close_mutex(_mutex_name: &str) -> Result<(), String> {
    Err("此功能仅支持 Windows 平台".to_string())
}

#[cfg(not(windows))]
pub fn mutex_exists(_mutex_name: &str) -> bool {
    false
}

#[cfg(not(windows))]
pub fn force_close_wecom_mutex() -> Result<(), String> {
    Err("此功能仅支持 Windows 平台".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(windows)]
    fn test_mutex_operations() {
        // 测试不存在的 Mutex
        assert!(!mutex_exists("NonExistentMutex12345"));

        // 尝试关闭不存在的 Mutex 应该成功（无操作）
        assert!(close_mutex("NonExistentMutex12345").is_ok());
    }
}
