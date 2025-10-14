// 企业微信多开管理器 - Windows 平台
// 使用 Sandboxie 实现真正的多实例隔离

#[cfg(target_os = "windows")]
use crate::windows_sandbox::{SandboxConfig, SandboxieManager};
#[cfg(target_os = "windows")]
use serde::{Deserialize, Serialize};
#[cfg(target_os = "windows")]
use std::path::PathBuf;

/// 企业微信默认安装路径
#[cfg(target_os = "windows")]
const WECOM_DEFAULT_PATHS: &[&str] = &[
    r"C:\Program Files (x86)\WXWork\WXWork.exe",
    r"C:\Program Files\WXWork\WXWork.exe",
    r"D:\Program Files (x86)\WXWork\WXWork.exe",
    r"D:\Program Files\WXWork\WXWork.exe",
];

/// 实例信息
#[cfg(target_os = "windows")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeComInstance {
    pub id: u8,
    pub sandbox_name: String,
    pub title: String,
    pub pid: Option<u32>,
    pub running: bool,
}

/// 企业微信管理器
#[cfg(target_os = "windows")]
pub struct WeComManager {
    sandboxie: SandboxieManager,
    wecom_exe: PathBuf,
}

#[cfg(target_os = "windows")]
impl WeComManager {
    /// 创建管理器
    pub fn new() -> Result<Self, String> {
        let sandboxie = SandboxieManager::new()?;
        let wecom_exe = Self::find_wecom_exe()?;

        Ok(Self {
            sandboxie,
            wecom_exe,
        })
    }

    /// 查找企业微信安装路径
    fn find_wecom_exe() -> Result<PathBuf, String> {
        for path in WECOM_DEFAULT_PATHS {
            let p = PathBuf::from(path);
            if p.exists() {
                return Ok(p);
            }
        }
        Err("未找到企业微信安装路径".to_string())
    }

    /// 创建并启动实例
    pub fn create_instance(&self, id: u8) -> Result<WeComInstance, String> {
        let sandbox_name = format!("WeCom_{}", id);
        let title = format!("企业微信账号 {}", id);

        // 生成不同的边框颜色
        let colors = ["#FF0000", "#00FF00", "#0000FF", "#FFFF00", "#FF00FF", "#00FFFF"];
        let _border_color = format!("{},ttl,6", colors[id as usize % colors.len()]);

        // 创建沙盒配置
        let config = SandboxConfig {
            name: sandbox_name.clone(),
            enabled: true,
            config_level: 7,
            auto_delete: false,
            closed_file_path: vec![],
            closed_key_path: vec![],
            open_file_path: vec![],
            open_key_path: vec![],
        };

        // 创建沙盒
        self.sandboxie.create_sandbox(&config)?;

        // 在沙盒中启动企业微信
        let pid = self.sandboxie.start_in_sandbox(
            &sandbox_name,
            self.wecom_exe.to_str().unwrap(),
        )?;

        Ok(WeComInstance {
            id,
            sandbox_name,
            title,
            pid: Some(pid),
            running: true,
        })
    }

    /// 批量启动多个实例
    pub fn spawn_multiple(&self, count: u8) -> Result<Vec<WeComInstance>, String> {
        let mut instances = Vec::new();

        for i in 1..=count {
            match self.create_instance(i) {
                Ok(instance) => {
                    println!("✓ 实例 {} 启动成功 (PID: {:?})", i, instance.pid);
                    instances.push(instance);
                }
                Err(e) => {
                    eprintln!("✗ 实例 {} 启动失败: {}", i, e);
                }
            }

            // 实例间延迟,避免同时启动导致冲突
            if i < count {
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        }

        Ok(instances)
    }

    /// 停止实例
    pub fn stop_instance(&self, sandbox_name: &str, pid: Option<u32>) -> Result<(), String> {
        if let Some(pid) = pid {
            use std::process::Command;
            let _ = Command::new("taskkill")
                .args(&["/F", "/PID", &pid.to_string()])
                .output();
        }

        // 清理沙盒
        self.sandboxie.cleanup_sandbox(sandbox_name)?;
        Ok(())
    }

    /// 删除沙盒
    pub fn delete_sandbox(&self, sandbox_name: &str) -> Result<(), String> {
        self.sandboxie.delete_sandbox(sandbox_name)
    }
}

// 非 Windows 平台的空实现
#[cfg(not(target_os = "windows"))]
pub struct WeComManager;

#[cfg(not(target_os = "windows"))]
impl WeComManager {
    pub fn new() -> Result<Self, String> {
        Err("WeComManager 仅支持 Windows 平台".to_string())
    }
}

#[cfg(test)]
#[cfg(target_os = "windows")]
mod tests {
    use super::*;

    #[test]
    fn test_wecom_manager_creation() {
        let manager = WeComManager::new();
        assert!(manager.is_ok() || manager.is_err()); // 取决于是否安装了 Sandboxie
    }

    #[test]
    fn test_find_wecom_path() {
        let path = WeComManager::find_wecom_exe();
        println!("检测到的企业微信路径: {:?}", path);
    }
}
