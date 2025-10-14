# Windows 本地构建和使用指南

## 🚀 快速开始 (3 步完成)

### 准备工作

确保你的 Windows 电脑已安装:

1. **Node.js** (必需)
   - 下载: https://nodejs.org/
   - 选择 LTS 版本
   - 默认安装即可

2. **Rust** (必需)
   - 下载: https://rustup.rs/
   - 或运行: `winget install Rustlang.Rustup`
   - 安装后重启终端

3. **Visual Studio Build Tools** (必需)
   - 下载: https://visualstudio.microsoft.com/downloads/
   - 选择 "Build Tools for Visual Studio 2022"
   - 勾选 "Desktop development with C++"
   - 安装需要 5-10 分钟

## 📦 方式 1: 直接使用 PowerShell 脚本 (最简单,无需构建)

**适合**: 快速测试,立即使用

```powershell
# 1. 下载或克隆项目到本地
# 2. 打开 PowerShell,进入项目目录
cd C:\path\to\mutil_wechat

# 3. 进入 scripts 目录
cd scripts

# 4. 允许脚本执行
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass

# 5. 启动企业微信多开 (启动 2 个实例)
.\wecom_multi_open.ps1 -Count 2

# 启动 3 个实例
.\wecom_multi_open.ps1 -Count 3

# 指定企业微信路径
.\wecom_multi_open.ps1 -Count 2 -WeComPath "C:\Program Files\WXWork\WXWork.exe"
```

**优点**:
- ✅ 无需任何编译
- ✅ 立即可用
- ✅ 修改方便

**缺点**:
- ❌ 无图形界面
- ❌ 功能有限

---

## 🖥️ 方式 2: 构建 GUI 应用 (完整功能)

**适合**: 需要图形界面和完整功能

### Step 1: 克隆或下载项目

```powershell
# 克隆项目 (如果使用 Git)
git clone https://github.com/yourusername/wecom-multi-open.git
cd wecom-multi-open

# 或者直接下载 ZIP 解压
```

### Step 2: 安装依赖

```powershell
# 在项目根目录运行
npm install

# 等待安装完成 (需要 2-5 分钟)
```

### Step 3: 生成临时图标 (可选)

```powershell
# 安装 Python Pillow 库
pip install pillow

# 运行图标生成脚本
python scripts/generate_temp_icon.py
```

### Step 4: 开发模式运行 (测试)

```powershell
# 启动开发模式
npm run tauri:dev

# 会自动编译 Rust 代码并启动应用
# 首次编译需要 5-10 分钟
```

**如果成功**: 会弹出应用窗口,可以立即使用

**如果失败**: 查看错误信息,可能需要:
- 重启终端 (让 Rust 环境生效)
- 检查是否安装了 Visual Studio Build Tools
- 运行 `rustc --version` 验证 Rust 已安装

### Step 5: 构建安装包 (正式使用)

```powershell
# 构建生产版本
npm run tauri:build

# 构建过程:
# 1. 编译 Rust 代码 (5-10 分钟)
# 2. 构建前端 (1-2 分钟)
# 3. 打包安装程序 (1 分钟)
```

### Step 6: 安装使用

构建完成后,找到安装包:

```
src-tauri\target\release\bundle\
├── msi\
│   └── 企业微信多开工具_0.1.0_x64_zh-CN.msi  ← 双击安装
└── nsis\
    └── 企业微信多开工具_0.1.0_x64-setup.exe  ← 或这个
```

**双击 MSI 文件** → 按向导安装 → 完成!

安装后:
- 开始菜单搜索 "企业微信多开工具"
- 或从安装目录启动

---

## 🔧 构建常见问题

### Q1: `npm install` 失败

**错误**: `npm ERR! network timeout`

**解决**:
```powershell
# 使用国内镜像
npm config set registry https://registry.npmmirror.com
npm install
```

### Q2: Rust 未找到

**错误**: `cargo: command not found`

**解决**:
```powershell
# 1. 确认已安装 Rust
rustup --version

# 2. 如未安装,下载安装:
# https://rustup.rs/

# 3. 安装后重启 PowerShell
```

### Q3: 编译报错 - 找不到 MSVC

**错误**: `error: linker 'link.exe' not found`

**解决**:
1. 安装 Visual Studio Build Tools
2. 勾选 "Desktop development with C++"
3. 重启电脑

### Q4: 构建时间太长

**原因**: 首次构建需要下载和编译大量依赖

**优化**:
```powershell
# 使用 Rust 国内镜像
# 编辑文件: C:\Users\你的用户名\.cargo\config

[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

### Q5: WebView2 未找到

**错误**: `WebView2 runtime not found`

**解决**:
1. 下载: https://developer.microsoft.com/microsoft-edge/webview2/
2. 安装 "Evergreen Bootstrapper"
3. 重启应用

---

## 📋 完整构建步骤总结

```powershell
# 1. 安装前置软件
# - Node.js (必需)
# - Rust (必需)
# - Visual Studio Build Tools (必需)

# 2. 下载项目
cd C:\Projects
git clone [项目地址]
cd wecom-multi-open

# 3. 安装依赖
npm install

# 4. 生成图标 (可选)
pip install pillow
python scripts/generate_temp_icon.py

# 5. 测试运行
npm run tauri:dev

# 6. 构建安装包
npm run tauri:build

# 7. 安装使用
cd src-tauri\target\release\bundle\msi
# 双击 .msi 文件安装

# 8. 启动使用
# 从开始菜单搜索并启动
```

---

## ⚡ 推荐流程

### 对于急用者:

```powershell
# 直接使用 PowerShell 脚本
cd scripts
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
.\wecom_multi_open.ps1 -Count 2
```

### 对于长期使用者:

```powershell
# 构建安装包,一次构建永久使用
npm install
npm run tauri:build
# 然后安装生成的 MSI 文件
```

---

## 📝 使用建议

1. **首次使用**: 先用 PowerShell 脚本测试功能
2. **确认可用**: 再花时间构建 GUI 版本
3. **安装后**: 可以删除源代码,只保留安装的程序

---

## 🎯 最终目标

安装后你会得到:
- 一个可以双击启动的桌面应用
- 友好的图形界面
- 完整的进程管理功能
- 出现在开始菜单中

**就像使用其他 Windows 软件一样简单!**

---

有问题随时查看文档或反馈!
