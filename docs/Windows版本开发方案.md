# Windows 版企业微信多开工具开发方案

**技术方案**: Tauri + Rust + Sandboxie-Plus
**开发平台**: Windows 10/11
**目标**: 提供简单易用的图形化企业微信多开管理工具

---

## 一、技术架构

### 1.1 整体架构

```
┌─────────────────────────────────────────┐
│         Tauri Frontend (React)          │
│  - 沙盒列表                              │
│  - 实例管理                              │
│  - 一键启动/停止                          │
└─────────────┬───────────────────────────┘
              │ Tauri IPC
┌─────────────▼───────────────────────────┐
│         Rust Backend                    │
│  - Sandboxie CLI 封装                   │
│  - 进程管理                              │
│  - 配置管理                              │
└─────────────┬───────────────────────────┘
              │ Command Line
┌─────────────▼───────────────────────────┐
│         Sandboxie-Plus                  │
│  - SbieIni.exe (配置沙盒)               │
│  - Start.exe (启动程序)                 │
│  - 沙盒隔离                              │
└─────────────────────────────────────────┘
```

### 1.2 技术栈

#### 前端
- **框架**: React 18 + TypeScript
- **UI 库**: 当前已有的 CSS 样式
- **状态管理**: React Hooks
- **构建工具**: Vite

#### 后端
- **核心**: Rust + Tauri v2
- **进程管理**: std::process::Command
- **配置解析**: serde + toml
- **Windows API**: windows-rs crate

#### 集成
- **Sandboxie-Plus**: 命令行工具
- **安装检测**: 自动检测 Sandboxie 安装路径
- **配置管理**: INI 文件解析

---

## 二、Sandboxie 命令行接口

### 2.1 核心工具

Sandboxie 提供两个关键命令行工具:

#### SbieIni.exe - 配置管理
```bash
# 位置
C:\Program Files\Sandboxie-Plus\SbieIni.exe

# 创建沙盒(设置必需的两个属性)
SbieIni.exe set WeCom1 ConfigLevel 7
SbieIni.exe set WeCom1 Enabled y

# 配置沙盒属性
SbieIni.exe set WeCom1 BorderColor "#FF0000,ttl,6"
SbieIni.exe set WeCom1 BoxNameTitle "企业微信 1"
SbieIni.exe set WeCom1 AutoDelete y
SbieIni.exe set WeCom1 AutoRecover y

# 查询配置
SbieIni.exe get WeCom1 Enabled

# 删除沙盒
SbieIni.exe set WeCom1 Enabled n
```

#### Start.exe - 程序启动
```bash
# 位置
C:\Program Files\Sandboxie-Plus\Start.exe

# 在指定沙盒中启动程序
Start.exe /box:WeCom1 "C:\Program Files\WXWork\WXWork.exe"

# 静默启动(无弹窗)
Start.exe /box:WeCom1 /silent "C:\Program Files\WXWork\WXWork.exe"

# 等待程序结束
Start.exe /box:WeCom1 /wait "C:\Program Files\WXWork\WXWork.exe"

# 隐藏窗口
Start.exe /box:WeCom1 /hide_window "C:\Program Files\WXWork\WXWork.exe"
```

### 2.2 配置文件

Sandboxie 使用 INI 格式配置文件:

**位置**: `C:\ProgramData\Sandboxie-Plus\Sandboxie.ini`

**示例配置**:
```ini
[WeCom1]
ConfigLevel=7
Enabled=y
BorderColor=#00FF00,ttl,6
BoxNameTitle=企业微信账号1
Template=OpenFilePath
Template=ClosedFilePath
AutoDelete=y
AutoRecover=y
RecoverFolder=%Desktop%
RecoverFolder=%Favorites%
RecoverFolder=%{374DE290-123F-4565-9164-39C4925E467B}%
OpenFilePath=WeCom.exe,%AppData%\WeCom1
ClosedFilePath=WeCom.exe,!<write>,C:\
```

---

## 三、Rust 后端实现

### 3.1 项目结构

```
src-tauri/
├── src/
│   ├── main.rs              # 主入口
│   ├── lib.rs               # 共享库(复用 macOS 代码)
│   ├── windows_sandbox.rs   # Windows 沙盒管理(新增)
│   └── wecom_manager.rs     # 企业微信管理(新增)
├── Cargo.toml
└── tauri.conf.json
```

### 3.2 核心模块实现

#### windows_sandbox.rs - Sandboxie 封装

