# macOS 版本使用指南

## 🍎 macOS 企业微信多开

本工具现已支持 macOS 平台!

### ✨ 特点

- ✅ 原生 macOS 支持
- ✅ 使用 `open -n` 命令实现多开
- ✅ 无需复杂配置
- ✅ 与 Windows 版本共用代码库

### 📋 系统要求

- macOS 10.15 (Catalina) 或更高
- 已安装企业微信 (WeCom)
- Rust 工具链 (用于编译)

### 🚀 快速开始

#### 方法 1: 编译使用

```bash
# 1. 安装 Rust (如果还没安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 编译项目
cargo build --release

# 3. 运行
./target/release/wecom-multi-open 3
```

#### 方法 2: 使用脚本 (最简单)

```bash
# 直接使用 open 命令
open -n -a "WeCom"
open -n -a "WeCom"
open -n -a "WeCom"
```

### 💡 工作原理

macOS 版本与 Windows 版本不同:

**Windows**:
- 需要关闭独占 Mutex
- 使用系统 API 枚举句柄

**macOS**:
- 使用 `open -n` 命令
- `-n` 参数表示打开新实例
- 无需关闭任何系统对象

### 📱 企业微信路径

默认路径:
```
/Applications/WeCom.app
```

如果安装在其他位置,可以指定:
```bash
wecom-multi-open --path /path/to/WeCom.app 3
```

### 🔧 编译选项

```bash
# 仅编译 CLI 版本
cargo build --release

# 编译 GUI 版本 (未来支持)
cargo build --release --features gui
```

### ❓ 常见问题

#### Q: macOS 上能启动几个实例?

A: macOS 对多开支持更好,可以启动更多实例。建议:
- 8GB 内存: 3-5 个
- 16GB+ 内存: 5-10 个

#### Q: 为什么 macOS 不需要关闭 Mutex?

A: macOS 的应用程序默认不使用独占锁机制,使用 `open -n` 就能打开多个实例。

#### Q: 如何关闭多个实例?

A:
```bash
# 查找所有 WeCom 进程
ps aux | grep WeCom

# 关闭所有
killall WeCom

# 或关闭指定 PID
kill <PID>
```

### 🆚 跨平台对比

| 特性 | Windows | macOS |
|------|---------|-------|
| 多开原理 | 关闭 Mutex | open -n 命令 |
| 需要管理员 | 建议 | 不需要 |
| 实现复杂度 | 高 | 低 |
| 稳定性 | 优秀 | 优秀 |

### 📖 技术细节

macOS 实现代码 (src/lib.rs):

```rust
#[cfg(target_os = "macos")]
pub mod platform {
    pub async fn spawn_multiple(req: SpawnRequest) -> Result<SpawnResponse, String> {
        for i in 0..req.count {
            Command::new("open")
                .arg("-n") // 新实例
                .arg("-a")
                .arg(&app_path)
                .spawn()?;
        }
        Ok(response)
    }
}
```

### 🚧 已知限制

1. **GUI 版本**: macOS GUI 尚未完成,目前仅支持 CLI
2. **进程管理**: 无法像 Windows 版本一样精确管理进程
3. **自动检测**: 需要企业微信安装在标准路径

### 🔮 未来计划

- [ ] macOS 原生 GUI (使用 Tauri)
- [ ] 系统托盘支持
- [ ] 更好的进程管理
- [ ] 配置文件支持

### 💻 开发

```bash
# 运行开发版本
cargo run -- 3

# 查看详细日志
RUST_LOG=debug cargo run -- 3

# 运行测试
cargo test
```

### 📦 分发

编译后的二进制文件可以直接分发:

```bash
# 复制到 /usr/local/bin
sudo cp target/release/wecom-multi-open /usr/local/bin/

# 然后在任何地方运行
wecom-multi-open 3
```

### 🌟 示例

```bash
# 启动 3 个实例
wecom-multi-open 3

# 启动 5 个实例
wecom-multi-open 5

# 使用自定义路径
wecom-multi-open --path ~/Applications/WeCom.app 2
```

---

**macOS 用户现在也能享受多开的便利!** 🎉
