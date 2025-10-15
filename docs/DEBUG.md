# 调试指南

## 控制台窗口行为

### Release 模式 (正常使用)
双击运行 `wecom-multi-open.exe` 或 `.app` 时:
- ✅ **不会显示控制台窗口**
- ✅ 只显示 GUI 界面
- ✅ 提供更好的用户体验

### Debug 模式 (查看日志)

#### Windows

**方法 1: 从终端启动 (推荐)**
```cmd
# 在安装目录打开 cmd 或 PowerShell
.\wecom-multi-open.exe

# 或者使用完整路径
"C:\Program Files\wecom-multi-open\wecom-multi-open.exe"
```

**方法 2: 从 PowerShell 启动并查看输出**
```powershell
Start-Process -FilePath ".\wecom-multi-open.exe" -NoNewWindow -Wait
```

**方法 3: 运行开发版本**
```bash
# 在项目目录
cargo run --features gui
```

#### macOS

**方法 1: 从终端启动 (推荐)**
```bash
# 在应用目录
/Applications/wecom-multi-open.app/Contents/MacOS/wecom-multi-open

# 查看实时日志
/Applications/wecom-multi-open.app/Contents/MacOS/wecom-multi-open 2>&1 | tee app.log
```

**方法 2: 使用 Console.app 查看日志**
1. 打开 `/Applications/Utilities/Console.app`
2. 搜索 "wecom-multi-open"
3. 查看应用的所有 stdout/stderr 输出

**方法 3: 运行开发版本**
```bash
# 在项目目录
cargo run --features gui
```

## 日志级别

应用使用 `println!` 和 `eprintln!` 输出日志:

```rust
println!("✓ 成功信息");      // 正常操作
println!("⚠ 警告信息");      // 潜在问题
eprintln!("✗ 错误信息");     // 错误和失败
```

### 启用详细日志

```bash
# 设置 Rust 日志级别
RUST_LOG=debug cargo run --features gui

# Windows PowerShell
$env:RUST_LOG="debug"; cargo run --features gui
```

## 常见调试场景

### 1. 实例启动失败
```bash
# 运行并捕获输出
./wecom-multi-open 2>&1 | tee debug.log

# 查看日志中的错误信息
grep "✗" debug.log
grep "失败" debug.log
```

### 2. 路径检测问题
```bash
# Windows
./wecom-multi-open.exe > path-debug.log 2>&1

# macOS
/Applications/wecom-multi-open.app/Contents/MacOS/wecom-multi-open > path-debug.log 2>&1
```

### 3. Sandboxie 集成问题 (Windows)
```powershell
# 检查 Sandboxie 日志
Get-Content "$env:USERPROFILE\Sandbox\*\Logs\*.log" -Tail 50

# 运行应用并查看 Sandboxie 相关输出
.\wecom-multi-open.exe 2>&1 | Select-String "Sandboxie"
```

## 构建模式差异

| 模式 | 控制台窗口 | 用途 |
|------|-----------|------|
| Release (`cargo build --release`) | ❌ 隐藏 | 最终用户 |
| Debug (`cargo build`) | ✅ 显示 | 开发调试 |
| Dev (`cargo run`) | ✅ 显示 | 实时开发 |

## 技术实现

控制台隐藏通过 Rust 编译器属性实现:

```rust
// src/gui.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

- `debug_assertions`: Debug 模式下为 `true`,显示控制台
- `windows_subsystem = "windows"`: Release 模式下隐藏控制台
- 仅影响 Windows,macOS 和 Linux 默认无控制台

## CLI 版本

CLI 版本 (`wecom-multi-open-cli`) 始终保留控制台:

```bash
# Windows
wecom-multi-open-cli.exe 3

# macOS/Linux
wecom-multi-open-cli 3
```

CLI 版本设计为命令行工具,不受 `windows_subsystem` 影响。

## 故障排查

### 看不到任何输出?

1. **确认从终端启动**:双击启动不会显示输出
2. **检查重定向**:使用 `2>&1` 捕获 stderr
3. **运行 Debug 版本**:
   ```bash
   cargo build --features gui
   ./target/debug/wecom-multi-open
   ```

### 应用启动但无响应?

1. **查看错误日志**:
   ```bash
   ./wecom-multi-open 2> error.log
   cat error.log
   ```

2. **检查进程状态**:
   ```bash
   # Windows
   tasklist | findstr "wecom"

   # macOS
   ps aux | grep wecom
   ```

3. **验证依赖**:
   - Windows: 检查 Sandboxie-Plus 是否正确安装
   - macOS: 检查应用权限设置

## 相关文件

- `src/gui.rs`: GUI 入口点,包含控制台配置
- `src/main.rs`: CLI 入口点,始终有控制台
- `build.rs`: 构建脚本
- `tauri.conf.json`: Tauri 配置