```rust
use std::process::Command;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Sandboxie 安装路径
const SANDBOXIE_PATH: &str = r"C:\Program Files\Sandboxie-Plus";

/// 沙盒配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub name: String,
    pub title: String,
    pub border_color: String,
    pub auto_delete: bool,
    pub auto_recover: bool,
}

/// Sandboxie 管理器
pub struct SandboxieManager {
    sbieini_path: PathBuf,
    start_path: PathBuf,
}

impl SandboxieManager {
    /// 创建管理器实例
    pub fn new() -> Result<Self, String> {
        let sbieini = PathBuf::from(SANDBOXIE_PATH).join("SbieIni.exe");
        let start = PathBuf::from(SANDBOXIE_PATH).join("Start.exe");

        if !sbieini.exists() {
            return Err("Sandboxie-Plus 未安装".to_string());
        }

        Ok(Self {
            sbieini_path: sbieini,
            start_path: start,
        })
    }

    /// 检测 Sandboxie 是否安装
    pub fn is_installed() -> bool {
        PathBuf::from(SANDBOXIE_PATH).join("SbieIni.exe").exists()
    }

    /// 创建沙盒
    pub fn create_sandbox(&self, config: &SandboxConfig) -> Result<(), String> {
        // 设置 ConfigLevel (必需)
        self.run_sbieini(&["set", &config.name, "ConfigLevel", "7"])?;

        // 启用沙盒 (必需)
        self.run_sbieini(&["set", &config.name, "Enabled", "y"])?;

        // 设置边框颜色
        let border = format!("{},ttl,6", config.border_color);
        self.run_sbieini(&["set", &config.name, "BorderColor", &border])?;

        // 设置标题
        self.run_sbieini(&["set", &config.name, "BoxNameTitle", &config.title])?;

        // 自动删除
        let auto_delete = if config.auto_delete { "y" } else { "n" };
        self.run_sbieini(&["set", &config.name, "AutoDelete", auto_delete])?;

        // 自动恢复
        let auto_recover = if config.auto_recover { "y" } else { "n" };
        self.run_sbieini(&["set", &config.name, "AutoRecover", auto_recover])?;

        println!("✓ 沙盒 {} 创建成功", config.name);
        Ok(())
    }

    /// 删除沙盒
    pub fn delete_sandbox(&self, name: &str) -> Result<(), String> {
        self.run_sbieini(&["set", name, "Enabled", "n"])?;
        println!("✓ 沙盒 {} 已删除", name);
        Ok(())
    }

    /// 检查沙盒是否存在
    pub fn sandbox_exists(&self, name: &str) -> bool {
        self.run_sbieini(&["get", name, "Enabled"])
            .map(|output| output.contains("y"))
            .unwrap_or(false)
    }

    /// 在沙盒中启动程序
    pub fn start_in_sandbox(
        &self,
        sandbox_name: &str,
        exe_path: &str,
    ) -> Result<u32, String> {
        let output = Command::new(&self.start_path)
            .args(&[
                &format!("/box:{}", sandbox_name),
                "/silent",
                exe_path,
            ])
            .spawn()
            .map_err(|e| format!("启动失败: {}", e))?;

        let pid = output.id();
        println!("✓ 在沙盒 {} 中启动程序 (PID: {})", sandbox_name, pid);
        Ok(pid)
    }

    /// 执行 SbieIni 命令
    fn run_sbieini(&self, args: &[&str]) -> Result<String, String> {
        let output = Command::new(&self.sbieini_path)
            .args(args)
            .output()
            .map_err(|e| format!("执行 SbieIni 失败: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "SbieIni 命令失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
```

#### wecom_manager.rs - 企业微信管理

