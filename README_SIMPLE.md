# 企业微信多开工具 - 极简版

## 🚀 三种使用方式 (任选其一)

### 方式 1: 直接下载 EXE (最简单) ⭐

**适合**: 普通用户,不想编译

1. 下载 `wecom-multi-open.exe`
2. 双击运行
3. 完成!

默认启动 3 个实例,如需更多:
```cmd
wecom-multi-open.exe 5
```

---

### 方式 2: PowerShell 脚本 (无需安装)

**适合**: 快速测试

```powershell
# 打开 PowerShell
cd 项目目录\scripts

# 允许脚本运行
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass

# 启动 2 个实例
.\wecom-multi-open-simple.ps1 -Count 2
```

---

### 方式 3: 自己编译 (开发者)

**需要**: 安装 Rust

```bash
# 1. 安装 Rust (只需一次)
# 访问 https://rustup.rs/ 按提示安装

# 2. 编译
cargo build --release

# 3. 运行
target\release\wecom-multi-open.exe
```

编译后得到单个 EXE 文件 (约 1.5MB),可复制到任何电脑使用。

---

## 📋 系统要求

- Windows 10/11
- 已安装企业微信
- 4GB+ 内存 (建议 8GB)

## ❓ 常见问题

**Q: 提示"企业微信程序不存在"?**

A: 确认企业微信已安装,常见路径:
- `C:\Program Files (x86)\WXWork\WXWork.exe`
- `C:\Program Files\WXWork\WXWork.exe`

**Q: 启动失败?**

A: 尝试以管理员身份运行

**Q: 多少个实例合适?**

A:
- 4GB 内存 → 2-3 个
- 8GB 内存 → 3-5 个
- 16GB+ → 5-10 个

**Q: 如何关闭?**

A: 直接关闭企业微信窗口,或在任务管理器结束进程

---

## 🎯 就是这么简单!

**不需要**:
- ❌ Tauri
- ❌ Node.js
- ❌ 复杂配置
- ❌ 学习文档

**只需要**:
- ✅ 下载 EXE
- ✅ 双击运行
- ✅ 完成!

---

## 📦 文件说明

- `Cargo.toml` - Rust 项目配置
- `src/main.rs` - 核心代码 (约 300 行)
- `scripts/wecom-multi-open-simple.ps1` - PowerShell 脚本
- `README_SIMPLE.md` - 本文档

**总代码量**: < 400 行

---

## 🔧 技术原理

1. 查找并关闭系统 Mutex (`Tencent.WeWork.ExclusiveObject`)
2. 快速启动企业微信进程
3. 重复 N 次

**不修改程序,不注入代码,完全安全**

---

## 📄 许可证

MIT License - 可自由使用、修改、分发

---

**就是这么简单! 立即开始使用吧!** 🎉
