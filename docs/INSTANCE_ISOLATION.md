# 实例隔离配置说明

## 概述

本项目提供两种Windows实例隔离方案,以及macOS原生隔离支持:

| 平台 | 方法 | 数据隔离 | 代理隔离 | 需要额外软件 |
|------|------|---------|---------|------------|
| **macOS** | 应用副本 + 环境变量 | ✅ 完全隔离 | ✅ 支持 | ❌ 无需 |
| **Windows** | Sandboxie沙盒 | ✅ 完全隔离 | ✅ 支持 | ✅ 需要Sandboxie-Plus |
| **Windows** | Mutex关闭(简单模式) | ❌ 共享数据 | ❌ 不支持 | ❌ 无需 |

---

## macOS 实例隔离 ✅

### 实现原理

macOS版本通过以下方式实现完全隔离:

1. **应用副本**: 为每个实例创建独立的 `.app` 副本
2. **Bundle ID修改**: 修改 `Info.plist` 使每个实例有唯一标识
3. **数据目录隔离**: 通过环境变量为每个实例分配独立的数据目录

### 数据目录

每个实例的数据存储在:
```
~/Library/Containers/WeComInstance{N}/
├── Documents/       # 文档
├── tmp/            # 临时文件
├── config/         # 配置(XDG_CONFIG_HOME)
├── data/           # 数据(XDG_DATA_HOME)
└── cache/          # 缓存(XDG_CACHE_HOME)
```

### 代理配置

通过设置 `HTTP_PROXY` 和 `HTTPS_PROXY` 环境变量实现:

```rust
Command::new(&executable_path)
    .env("HOME", &instance_home)
    .env("HTTP_PROXY", "http://127.0.0.1:7890")
    .env("HTTPS_PROXY", "http://127.0.0.1:7890")
    .spawn()
```

### 使用示例

```rust
use wecom_multi_open::{SpawnRequest, InstanceConfig, AppType};

let req = SpawnRequest {
    count: 3,
    app_path: None,
    app_type: Some(AppType::WeCom),
    instance_configs: Some(vec![
        InstanceConfig {
            data_dir: None,  // 自动分配
            proxy: Some("http://127.0.0.1:7890".to_string()),
        },
        InstanceConfig {
            data_dir: None,
            proxy: Some("http://127.0.0.1:7891".to_string()),
        },
        InstanceConfig {
            data_dir: None,
            proxy: None,
        },
    ]),
};

platform::spawn_multiple(req).await?;
```

---

## Windows 方案1: Sandboxie沙盒隔离 ✅ (推荐)

### 实现原理