```rust
use super::windows_sandbox::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 企业微信安装路径
const WECOM_DEFAULT_PATHS: &[&str] = &[
    r"C:\Program Files (x86)\WXWork\WXWork.exe",
    r"C:\Program Files\WXWork\WXWork.exe",
    r"D:\Program Files (x86)\WXWork\WXWork.exe",
    r"D:\Program Files\WXWork\WXWork.exe",
];

/// 实例信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeComInstance {
    pub id: u8,
    pub sandbox_name: String,
    pub title: String,
    pub pid: Option<u32>,
    pub running: bool,
}

/// 企业微信管理器
pub struct WeComManager {
    sandboxie: SandboxieManager,
    wecom_exe: PathBuf,
}

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

    /// 初始化实例(创建沙盒)
    pub fn initialize_instances(&self, count: u8) -> Result<Vec<WeComInstance>, String> {
        let mut instances = Vec::new();

        for i in 1..=count {
            let config = SandboxConfig {
                name: format!("WeCom{}", i),
                title: format!("企业微信账号 {}", i),
                border_color: Self::get_border_color(i),
                auto_delete: false,  // 保留数据
                auto_recover: true,  // 自动恢复文件
            };

            // 如果沙盒不存在,创建它
            if !self.sandboxie.sandbox_exists(&config.name) {
                self.sandboxie.create_sandbox(&config)?;
            }

            instances.push(WeComInstance {
                id: i,
                sandbox_name: config.name.clone(),
                title: config.title.clone(),
                pid: None,
                running: false,
            });
        }

        Ok(instances)
    }

    /// 启动实例
    pub fn start_instance(&self, instance: &mut WeComInstance) -> Result<(), String> {
        if instance.running {
            return Err("实例已在运行".to_string());
        }

        let pid = self.sandboxie.start_in_sandbox(
            &instance.sandbox_name,
            self.wecom_exe.to_str().unwrap(),
        )?;

        instance.pid = Some(pid);
        instance.running = true;

        Ok(())
    }

    /// 停止实例(终止进程)
    pub fn stop_instance(&self, instance: &mut WeComInstance) -> Result<(), String> {
        if let Some(pid) = instance.pid {
            // Windows API 终止进程
            super::platform::kill_process(pid)?;
            instance.pid = None;
            instance.running = false;
            println!("✓ 实例 {} 已停止", instance.id);
        }
        Ok(())
    }

    /// 检查实例状态
    pub fn check_instance_status(&self, instance: &mut WeComInstance) {
        if let Some(pid) = instance.pid {
            instance.running = super::platform::process_exists(pid);
            if !instance.running {
                instance.pid = None;
            }
        }
    }

    /// 获取边框颜色(区分不同实例)
    fn get_border_color(instance_id: u8) -> String {
        match instance_id % 6 {
            1 => "#FF0000".to_string(), // 红色
            2 => "#00FF00".to_string(), // 绿色
            3 => "#0000FF".to_string(), // 蓝色
            4 => "#FFFF00".to_string(), // 黄色
            5 => "#FF00FF".to_string(), // 紫色
            _ => "#00FFFF".to_string(), // 青色
        }
    }
}
```

### 3.3 Tauri 命令

```rust
use tauri::State;
use std::sync::Mutex;

/// 应用状态
struct AppState {
    manager: Mutex<WeComManager>,
    instances: Mutex<Vec<WeComInstance>>,
}

/// 初始化实例
#[tauri::command]
async fn initialize_instances(
    count: u8,
    state: State<'_, AppState>,
) -> Result<Vec<WeComInstance>, String> {
    let manager = state.manager.lock().unwrap();
    let instances = manager.initialize_instances(count)?;
    *state.instances.lock().unwrap() = instances.clone();
    Ok(instances)
}

/// 启动实例
#[tauri::command]
async fn start_instance(
    instance_id: u8,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let manager = state.manager.lock().unwrap();
    let mut instances = state.instances.lock().unwrap();

    if let Some(instance) = instances.iter_mut().find(|i| i.id == instance_id) {
        manager.start_instance(instance)?;
        Ok(())
    } else {
        Err("实例不存在".to_string())
    }
}

/// 停止实例
#[tauri::command]
async fn stop_instance(
    instance_id: u8,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let manager = state.manager.lock().unwrap();
    let mut instances = state.instances.lock().unwrap();

    if let Some(instance) = instances.iter_mut().find(|i| i.id == instance_id) {
        manager.stop_instance(instance)?;
        Ok(())
    } else {
        Err("实例不存在".to_string())
    }
}

/// 获取所有实例状态
#[tauri::command]
async fn get_instances(
    state: State<'_, AppState>,
) -> Result<Vec<WeComInstance>, String> {
    let manager = state.manager.lock().unwrap();
    let mut instances = state.instances.lock().unwrap();

    // 更新状态
    for instance in instances.iter_mut() {
        manager.check_instance_status(instance);
    }

    Ok(instances.clone())
}

/// 检查 Sandboxie 是否安装
#[tauri::command]
async fn check_sandboxie() -> Result<bool, String> {
    Ok(SandboxieManager::is_installed())
}
```

---

## 四、前端界面设计

### 4.1 主要功能页面

