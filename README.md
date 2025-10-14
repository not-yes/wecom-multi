# 企业微信多开工具

> 轻量级、零配置、单文件 EXE - 让企业微信多开变得超级简单!

## ⚡ 特点

- ✅ **单文件 EXE** - 只有 1.5MB,双击就能用
- ✅ **零配置** - 自动检测企业微信路径
- ✅ **零侵入** - 不修改程序,不注入代码
- ✅ **超轻量** - 核心代码仅 300 行
- ✅ **开源免费** - MIT 协议,可自由使用

## 🚀 快速开始

### 方式 1: 直接使用 (推荐)

**下载预编译的 EXE**:

1. 前往 [Releases](https://github.com/yourusername/wecom-multi-open/releases) 页面
2. 下载 `wecom-multi-open.exe`
3. 双击运行

```cmd
# 默认启动 3 个实例
wecom-multi-open.exe

# 启动 5 个实例
wecom-multi-open.exe 5
```

### 方式 2: PowerShell 脚本 (无需编译)

```powershell
cd scripts
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
.\wecom-multi-open-simple.ps1 -Count 2
```

### 方式 3: 自己编译

```bash
# 1. 安装 Rust (只需一次)
# 访问 https://rustup.rs/

# 2. 编译
cargo build --release

# 3. 使用
target\release\wecom-multi-open.exe
```

**一键构建**: 双击 `build.bat`

## 📋 系统要求

- Windows 10/11 (64位)
- 已安装企业微信
- 4GB+ 内存 (推荐 8GB)

## 🎯 使用场景

- **多账号管理** - 同时登录多个企业微信账号
- **工作测试分离** - 生产环境和测试环境分开
- **多企业协作** - 管理多个企业的账号
- **效率提升** - 避免频繁切换账号

## 💡 工作原理

1. 查找系统中企业微信的独占 Mutex (`Tencent.WeWork.ExclusiveObject`)
2. 关闭该 Mutex
3. 快速启动企业微信进程
4. 重复 N 次

**完全安全** - 只操作系统公开 API,不修改任何程序文件。

## 📖 文档

- [README_SIMPLE.md](README_SIMPLE.md) - 极简使用指南
- [BUILD_GUIDE.md](BUILD_GUIDE.md) - 编译构建指南
- [scripts/README.md](scripts/README.md) - PowerShell 脚本说明

## ❓ 常见问题

### 提示"企业微信程序不存在"?

确认企业微信已安装在以下位置之一:
- `C:\Program Files (x86)\WXWork\WXWork.exe`
- `C:\Program Files\WXWork\WXWork.exe`

### 启动失败?

1. 以管理员身份运行
2. 关闭杀毒软件或添加信任
3. 确保企业微信未在运行

### 建议启动几个实例?

| 内存 | 推荐实例数 |
|------|-----------|
| 4GB | 2-3 个 |
| 8GB | 3-5 个 |
| 16GB+ | 5-10 个 |

### 如何关闭?

直接关闭企业微信窗口,或在任务管理器中结束进程。

## 🔧 技术栈

- **语言**: Rust (安全、高性能)
- **依赖**:
  - `windows-rs` - Windows API 绑定
  - `tokio` - 异步运行时
- **代码量**: < 400 行

## 📦 项目结构

```
wecom-multi-open/
├── Cargo.toml                          # Rust 项目配置
├── src/
│   └── main.rs                         # 核心代码 (300 行)
├── scripts/
│   └── wecom-multi-open-simple.ps1    # PowerShell 脚本
├── build.bat                           # 一键构建脚本
├── README.md                           # 本文档
├── README_SIMPLE.md                    # 极简指南
└── BUILD_GUIDE.md                      # 构建指南
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request!

### 开发

```bash
# 克隆项目
git clone https://github.com/yourusername/wecom-multi-open.git
cd wecom-multi-open

# 运行开发版本
cargo run

# 编译 Release 版本
cargo build --release
```

## 📄 许可证

MIT License - 可自由使用、修改、分发

详见 [LICENSE](LICENSE) 文件

## ⚠️ 免责声明

- 本工具仅供学习研究使用
- 使用者需自行承担使用风险
- 请遵守企业微信服务条款
- 请遵守相关法律法规

## 🌟 Star History

如果这个项目对你有帮助,请给个 Star ⭐

---

**让企业微信多开变得简单!** 🎉
