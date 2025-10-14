# 构建指南 - 超级简化版

## 🎯 目标

编译出一个 **单个 EXE 文件** (约 1.5MB),可以在任何 Windows 电脑直接运行。

## 📋 准备工作 (只需一次)

### 1. 安装 Rust

**Windows 安装**:

1. 访问 https://rustup.rs/
2. 下载 `rustup-init.exe`
3. 双击运行,按默认选项安装
4. 重启终端 (PowerShell 或 CMD)
5. 验证安装:
   ```cmd
   rustc --version
   cargo --version
   ```

**耗时**: 约 5 分钟

**只需安装一次**,以后可以编译任何 Rust 项目。

---

## 🚀 编译步骤

### 方法 1: 使用一键脚本 (推荐)

```cmd
# 双击运行
build.bat
```

等待 5-10 分钟,完成!

### 方法 2: 手动命令

```cmd
# 打开 CMD 或 PowerShell,进入项目目录
cd C:\path\to\wecom-multi-open

# 编译
cargo build --release

# 等待编译完成
```

---

## 📦 编译产物

编译完成后:

```
target/
└── release/
    └── wecom-multi-open.exe  ← 这就是你需要的文件!
```

**文件大小**: 约 1.5 MB
**可以**: 复制到任何 Windows 电脑使用
**无需**: 安装、配置、依赖

---

## 💡 使用编译好的 EXE

```cmd
# 双击直接运行 (默认启动 3 个)
wecom-multi-open.exe

# 或指定数量
wecom-multi-open.exe 5

# 复制到桌面使用
copy target\release\wecom-multi-open.exe %USERPROFILE%\Desktop\
```

---

## 🔧 首次编译慢的原因

第一次编译需要:
1. 下载 Rust 依赖包 (约 50MB)
2. 编译依赖项 (tokio, windows-rs 等)
3. 编译项目代码

**首次**: 5-10 分钟
**后续**: < 30 秒

编译完成后,所有依赖都已缓存,下次修改代码只需重新编译很少的部分。

---

## 🐛 常见问题

### Q: 编译报错 "linker 'link.exe' not found"

**原因**: 缺少 MSVC 工具链

**解决**:
1. 下载 Visual Studio Build Tools
2. 地址: https://visualstudio.microsoft.com/downloads/
3. 安装时勾选 "Desktop development with C++"
4. 重新运行编译

或使用 GNU 工具链:
```cmd
rustup default stable-x86_64-pc-windows-gnu
```

### Q: 编译很慢,如何加速?

**使用国内镜像**:

创建/编辑 `%USERPROFILE%\.cargo\config.toml`:
```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

### Q: 编译后 EXE 太大?

**优化大小**:

编辑 `Cargo.toml`,添加:
```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
```

重新编译:
```cmd
cargo build --release
```

可将大小减小到约 500KB。

### Q: 需要跨平台?

**Linux**:
```bash
cargo build --release
```

**macOS**:
```bash
# macOS 不支持 Windows API
# 需要使用不同的实现
```

---

## 📊 编译时间参考

| 硬件配置 | 首次编译 | 增量编译 |
|---------|---------|---------|
| i5 + HDD | 10-15 分钟 | 1-2 分钟 |
| i7 + SSD | 5-8 分钟 | 10-30 秒 |
| Ryzen 5 + NVMe | 3-5 分钟 | 5-15 秒 |

---

## 🎁 分发给其他用户

编译后的 `wecom-multi-open.exe`:

✅ **可以**:
- 复制到任何 Windows 10/11 电脑
- 通过 U 盘、网盘分享
- 上传到 GitHub Releases
- 打包成 ZIP 分发

❌ **不需要**:
- 其他用户安装 Rust
- 包含源代码
- 安装任何运行时

**就是一个简单的 EXE 文件!**

---

## 🔐 代码签名 (可选)

为避免 Windows SmartScreen 警告:

1. 购买代码签名证书 (约 $100-300/年)
2. 使用 `signtool` 签名:
   ```cmd
   signtool sign /f cert.pfx /p password wecom-multi-open.exe
   ```

或者:
- 用户首次运行时点击 "更多信息" → "仍要运行"
- 等待足够多用户运行后,Windows 会自动信任

---

## 📝 总结

1. **安装 Rust** (一次性,5 分钟)
2. **运行 build.bat** (首次 5-10 分钟)
3. **得到 EXE** (可分发给所有人)

**就是这么简单!**

如有问题,查看 `README_SIMPLE.md` 或提交 Issue。