使用 [Sandboxie-Plus](https://github.com/sandboxie-plus/Sandboxie) 为每个实例创建独立的沙盒环境。

### 优势

- ✅ **完全隔离**: 文件系统、注册表、进程全部隔离
- ✅ **安全性高**: 沙盒内的操作不会影响主系统
- ✅ **可配置**: 支持精细的访问控制规则
- ✅ **可视化管理**: 通过Sandboxie Control可以查看每个沙盒状态

### 前置条件

1. **安装 Sandboxie-Plus**
   - 下载地址: https://github.com/sandboxie-plus/Sandboxie/releases
   - 安装到默认路径: `C:\Program Files\Sandboxie-Plus\`

2. **管理员权限**
   - 首次使用需要管理员权限配置沙盒

### 使用示例

```rust
use wecom_multi_open::wecom_manager::WeComManager;

// 创建管理器
let manager = WeComManager::new()?;

// 启动3个隔离实例
let instances = manager.spawn_multiple(3)?;

for instance in &instances {
    println!("实例 {}: PID={:?}, 沙盒={}",
        instance.id,
        instance.pid,
        instance.sandbox_name
    );
}

// 停止特定实例
manager.stop_instance(&instances[0].sandbox_name, instances[0].pid)?;

// 删除沙盒配置
manager.delete_sandbox(&instances[0].sandbox_name)?;
```

### 沙盒配置

每个实例在独立沙盒中运行:

```
沙盒名称: WeCom_1, WeCom_2, WeCom_3, ...
配置级别: 7 (标准隔离)
自动删除: 否 (保留数据)
```

### 数据位置

沙盒数据存储在:
```
C:\Sandbox\{用户名}\WeCom_{N}\
├── drive\          # 虚拟文件系统
├── user\           # 用户数据
└── RegHive         # 注册表副本
```

### 限制

- ⚠️ 需要额外安装Sandboxie-Plus软件
- ⚠️ 首次配置需要管理员权限
- ⚠️ 沙盒启动速度略慢于普通方式

---

## Windows 方案2: Mutex关闭(简单模式) ⚠️

### 实现原理

关闭系统级Mutex对象,允许多个实例同时运行。

### 优势

- ✅ **无需额外软件**: 不依赖第三方工具
- ✅ **启动速度快**: 无沙盒开销
- ✅ **轻量级**: 对系统影响小

### 限制

- ❌ **无数据隔离**: 所有实例共享相同的数据目录
- ❌ **可能冲突**: 多个实例操作同一文件可能导致问题
- ❌ **无代理隔离**: 所有实例使用系统代理设置

### 数据目录

所有实例共享:
```
C:\Users\{用户名}\Documents\WeChat Files\     # 微信
C:\Users\{用户名}\AppData\Roaming\Tencent\WXWork\  # 企业微信
```

### 使用场景

适用于:
- 快速测试多开功能
- 不同账号登录,不关心数据混合
- 临时使用,关闭后不保留数据

### 使用示例

```rust
use wecom_multi_open::{SpawnRequest, AppType, platform};

let req = SpawnRequest {
    count: 3,
    app_path: None,
    app_type: Some(AppType::WeCom),
    instance_configs: None,  // 简单模式不支持配置
};

let response = platform::spawn_multiple(req).await?;
println!("成功启动 {} 个实例", response.success);
```

---

## 注册表配置方案 (待实现)

### 原理

通过修改注册表项改变数据目录:

**企业微信:**
```
HKEY_CURRENT_USER\Software\Tencent\WXWork
└── DataLocationPath = "D:\WeCom\Instance1"
```

**个人微信:**
```
HKEY_CURRENT_USER\Software\Tencent\WeChat
└── FileSavePath = "D:\WeChat\Instance1"
```

### 实现思路

1. 在启动每个实例前修改注册表
2. 为每个实例设置独立的数据路径
3. 启动实例后恢复注册表

### 优势

- ✅ 无需额外软件
- ✅ 真正的数据隔离

### 挑战

- ⚠️ 需要在实例启动前修改注册表
- ⚠️ 多个实例同时运行时,注册表值只有一个
- ⚠️ 可能需要重启应用才能生效

### 状态

🚧 **待实现** - 需要进一步研究时序问题

---

## API 接口

### InstanceConfig 结构

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstanceConfig {
    /// 自定义数据目录 (可选)
    pub data_dir: Option<PathBuf>,

    /// 代理配置,格式: "http://host:port" (可选)
    pub proxy: Option<String>,
}
```

### SpawnRequest 结构

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpawnRequest {
    /// 启动实例数量
    pub count: u8,

    /// 应用程序路径 (可选,默认自动检测)
    pub app_path: Option<PathBuf>,

    /// 应用类型 (WeCom 或 WeChat)
    pub app_type: Option<AppType>,

    /// 每个实例的配置 (可选)
    /// 如果提供,长度应等于 count
    pub instance_configs: Option<Vec<InstanceConfig>>,
}
```

---

## 选择建议

### 推荐方案

| 场景 | 推荐方案 | 原因 |
|------|---------|------|
| **macOS用户** | macOS原生隔离 | 无需配置,自动完全隔离 |
| **Windows高级用户** | Sandboxie沙盒 | 完全隔离,安全可靠 |
| **Windows快速测试** | Mutex关闭(简单模式) | 无需安装,即开即用 |
| **企业环境** | Sandboxie沙盒 | 符合安全规范 |

### 对比总结

**Sandboxie沙盒** vs **Mutex简单模式**:

| 特性 | Sandboxie | Mutex模式 |
|-----|----------|----------|
| 数据隔离 | ✅ 完全隔离 | ❌ 完全共享 |
| 安全性 | ✅ 高 | ⚠️ 一般 |
| 启动速度 | ⚠️ 较慢 | ✅ 快 |
| 系统要求 | ⚠️ 需Sandboxie-Plus | ✅ 无 |
| 学习成本 | ⚠️ 中等 | ✅ 低 |

---

## 常见问题

### Q: macOS需要手动配置吗?
**A:** 不需要。macOS版本自动处理所有隔离配置。

### Q: Windows简单模式会不会导致数据混乱?
**A:** 可能会。多个实例操作同一数据目录可能导致:
- 聊天记录混乱
- 文件下载冲突
- 配置被覆盖

建议使用Sandboxie沙盒方案避免此问题。

### Q: 如何为不同实例设置不同代理?
**A:**
- **macOS**: 通过 `InstanceConfig.proxy` 字段指定
- **Windows Sandboxie**: 通过沙盒配置设置网络规则
- **Windows 简单模式**: 不支持

### Q: Sandboxie-Plus是免费的吗?
**A:** 是的,Sandboxie-Plus是开源免费软件。

### Q: 能否同时使用Sandboxie和简单模式?
**A:** 可以,但不建议。建议统一使用一种方案避免混乱。

---

## 未来改进

- [ ] Windows注册表配置方案实现
- [ ] GUI界面支持Sandboxie配置
- [ ] 自动检测Sandboxie并提示安装
- [ ] 实例配置持久化
- [ ] 批量导入/导出实例配置