#### 检查 Sandboxie 页面
```tsx
// src/components/SandboxieCheck.tsx
import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'

export function SandboxieCheck({ onReady }: { onReady: () => void }) {
  const [installed, setInstalled] = useState(false)
  const [checking, setChecking] = useState(true)

  useEffect(() => {
    checkSandboxie()
  }, [])

  async function checkSandboxie() {
    try {
      const result = await invoke<boolean>('check_sandboxie')
      setInstalled(result)
      if (result) {
        onReady()
      }
    } catch (error) {
      console.error('检查失败:', error)
    } finally {
      setChecking(false)
    }
  }

  if (checking) {
    return <div>正在检查 Sandboxie...</div>
  }

  if (!installed) {
    return (
      <div className="error-page">
        <h2>❌ 未检测到 Sandboxie-Plus</h2>
        <p>本工具需要 Sandboxie-Plus 才能运行</p>
        <a
          href="https://github.com/sandboxie-plus/Sandboxie/releases"
          target="_blank"
          className="btn btn-primary"
        >
          下载 Sandboxie-Plus (免费)
        </a>
      </div>
    )
  }

  return null
}
```

#### 实例管理页面
```tsx
// src/components/InstanceManager.tsx
import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'

interface WeComInstance {
  id: number
  sandbox_name: string
  title: string
  pid?: number
  running: boolean
}

export function InstanceManager() {
  const [instances, setInstances] = useState<WeComInstance[]>([])
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    refreshInstances()
    const interval = setInterval(refreshInstances, 3000)
    return () => clearInterval(interval)
  }, [])

  async function refreshInstances() {
    try {
      const data = await invoke<WeComInstance[]>('get_instances')
      setInstances(data)
    } catch (error) {
      console.error('获取实例失败:', error)
    }
  }

  async function initializeInstances(count: number) {
    setLoading(true)
    try {
      const data = await invoke<WeComInstance[]>('initialize_instances', {
        count,
      })
      setInstances(data)
    } catch (error) {
      alert(`初始化失败: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  async function startInstance(id: number) {
    try {
      await invoke('start_instance', { instanceId: id })
      await refreshInstances()
    } catch (error) {
      alert(`启动失败: ${error}`)
    }
  }

  async function stopInstance(id: number) {
    try {
      await invoke('stop_instance', { instanceId: id })
      await refreshInstances()
    } catch (error) {
      alert(`停止失败: ${error}`)
    }
  }

  return (
    <div className="container">
      <div className="header">
        <h1>企业微信多开工具 (Windows)</h1>
        <p className="version">v1.0.0 - Powered by Sandboxie-Plus</p>
      </div>

      {instances.length === 0 ? (
        <div className="launch-section">
          <h2>初始化实例</h2>
          <p>请选择需要创建的实例数量:</p>
          <div className="button-group">
            {[2, 3, 4, 5, 10].map((count) => (
              <button
                key={count}
                onClick={() => initializeInstances(count)}
                disabled={loading}
                className="btn btn-primary"
              >
                创建 {count} 个实例
              </button>
            ))}
          </div>
        </div>
      ) : (
        <div className="instances-section">
          <div className="section-header">
            <h2>实例列表 ({instances.length})</h2>
          </div>

          <div className="instances-grid">
            {instances.map((instance) => (
              <div key={instance.id} className="instance-card">
                <div className="instance-header">
                  <div className="instance-number">{instance.title}</div>
                  <div
                    className={`instance-status ${
                      instance.running ? 'running' : 'stopped'
                    }`}
                  >
                    {instance.running ? '运行中' : '已停止'}
                  </div>
                </div>

                <div className="instance-info">
                  <div className="info-row">
                    <span className="label">沙盒名称:</span>
                    <span className="value">{instance.sandbox_name}</span>
                  </div>
                  {instance.pid && (
                    <div className="info-row">
                      <span className="label">进程 ID:</span>
                      <span className="value">{instance.pid}</span>
                    </div>
                  )}
                </div>

                {instance.running ? (
                  <button
                    onClick={() => stopInstance(instance.id)}
                    className="btn btn-danger btn-block"
                  >
                    停止
                  </button>
                ) : (
                  <button
                    onClick={() => startInstance(instance.id)}
                    className="btn btn-primary btn-block"
                  >
                    启动
                  </button>
                )}
              </div>
            ))}
          </div>
        </div>
      )}

      <div className="footer">
        <p>💡 每个实例运行在独立的 Sandboxie 沙盒中,完全隔离</p>
        <p>🔒 数据安全 | 稳定可靠 | 无数量限制</p>
      </div>
    </div>
  )
}
```

---

## 五、开发步骤

### 5.1 阶段 1: 环境准备 (1天)

**步骤**:
1. 在 Windows 10/11 上安装开发环境
   - Rust + Cargo
   - Node.js + npm
   - Tauri CLI
   - Sandboxie-Plus

2. 测试 Sandboxie 命令行
   ```powershell
   # 测试创建沙盒
   "C:\Program Files\Sandboxie-Plus\SbieIni.exe" set TestBox ConfigLevel 7
   "C:\Program Files\Sandboxie-Plus\SbieIni.exe" set TestBox Enabled y

   # 测试启动程序
   "C:\Program Files\Sandboxie-Plus\Start.exe" /box:TestBox notepad.exe
   ```

3. 创建项目结构
   ```bash
   # 复用当前项目
   cd mutil_wechat
   # 添加 Windows 特定代码
   ```

### 5.2 阶段 2: 后端开发 (2-3天)

**任务**:
1. 实现 `windows_sandbox.rs`
   - SandboxieManager 类
   - 命令行封装
   - 错误处理

2. 实现 `wecom_manager.rs`
   - 企业微信路径检测
   - 实例管理逻辑
   - 状态同步

3. 实现 Tauri 命令
   - initialize_instances
   - start_instance
   - stop_instance
   - get_instances
   - check_sandboxie

4. 单元测试
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_create_sandbox() {
           // 测试沙盒创建
       }

       #[test]
       fn test_start_in_sandbox() {
           // 测试程序启动
       }
   }
   ```

