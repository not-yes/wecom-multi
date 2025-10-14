# macOS 快速开始指南

## 🚀 最快使用方式 (3 分钟)

### 方法 1: 使用预编译版本 (推荐)

```bash
# 1. 从 GitHub Releases 下载预编译版本
# https://github.com/aaronwang123321/wecom-multi/releases

# 2. 下载后赋予执行权限
chmod +x wecom-multi-open-macos-m1  # M1/M2/M3 芯片
# 或
chmod +x wecom-multi-open-macos-intel  # Intel 芯片

# 3. 运行 (启动 3 个实例)
./wecom-multi-open-macos-m1 3

# 4. 可选: 移到系统路径
sudo mv wecom-multi-open-macos-m1 /usr/local/bin/wecom-multi-open
wecom-multi-open 5  # 现在可以在任何地方使用
```

---

## 💻 从源码编译使用

### 前置要求

1. **安装 Xcode Command Line Tools**
```bash
xcode-select --install
```

2. **安装 Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

3. **验证安装**
```bash
rustc --version
cargo --version
```

### CLI 版本 (命令行)

#### 快速编译和运行

```bash
# 1. 进入项目目录
cd /Users/wangke/Documents/Program/mutil_wechat

# 2. 开发模式运行 (快速测试)
cargo run 3

# 3. 或者编译 Release 版本
cargo build --release

# 4. 使用编译后的程序
./target/release/wecom-multi-open 3
```

#### 使用示例

```bash
# 默认启动 3 个实例
./target/release/wecom-multi-open

# 启动 5 个实例
./target/release/wecom-multi-open 5

# 启动 2 个实例
./target/release/wecom-multi-open 2
```

### GUI 版本 (图形界面)

#### 安装依赖

```bash
# 1. 确保已安装 Node.js
node --version  # 应该显示 v18+ 或更高

# 如果没有安装:
brew install node

# 2. 安装前端依赖
cd ui
npm install
cd ..
```

#### 开发模式运行

```bash
# 终端 1: 启动前端开发服务器
cd ui
npm run dev

# 终端 2: 启动 Tauri 应用
cargo run --bin wecom-multi-open-gui --features gui
```

#### 编译 Release 版本

```bash
# 方法 1: 使用 Cargo
cd ui && npm run build && cd ..
cargo build --release --bin wecom-multi-open-gui --features gui

# 方法 2: 使用 Tauri CLI (推荐)
cargo install tauri-cli
cargo tauri build

# 输出位置:
# - App: target/release/bundle/macos/wecom-multi-open.app
# - DMG: target/release/bundle/dmg/wecom-multi-open_0.2.0_x64.dmg
```

---

## 📝 使用说明

### CLI 版本特点

✅ **优势**:
- 体积小 (< 2MB)
- 启动快
- 适合命令行用户

❌ **限制**:
- 无图形界面
- 无实例管理
- 无系统托盘

### GUI 版本特点

✅ **优势**:
- 图形界面友好
- 实时监控实例
- 系统托盘支持
- 一键管理所有实例

❌ **限制**:
- 体积较大 (8MB)
- 需要安装 Node.js 依赖

---

## 🎯 企业微信路径

默认路径: `/Applications/WeCom.app`

如果安装在其他位置,暂时不支持自定义路径 (计划中)。

---

## ❓ 常见问题

### 1. 提示 "应用程序不存在"

确认企业微信已安装:
```bash
ls -la /Applications/WeCom.app
```

如果不存在,请先安装企业微信。

### 2. macOS 提示 "无法打开,因为无法验证开发者"

```bash
# 方法 1: 允许此应用
sudo spctl --add ./wecom-multi-open-macos-m1
sudo xattr -d com.apple.quarantine ./wecom-multi-open-macos-m1

# 方法 2: 在系统偏好设置中允许
# 系统偏好设置 > 安全性与隐私 > 通用 > 仍要打开
```

### 3. 提示 "command not found: cargo"

Rust 未正确安装或环境变量未生效:
```bash
# 重新加载环境变量
source ~/.cargo/env

# 或重新打开终端
```

### 4. 编译失败: "linker `cc` not found"

需要安装 Xcode Command Line Tools:
```bash
xcode-select --install
```

### 5. macOS 上可以启动多少个实例?

建议:
- 8GB 内存: 3-5 个实例
- 16GB 内存: 5-10 个实例
- 32GB+ 内存: 10+ 个实例

### 6. 如何查看正在运行的实例?

```bash
# 查看所有企业微信进程
ps aux | grep WeCom

# 查看进程数量
ps aux | grep WeCom | wc -l
```

### 7. 如何关闭所有实例?

```bash
# 方法 1: 关闭所有企业微信
killall WeCom

# 方法 2: 强制关闭
killall -9 WeCom

# 方法 3: 关闭特定 PID
kill <PID>
```

---

## 🔧 性能优化

### Release 编译优化

已在 `Cargo.toml` 中配置:
```toml
[profile.release]
opt-level = "z"     # 优化体积
lto = true          # 链接时优化
codegen-units = 1   # 单个代码生成单元
strip = true        # 移除符号
```

### 编译后体积

- CLI 版本: ~600 KB
- GUI 版本: ~8 MB (包含 Web 前端)

---

## 🐛 调试

### 开启详细日志

```bash
# CLI 版本
RUST_LOG=debug cargo run 3

# GUI 版本
RUST_LOG=debug cargo run --bin wecom-multi-open-gui --features gui
```

### 查看系统日志

```bash
# 查看控制台日志
log show --predicate 'process == "wecom-multi-open"' --last 5m

# 实时监控
log stream --predicate 'process == "wecom-multi-open"'
```

---

## 📦 卸载

### 删除编译后的程序

```bash
# 删除 Release 文件
rm -rf target/release

# 删除整个构建目录
cargo clean
```

### 卸载 Rust (可选)

```bash
rustup self uninstall
```

---

## 🎨 macOS 特色

### macOS 多开原理

macOS 不使用 Mutex 机制,直接使用 `open -n` 命令:

```bash
# -n 参数表示打开新实例
open -n -a "WeCom"
open -n -a "WeCom"
open -n -a "WeCom"
```

这比 Windows 的实现简单得多!

### AppleScript 自动化

可以结合 AppleScript 使用:

```applescript
-- 启动 3 个企业微信实例
repeat 3 times
    do shell script "open -n -a 'WeCom'"
    delay 0.5
end repeat
```

---

## 📚 相关文档

- [MACOS_GUIDE.md](MACOS_GUIDE.md) - 详细的 macOS 使用指南
- [GUI_GUIDE.md](GUI_GUIDE.md) - GUI 版本使用指南
- [BUILD_GUIDE.md](BUILD_GUIDE.md) - 编译构建指南
- [README.md](README.md) - 项目总览

---

## 💡 提示

1. **首次使用建议**:
   - 先尝试启动 2-3 个实例测试
   - 观察内存使用情况
   - 根据机器性能调整实例数量

2. **最佳实践**:
   - 使用 CLI 版本做快速测试
   - 日常使用建议 GUI 版本 (更方便管理)

3. **芯片选择**:
   - M1/M2/M3 芯片: 使用 `aarch64-apple-darwin` 版本
   - Intel 芯片: 使用 `x86_64-apple-darwin` 版本
   - 不确定? 运行 `uname -m` 查看:
     - `arm64` = M 系列芯片
     - `x86_64` = Intel 芯片

---

**享受 macOS 上的企业微信多开!** 🎉
