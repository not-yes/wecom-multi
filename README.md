# 企业微信多开工具 (WeWork Multi-Open)

## 项目简介

100% 开源的企业微信 PC 端多开工具,采用系统级 Mutex 管理机制实现多开功能。

### 核心特性

- ✅ 100% 开源 (MIT License)
- ✅ 不修改企业微信主程序
- ✅ 不注入 DLL、不 Hook
- ✅ 支持任意实例数
- ✅ 支持开机自启
- ✅ GUI 一键操作
- ✅ 跨平台架构设计

## 技术架构

```
┌─────────────────────────────┐
│  Presentation Layer         │  GUI / 系统托盘
│  Tauri + React              │
└──────────────┬──────────────┘
               │ IPC (JSON-RPC)
┌──────────────┴──────────────┐
│  Service Layer              │  多开守护服务
│  Rust + windows-rs          │  Mutex 关闭 + 进程拉起
└──────────────┬──────────────┘
               │
┌──────────────┴──────────────┐
│  WeWork Layer               │  企业微信原版程序
└─────────────────────────────┘
```

## 目录结构

```
wecom-multi-open/
├── src-tauri/           # Rust 后端服务
│   ├── src/
│   │   ├── main.rs      # 主入口
│   │   ├── lib/
│   │   │   ├── mutex.rs # Mutex 关闭逻辑
│   │   │   └── process.rs # 进程管理
│   │   └── commands.rs  # Tauri 命令
│   └── Cargo.toml
├── src/                 # React 前端
│   ├── App.tsx
│   ├── main.tsx
│   └── components/
├── scripts/
│   ├── handle.ps1       # PowerShell 降级脚本
│   └── README.md
├── docs/                # 文档
├── tests/               # 测试
└── package.json
```

## 快速开始

### 最简单的方式 - 使用 PowerShell 脚本

无需编译,直接运行:

```powershell
# 启动 2 个企业微信实例
.\scripts\wecom_multi_open.ps1 -Count 2

# 启动 3 个实例
.\scripts\wecom_multi_open.ps1 -Count 3
```

详细说明请查看 [快速开始指南](QUICKSTART.md)

### 使用完整 GUI 应用

#### 环境要求

- Node.js >= 18
- Rust >= 1.70
- Windows 10/11 (核心功能仅支持 Windows)

#### 安装依赖

```bash
npm install
```

#### 开发模式

```bash
npm run tauri:dev
```

#### 构建

```bash
npm run tauri:build
```

更多详情请查看:
- [快速开始指南](QUICKSTART.md)
- [用户指南](docs/USER_GUIDE.md)
- [开发文档](docs/DEVELOPMENT.md)

## 工作原理

1. **启动阶段**: 枚举系统句柄,查找并关闭企业微信独占 Mutex (`Tencent.WeWork.ExclusiveObject`)
2. **进程拉起**: 立即启动企业微信进程,在 Mutex 重建前完成启动
3. **守护模式**: 可选的进程监控,确保多实例稳定运行

## 安全与合规

- 仅操作系统公开句柄,不修改企业微信程序
- 符合 Windows 使用条款
- MIT 开源许可,允许商业使用

## License

MIT License - 详见 [LICENSE](LICENSE) 文件

## 贡献

欢迎提交 Issue 和 Pull Request!

## 免责声明

本工具仅供学习研究使用,使用者需自行承担使用风险。请遵守企业微信服务条款和相关法律法规。
