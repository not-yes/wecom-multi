# 跨平台编译指南

## 🌍 支持的平台

- ✅ **Windows 10/11** (x64) - 完全支持
- ✅ **macOS** (Intel/Apple Silicon) - 完全支持
- ⏳ **Linux** - 计划支持

## 📦 在各平台编译

### Windows

```powershell
# 1. 安装 Rust
winget install Rustlang.Rustup

# 2. 安装 Visual Studio Build Tools
# https://visualstudio.microsoft.com/downloads/

# 3. 编译
cargo build --release

# 4. 输出
target\release\wecom-multi-open.exe
```

###macOS

```bash
# 1. 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 编译
cargo build --release

# 3. 输出
target/release/wecom-multi-open
```

### Linux (计划中)

```bash
# 1. 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 编译
cargo build --release

# 3. 输出
target/release/wecom-multi-open
```

## 🔄 交叉编译

### 在 macOS 上编译 Windows 版本

```bash
# 1. 添加 Windows 目标
rustup target add x86_64-pc-windows-gnu

# 2. 安装交叉编译工具
brew install mingw-w64

# 3. 编译
cargo build --release --target x86_64-pc-windows-gnu

# 注意: Windows API 调用可能需要在 Windows 上测试
```

### 在 Windows 上编译 macOS 版本

```powershell
# 交叉编译到 macOS 较复杂,建议使用 GitHub Actions
```

## 🤖 GitHub Actions 自动构建

项目包含 GitHub Actions 配置,推送代码后自动构建所有平台:

```yaml
# .github/workflows/release.yml
on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        os: [windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
```

### 发布流程

```bash
# 1. 更新版本号
# Cargo.toml: version = "0.2.0"

# 2. 创建 tag
git tag v0.2.0
git push origin v0.2.0

# 3. GitHub Actions 自动构建并发布
# - wecom-multi-open-windows.exe
# - wecom-multi-open-macos
```

## 📊 平台差异

### 核心实现

| 功能 | Windows | macOS | Linux |
|------|---------|-------|-------|
| Mutex 关闭 | ✅ NtQuerySystemInformation | ❌ 不需要 | ⏳ 待实现 |
| 进程启动 | ✅ CreateProcessW | ✅ open -n | ⏳ 待实现 |
| 进程管理 | ✅ OpenProcess | ✅ ps/kill | ⏳ 待实现 |

### 代码结构

```rust
// src/lib.rs
pub mod platform {
    #[cfg(target_os = "windows")]
    pub mod windows { /* Windows 实现 */ }

    #[cfg(target_os = "macos")]
    pub mod macos { /* macOS 实现 */ }

    #[cfg(target_os = "linux")]
    pub mod linux { /* Linux 实现 */ }
}
```

## 🔧 条件编译

### Cargo.toml

```toml
[target.'cfg(windows)'.dependencies]
windows = { version = "0.54", features = [...] }

[target.'cfg(target_os = "macos")'.dependencies]
# macOS 特定依赖
```

### 源代码

```rust
#[cfg(target_os = "windows")]
fn windows_specific() { ... }

#[cfg(target_os = "macos")]
fn macos_specific() { ... }

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn unsupported() {
    panic!("Platform not supported");
}
```

## 📦 二进制大小优化

### Release 配置

```toml
[profile.release]
opt-level = "z"          # 优化大小
lto = true               # 链接时优化
codegen-units = 1        # 单个代码生成单元
strip = true             # 移除符号
panic = "abort"          # 直接终止而非展开
```

### 大小对比

| 平台 | 默认 | 优化后 |
|------|------|--------|
| Windows | 1.5 MB | 800 KB |
| macOS | 1.2 MB | 600 KB |

## 🧪 测试

### 单元测试

```bash
# 运行所有测试
cargo test

# 运行特定平台测试
cargo test --lib

# 查看测试覆盖率
cargo tarpaulin
```

### 集成测试

```bash
# 端到端测试
cargo test --test integration

# 特定平台测试
cargo test --target x86_64-pc-windows-msvc
```

## 🐛 调试

### Windows

```powershell
# 开启调试日志
$env:RUST_LOG="debug"
cargo run
```

### macOS

```bash
# 开启调试日志
RUST_LOG=debug cargo run

# 使用 lldb 调试
lldb target/debug/wecom-multi-open
```

## 📝 最佳实践

1. **平台特定代码最小化**
   - 将平台差异隔离到 `platform` 模块
   - 使用统一的接口

2. **测试所有平台**
   - 在目标平台上实际测试
   - 使用 GitHub Actions CI

3. **文档更新**
   - 记录平台特定行为
   - 更新使用说明

4. **版本同步**
   - 所有平台使用相同版本号
   - 同时发布所有平台版本

## 🚀 发布清单

- [ ] 在 Windows 上测试
- [ ] 在 macOS 上测试
- [ ] 更新 CHANGELOG.md
- [ ] 更新版本号
- [ ] 创建 git tag
- [ ] 推送到 GitHub
- [ ] 验证 Actions 构建
- [ ] 发布 Release
- [ ] 更新文档

## 📚 参考资源

- [Rust Platform Support](https://doc.rust-lang.org/nightly/rustc/platform-support.html)
- [Cross Compilation](https://rust-lang.github.io/rustup/cross-compilation.html)
- [GitHub Actions for Rust](https://github.com/actions-rs)

---

**一次编写,到处运行!** 🌍
