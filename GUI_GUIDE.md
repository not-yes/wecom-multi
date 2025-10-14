# GUI 版本使用指南

## 🎨 GUI 特性

企业微信多开工具 GUI 版本提供了更友好的图形界面和系统托盘支持。

### ✨ 主要功能

- ✅ **图形界面** - 基于 Tauri + React 开发
- ✅ **系统托盘** - 最小化到托盘,后台运行
- ✅ **实时监控** - 自动显示当前运行的实例数量
- ✅ **快捷操作** - 一键启动 2/3/5 个实例
- ✅ **实例管理** - 查看和关闭所有运行中的实例
- ✅ **跨平台** - Windows 和 macOS 双平台支持

## 📦 安装依赖

### 1. 安装 Rust (如果还没有)

```bash
# Windows
winget install Rustlang.Rustup

# macOS / Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 安装 Node.js

```bash
# Windows
winget install OpenJS.NodeJS

# macOS
brew install node

# 或从官网下载: https://nodejs.org/
```

### 3. 安装前端依赖

```bash
cd ui
npm install
```

## 🚀 开发模式

### 启动开发服务器

```bash
# 1. 启动前端开发服务器 (终端 1)
cd ui
npm run dev

# 2. 启动 Tauri 开发模式 (终端 2)
cargo tauri dev
# 或
cargo run --bin wecom-multi-open-gui --features gui
```

开发模式特点:
- ✅ 热重载 - 前端代码修改自动刷新
- ✅ 调试工具 - 打开浏览器开发者工具
- ✅ 实时日志 - 查看后端日志输出

## 🔨 构建 Release 版本

### Windows

```bash
# 1. 构建前端
cd ui
npm run build

# 2. 构建 GUI 应用
cargo build --release --bin wecom-multi-open-gui --features gui

# 3. 输出位置
# target/release/wecom-multi-open-gui.exe
```

### macOS

```bash
# 1. 构建前端
cd ui
npm run build

# 2. 构建 GUI 应用
cargo build --release --bin wecom-multi-open-gui --features gui

# 3. 输出位置
# target/release/wecom-multi-open-gui
```

### 使用 Tauri CLI (推荐)

```bash
# 安装 Tauri CLI
cargo install tauri-cli

# 构建应用包
cargo tauri build

# 输出位置:
# Windows: target/release/bundle/msi/wecom-multi-open_0.2.0_x64.msi
# macOS: target/release/bundle/macos/wecom-multi-open.app
# macOS DMG: target/release/bundle/dmg/wecom-multi-open_0.2.0_x64.dmg
```

## 🎯 使用说明

### 主界面

1. **设置实例数量**
   - 输入框输入 1-10 的数字
   - 点击"启动实例"按钮

2. **快捷启动**
   - 点击"快速启动 2 个"
   - 点击"快速启动 3 个"
   - 点击"快速启动 5 个"

3. **查看运行状态**
   - 实时显示运行中的实例数量
   - 显示所有实例的进程 PID

4. **关闭实例**
   - 点击"关闭所有实例"按钮
   - 或在系统托盘菜单中选择"关闭所有实例"

### 系统托盘

右键点击托盘图标:

```
├─ 显示窗口
├─ ──────────
├─ 启动 3 个实例
├─ 启动 5 个实例
├─ 关闭所有实例
├─ ──────────
└─ 退出
```

左键点击托盘图标: 显示主窗口

### 窗口行为

- **关闭窗口**: 最小化到托盘,不退出程序
- **退出程序**: 通过托盘菜单选择"退出"

## 🆚 CLI vs GUI 对比

| 特性 | CLI 版本 | GUI 版本 |
|------|---------|---------|
| 体积 | ~1.5 MB | ~8 MB |
| 界面 | 命令行 | 图形界面 |
| 系统托盘 | ❌ | ✅ |
| 实例管理 | ❌ | ✅ |
| 实时监控 | ❌ | ✅ |
| 开机自启 | ❌ | ⏳ 计划中 |

## 🛠️ 技术栈

### 后端
- **Rust** - 主要编程语言
- **Tauri 1.5** - 应用框架
- **Tokio** - 异步运行时
- **windows-rs** - Windows API 绑定

### 前端
- **React 18** - UI 框架
- **TypeScript** - 类型安全
- **Vite** - 构建工具
- **@tauri-apps/api** - Tauri API 绑定

## 📂 项目结构

```
wecom-multi-open/
├── src/
│   ├── lib.rs              # 核心库
│   ├── main.rs             # CLI 版本
│   └── gui.rs              # GUI 版本 ⭐
├── ui/                     # React 前端 ⭐
│   ├── src/
│   │   ├── App.tsx         # 主组件
│   │   ├── App.css         # 样式
│   │   └── main.tsx        # 入口
│   ├── package.json
│   └── vite.config.ts
├── icons/                  # 应用图标 ⭐
│   ├── icon.png
│   ├── icon.ico
│   └── icon.icns
├── tauri.conf.json         # Tauri 配置 ⭐
├── build.rs                # 构建脚本 ⭐
└── Cargo.toml
```

## 🐛 常见问题

### 1. 编译失败: "tauri not found"

确保已安装 Tauri 依赖:

```bash
cargo clean
cargo build --features gui
```

### 2. 前端启动失败

```bash
cd ui
rm -rf node_modules package-lock.json
npm install
npm run dev
```

### 3. Windows 上无法构建 MSI

安装 WiX Toolset:

```powershell
# 使用 winget
winget install WiXToolset.WiX

# 或从官网下载
# https://wixtoolset.org/
```

### 4. macOS 上图标不显示

确保图标文件存在:

```bash
ls -la icons/
# 应该包含: icon.icns, icon.png 等
```

### 5. 系统托盘图标不显示

确认 `tauri.conf.json` 中配置正确:

```json
{
  "tauri": {
    "systemTray": {
      "iconPath": "icons/icon.png",
      "iconAsTemplate": true
    }
  }
}
```

## 🔮 未来计划

- [ ] **开机自启** - 系统启动时自动运行
- [ ] **配置文件** - 保存用户偏好设置
- [ ] **主题切换** - 支持亮色/暗色主题
- [ ] **多语言** - 中文/英文界面切换
- [ ] **更新检查** - 自动检查新版本
- [ ] **实例配置** - 为不同实例设置独立配置
- [ ] **日志查看** - 查看应用运行日志

## 📝 开发提示

### 调试技巧

```bash
# 1. 开启详细日志
RUST_LOG=debug cargo run --bin wecom-multi-open-gui --features gui

# 2. 前端调试
# 在开发模式下按 F12 打开 DevTools

# 3. 查看 Tauri 进程
# Windows: tasklist | findstr wecom
# macOS: ps aux | grep wecom
```

### 性能优化

```toml
# Cargo.toml
[profile.release]
opt-level = "z"          # 优化体积
lto = true               # 链接时优化
codegen-units = 1        # 单个代码生成单元
strip = true             # 移除符号
```

### 代码格式化

```bash
# Rust 代码
cargo fmt

# 前端代码
cd ui
npm run lint
npm run format
```

## 🤝 贡献指南

欢迎贡献代码!

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

---

**享受更优雅的多开体验!** 🎉
