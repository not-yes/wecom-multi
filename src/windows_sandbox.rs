// Windows 平台 - Sandboxie 集成模块
// 仅在 Windows 平台编译

#[cfg(target_os = "windows")]
use std::path::PathBuf;
#[cfg(target_os = "windows")]
use std::process::Command;
#[cfg(target_os = "windows")]
use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub name: String,
    pub enabled: bool,
    pub config_level: u8,
    pub auto_delete: bool,
    pub closed_file_path: Vec<String>,
    pub closed_key_path: Vec<String>,
    pub open_file_path: Vec<String>,
    pub open_key_path: Vec<String>,
}

#[cfg(target_os = "windows")]
impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            name: "DefaultBox".to_string(),
            enabled: true,
            config_level: 7,
            auto_delete: false,
            closed_file_path: vec![],
            closed_key_path: vec![],
            open_file_path: vec![],
            open_key_path: vec![],
        }
    }
}

#[cfg(target_os = "windows")]
pub struct SandboxieManager {
    sbieini_path: PathBuf,
    start_path: PathBuf,
}

#[cfg(target_os = "windows")]
impl SandboxieManager {
    /// 创建 Sandboxie 管理器
    pub fn new() -> Result<Self, String> {
        // 检测 Sandboxie-Plus 安装路径
        let possible_paths = vec![
            r"C:\Program Files\Sandboxie-Plus\SbieIni.exe",
            r"C:\Program Files (x86)\Sandboxie-Plus\SbieIni.exe",
        ];

        let mut sbieini_path = None;
        for path in possible_paths {
            let p = PathBuf::from(path);
            if p.exists() {
                sbieini_path = Some(p);
                break;
            }
        }

        let sbieini_path = sbieini_path
            .ok_or_else(|| "未找到 Sandboxie-Plus 安装,请先安装 Sandboxie-Plus".to_string())?;

        let start_path = sbieini_path
            .parent()
            .ok_or_else(|| "无法获取 Sandboxie 目录".to_string())?
            .join("Start.exe");

        if !start_path.exists() {
            return Err("未找到 Start.exe".to_string());
        }

        Ok(Self {
            sbieini_path,
            start_path,
        })
    }

    /// 执行 SbieIni 命令
    fn run_sbieini(&self, args: &[&str]) -> Result<String, String> {
        let output = Command::new(&self.sbieini_path)
            .args(args)
            .output()
            .map_err(|e| format!("执行 SbieIni 失败: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("SbieIni 命令失败: {}", stderr));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// 创建沙盒
    pub fn create_sandbox(&self, config: &SandboxConfig) -> Result<(), String> {
        println!("正在创建沙盒: {}", config.name);

        // 设置基本配置
        self.run_sbieini(&["set", &config.name, "ConfigLevel", &config.config_level.to_string()])?;
        self.run_sbieini(&["set", &config.name, "Enabled", if config.enabled { "y" } else { "n" }])?;

        // 设置自动删除
        if config.auto_delete {
            self.run_sbieini(&["set", &config.name, "AutoDelete", "y"])?;
        }

        // 设置文件路径隔离
        for path in &config.closed_file_path {
            self.run_sbieini(&["append", &config.name, "ClosedFilePath", path])?;
        }

        for path in &config.open_file_path {
            self.run_sbieini(&["append", &config.name, "OpenFilePath", path])?;
        }

        // 设置注册表隔离
        for path in &config.closed_key_path {
            self.run_sbieini(&["append", &config.name, "ClosedKeyPath", path])?;
        }

        for path in &config.open_key_path {
            self.run_sbieini(&["append", &config.name, "OpenKeyPath", path])?;
        }

        // 重载配置
        self.run_sbieini(&["reload"])?;

        println!("✓ 沙盒 {} 创建成功", config.name);
        Ok(())
    }

    /// 删除沙盒
    pub fn delete_sandbox(&self, sandbox_name: &str) -> Result<(), String> {
        println!("正在删除沙盒: {}", sandbox_name);
        self.run_sbieini(&["delete_sandbox", sandbox_name])?;
        self.run_sbieini(&["reload"])?;
        println!("✓ 沙盒 {} 删除成功", sandbox_name);
        Ok(())
    }

    /// 在沙盒中启动程序
    pub fn start_in_sandbox(&self, sandbox_name: &str, exe_path: &str) -> Result<u32, String> {
        println!("正在沙盒 {} 中启动: {}", sandbox_name, exe_path);

        let child = Command::new(&self.start_path)
            .arg(format!("/box:{}", sandbox_name))
            .arg(exe_path)
            .spawn()
            .map_err(|e| format!("启动程序失败: {}", e))?;

        let pid = child.id();
        println!("✓ 程序已在沙盒中启动 (PID: {})", pid);
        Ok(pid)
    }

    /// 列出所有沙盒
    pub fn list_sandboxes(&self) -> Result<Vec<String>, String> {
        let output = self.run_sbieini(&["enum_sandboxes"])?;
        let sandboxes: Vec<String> = output
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect();
        Ok(sandboxes)
    }

    /// 检查沙盒是否存在
    pub fn sandbox_exists(&self, sandbox_name: &str) -> Result<bool, String> {
        let sandboxes = self.list_sandboxes()?;
        Ok(sandboxes.iter().any(|s| s == sandbox_name))
    }

    /// 清理沙盒内容(但保留沙盒配置)
    pub fn cleanup_sandbox(&self, sandbox_name: &str) -> Result<(), String> {
        println!("正在清理沙盒: {}", sandbox_name);

        // 终止沙盒中的所有进程
        let _ = Command::new(&self.start_path)
            .arg(format!("/box:{}", sandbox_name))
            .arg("/terminate")
            .output();

        // 删除沙盒内容
        self.run_sbieini(&["delete_sandbox_silent", sandbox_name])?;

        println!("✓ 沙盒 {} 清理成功", sandbox_name);
        Ok(())
    }
}

#[cfg(test)]
#[cfg(target_os = "windows")]
mod tests {
    use super::*;

    #[test]
    fn test_sandboxie_manager_creation() {
        let manager = SandboxieManager::new();
        assert!(manager.is_ok(), "应该能找到 Sandboxie-Plus");
    }

    #[test]
    fn test_sandbox_config_default() {
        let config = SandboxConfig::default();
        assert_eq!(config.name, "DefaultBox");
        assert_eq!(config.enabled, true);
        assert_eq!(config.config_level, 7);
    }
}

// 非 Windows 平台的空实现
#[cfg(not(target_os = "windows"))]
pub struct SandboxieManager;

#[cfg(not(target_os = "windows"))]
impl SandboxieManager {
    pub fn new() -> Result<Self, String> {
        Err("Sandboxie 仅支持 Windows 平台".to_string())
    }
}