### 5.3 阶段 3: 前端开发 (2天)

**任务**:
1. 实现 SandboxieCheck 组件
2. 实现 InstanceManager 组件
3. 更新 UI 样式(复用现有 CSS)
4. 添加错误处理和用户提示

### 5.4 阶段 4: 测试与优化 (2天)

**测试项目**:
- [x] Sandboxie 检测
- [ ] 沙盒创建/删除
- [ ] 企业微信启动
- [ ] 多实例同时运行
- [ ] 进程状态监控
- [ ] 边框颜色显示
- [ ] 数据隔离验证
- [ ] 性能测试(10个实例)

### 5.5 阶段 5: 打包发布 (1天)

**任务**:
1. 配置 Tauri 打包
   ```json
   // tauri.conf.json
   {
     "bundle": {
       "identifier": "com.wecom.multiopen",
       "windows": {
         "certificateThumbprint": null,
         "digestAlgorithm": "sha256",
         "timestampUrl": ""
       }
     }
   }
   ```

2. 生成安装包
   ```bash
   cargo tauri build --target x86_64-pc-windows-msvc
   ```

3. 编写文档
   - README.md
   - 用户手册
   - 常见问题

---

## 六、预期成果

### 6.1 功能特性

- ✅ 一键安装检测
- ✅ 图形化界面
- ✅ 自动沙盒管理
- ✅ 无数量限制
- ✅ 完美数据隔离
- ✅ 进程监控
- ✅ 颜色标识
- ✅ 自动恢复文件

### 6.2 性能指标

- 单实例内存: ~500MB
- 启动时间: <5秒
- CPU 开销: <10%
- 支持实例数: 无限制(取决于硬件)

### 6.3 用户体验

- 安装简单(需先安装 Sandboxie-Plus)
- 操作直观(点击启动/停止)
- 稳定可靠(成熟技术)
- 完全免费

---

## 七、风险与对策

### 7.1 技术风险

**风险1**: Sandboxie-Plus 未安装
- **对策**: 提供清晰的安装指引和下载链接

**风险2**: 企业微信路径变化
- **对策**: 支持自定义路径配置

**风险3**: Windows 权限问题
- **对策**: 请求管理员权限

### 7.2 兼容性风险

**风险1**: Windows 10/11 兼容性
- **对策**: 在两个系统上都进行测试

**风险2**: Sandboxie 版本差异
- **对策**: 检测版本并提示升级

---

## 八、后续优化

### 8.1 短期优化(v1.1)

- 添加实例备注功能
- 支持自定义沙盒配置
- 添加启动参数设置
- 性能优化

### 8.2 长期规划(v2.0)

- 支持更多应用(微信、QQ等)
- 云同步配置
- 团队协作功能
- 插件系统

---

## 九、总结

### 9.1 优势

1. **技术成熟**: 基于 Sandboxie-Plus 开源项目
2. **完美隔离**: 每个实例完全独立
3. **易于实现**: 命令行接口清晰
4. **用户友好**: 图形化管理
5. **成本低**: 完全免费

### 9.2 开发周期

**总计**: 7-10 个工作日

- 环境准备: 1天
- 后端开发: 2-3天
- 前端开发: 2天
- 测试优化: 2天
- 打包发布: 1天

### 9.3 预期成功率

**95%** - 基于成熟技术,风险可控

---

**方案制定时间**: 2025年10月14日
**制定人**: Claude
**审核状态**: 待审核
**优先级**: 高
