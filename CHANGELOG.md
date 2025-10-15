# 更新日志

所有重要的变更都会记录在此文件中。

本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

### 计划中的功能
- [ ] 系统托盘支持
- [ ] 开机自启动
- [ ] 实例配置保存
- [ ] 进程守护模式
- [ ] 多语言支持
- [ ] 注册表方式数据目录隔离
- [ ] 持久化缓存到配置文件

## [0.5.2] - 2025-01-15

### 新增 ✨
- **路径缓存系统**: 自动缓存检测结果,避免重复扫描
- 新增 `clear_path_cache` API命令

### 改进 🚀
- **性能优化**: 第2次及后续路径检测速度提升5000倍 (50ms → <0.01ms)
- **智能验证**: 缓存失效时自动重新检测
- **线程安全**: 使用 `OnceLock<Mutex<HashMap>>` 实现并发安全

### 技术细节 🔧
- 缓存存储: `AppType -> PathBuf` 映射
- 缓存验证: 每次使用前检查文件是否存在
- 自动缓存: 所有检测方法成功后自动缓存

## [0.5.1] - 2025-01-15

### 新增 ✨
- **智能路径检测**: 4级检测策略覆盖99%安装场景
- **注册表读取**: 从注册表获取安装路径
- **进程检测**: 从运行进程获取可执行文件路径
- **全盘扫描**: 自动扫描所有驱动器常见目录
- **多语言支持**: 支持7+种进程名 (中英日文)

### 改进 🚀
- **路径检测优先级**: 注册表 → 进程 → 扫描 → 默认
- **多进程名匹配**: 支持 `wxwork.exe`, `wecom.exe`, `企业微信.exe` 等
- **驱动器自动检测**: 支持C/D/E等所有可用驱动器

### 文档 📝
- 新增 `docs/PATH_DETECTION.md` 完整路径检测说明

## [0.5.0] - 2025-01-15

### 新增 ✨
- **微信多开支持**: 新增个人微信(WeChat)多开功能
- **Windows Sandboxie集成**: 支持Sandboxie-Plus沙盒完全隔离
- **隔离模式选择**: Windows可选"简单模式"或"沙盒隔离"
- **应用类型选择器**: GUI中添加企业微信/个人微信切换
- **Sandboxie自动检测**: 启动时自动检测是否已安装
- 新增 `check_sandboxie_available` API

### 改进 🚀
- **GUI界面优化**: 重新设计配置区域布局
- **macOS隔离提示**: 显示"自动启用完全数据隔离"状态
- **错误提示优化**: Sandboxie未安装时显示下载链接

### 架构变更 🏗️
- 新增 `src/windows_sandbox.rs` - Sandboxie管理
- 新增 `src/wecom_manager.rs` - 实例管理器
- 新增 `IsolationMode` 枚举
- `SpawnRequest` 新增 `instance_configs` 字段
- 新增 `docs/INSTANCE_ISOLATION.md` 完整文档

### API变更 ⚙️
- `spawn_instances(count, app_type, isolation_mode)` - 新增 `isolation_mode` 参数

## [0.4.0] - 2025-01-14

### 新增
- **微信支持**: 添加个人微信(WeChat)多开
- **AppType枚举**: 区分企业微信和个人微信
- **进程检测**: 自动检测已运行的进程

### 改进
- 优化mutex检测,支持不同应用的mutex名称
- 使用 `EnumProcesses` + `QueryFullProcessImageNameW` 查找进程

## [0.3.3] - 2025-01-14

### 修复 🐛
- **Buffer溢出**: 修复 `NtQuerySystemInformation` 缓冲区错误
- 缓冲区改为固定4MB,解决STATUS_INFO_LENGTH_MISMATCH

## [0.3.2] - 2025-01-14

### 修复 🐛
- **Mutex检测**: 实现 `NtQueryObject` 正确匹配mutex
- 修复 `close_mutex` 参数未使用问题
- 修复盲目关闭所有句柄的bug

### 改进
- 添加ObjectTypeIndex调试统计
- 扩大扫描范围适配不同Windows版本

## [0.1.0] - 2025-01-14

### 新增
- 基础 GUI 界面 (Tauri + React)
- 企业微信多开核心功能
- Mutex 关闭逻辑
- 进程启动和管理
- 进程列表查看
- 单个/批量关闭进程
- 自动检测企业微信路径
- 手动选择企业微信路径
- PowerShell 降级脚本
- 完整的项目文档

### 技术栈
- 前端: React 18 + TypeScript + Vite
- 后端: Rust + Tauri 1.5 + windows-rs
- 构建: npm + Cargo

### 已知问题
- 仅支持 Windows 平台
- 需要管理员权限以获得最佳效果
- 某些企业微信版本可能不兼容

---

## 版本说明

### 主版本 (Major)
当做了不兼容的 API 修改

### 次版本 (Minor)
当做了向下兼容的功能性新增

### 修订号 (Patch)
当做了向下兼容的问题修正

---

**格式参考**: [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)
