# Windows 发布版本构建指南

## 目标

构建一个可以直接双击使用的 Windows 安装包 (.msi 或 .exe)

## 构建步骤

### 1. 准备 Windows 环境

在 Windows 10/11 系统上:

```powershell
# 1. 安装 Node.js
# 下载: https://nodejs.org/
# 选择 LTS 版本 (18.x 或更高)

# 2. 安装 Rust
# 下载: https://rustup.rs/
# 或直接运行:
winget install Rustlang.Rustup

# 3. 安装 Visual Studio Build Tools
# 下载: https://visualstudio.microsoft.com/downloads/
# 选择 "Desktop development with C++"
```

### 2. 克隆并安装依赖

```powershell
# 克隆项目
git clone https://github.com/yourusername/wecom-multi-open.git
cd wecom-multi-open

# 安装依赖
npm install
```

### 3. 创建应用图标

在构建前需要准备图标文件:

```
src-tauri/icons/
├── 32x32.png
├── 128x128.png
├── 128x128@2x.png
├── icon.ico      # Windows 图标
└── icon.icns     # macOS 图标 (可选)
```

快速生成图标:
```powershell
# 使用在线工具转换
# 1. 准备一个 1024x1024 的 PNG 图片
# 2. 访问 https://icon.kitchen/
# 3. 上传图片并下载所有尺寸
# 4. 复制到 src-tauri/icons/ 目录
```

### 4. 配置打包选项

编辑 `src-tauri/tauri.conf.json`:

```json
{
  "package": {
    "productName": "企业微信多开工具",
    "version": "1.0.0"
  },
  "tauri": {
    "bundle": {
      "active": true,
      "targets": ["msi", "nsis"],
      "identifier": "com.wecom.multiopen",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": ""
      }
    }
  }
}
```

### 5. 构建发布版本

```powershell
# 构建生产版本
npm run tauri:build

# 构建过程需要 5-10 分钟
# 进度输出:
# - Compiling Rust dependencies...
# - Building frontend...
# - Bundling application...
# - Creating installers...
```

### 6. 查找构建产物

构建完成后,安装包位于:

```
src-tauri/target/release/bundle/
├── msi/
│   └── 企业微信多开工具_1.0.0_x64_zh-CN.msi    # MSI 安装包
└── nsis/
    └── 企业微信多开工具_1.0.0_x64-setup.exe    # NSIS 安装包
```

## 构建选项详解

### MSI 安装包 (推荐)

**优点**:
- Windows 标准安装格式
- 支持企业部署
- 支持静默安装
- 更受信任

**大小**: 约 5-8 MB

**使用**:
```powershell
# 双击安装
企业微信多开工具_1.0.0_x64_zh-CN.msi

# 或命令行静默安装
msiexec /i 企业微信多开工具_1.0.0_x64_zh-CN.msi /quiet
```

### NSIS 安装包

**优点**:
- 更小的体积
- 自定义安装界面
- 支持更多选项

**大小**: 约 4-6 MB

### 绿色版 (可选)

如果想要免安装版本,可以直接分发 .exe 文件:

```
src-tauri/target/release/
└── 企业微信多开工具.exe    # 单个可执行文件
```

复制此文件即可在任何 Windows 电脑上运行 (需要 WebView2 运行时)。

## 代码签名 (可选但推荐)

### 为什么需要签名?

- 避免 Windows SmartScreen 警告
- 避免杀毒软件误报
- 提升用户信任

### 获取代码签名证书

1. **购买证书** (约 $100-300/年):
   - DigiCert
   - Sectigo
   - GlobalSign

2. **使用开源证书** (适合个人):
   - Let's Encrypt (仅支持网站)
   - 自签名证书 (会有警告)

### 签名步骤

```powershell
# 1. 获取证书文件 (.pfx)

# 2. 签名 MSI
signtool sign /f "certificate.pfx" /p "password" /t http://timestamp.digicert.com "企业微信多开工具.msi"

# 3. 验证签名
signtool verify /pa "企业微信多开工具.msi"
```

### 自动化签名

在 `tauri.conf.json` 中配置:

```json
{
  "tauri": {
    "bundle": {
      "windows": {
        "certificateThumbprint": "YOUR_CERT_THUMBPRINT",
        "digestAlgorithm": "sha256",
        "timestampUrl": "http://timestamp.digicert.com"
      }
    }
  }
}
```

## 测试安装包

### 基础测试

```powershell
# 1. 双击 MSI 安装包
# 2. 按照安装向导完成安装
# 3. 从开始菜单启动应用
# 4. 测试所有功能
```

### 兼容性测试

测试环境:
- [ ] Windows 10 (1809+)
- [ ] Windows 11
- [ ] 全新系统 (无开发环境)
- [ ] 虚拟机测试

### 安全测试

- [ ] Windows Defender 扫描
- [ ] SmartScreen 测试
- [ ] 第三方杀毒软件测试

## 发布到 GitHub Releases

### 1. 创建 Release

```powershell
# 1. 在 GitHub 仓库创建新 Release
# 2. 创建标签: v1.0.0
# 3. 填写更新日志
# 4. 上传构建产物:
#    - 企业微信多开工具_1.0.0_x64_zh-CN.msi
#    - 企业微信多开工具_1.0.0_x64-setup.exe
# 5. 发布 Release
```

### 2. 添加下载说明

在 Release 描述中添加:

```markdown
## 下载说明

### 推荐下载

- **MSI 安装包** (推荐): `企业微信多开工具_1.0.0_x64_zh-CN.msi`
  - 适合所有 Windows 用户
  - 标准安装流程
  - 自动添加到开始菜单

### 系统要求

- Windows 10 (1809+) 或 Windows 11
- 需要安装 Microsoft Edge WebView2 (通常已预装)

### 安装步骤

1. 下载 MSI 安装包
2. 双击运行安装程序
3. 按照向导完成安装
4. 从开始菜单启动应用

### 首次运行

- 应用会自动检测企业微信安装路径
- 如果检测失败,请手动选择路径
- 建议启动 2-5 个实例

## 更新内容

- [x] 基础多开功能
- [x] 进程管理
- [x] GUI 界面
- [x] PowerShell 脚本支持
```

## 自动化构建 (GitHub Actions)

创建 `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install dependencies
        run: npm install

      - name: Build
        run: npm run tauri:build

      - name: Upload Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            src-tauri/target/release/bundle/msi/*.msi
            src-tauri/target/release/bundle/nsis/*.exe
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

推送标签即可自动构建:
```powershell
git tag v1.0.0
git push origin v1.0.0
```

## WebView2 运行时

应用需要 Microsoft Edge WebView2 运行时。

### 检查是否已安装

```powershell
# 检查注册表
reg query "HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}"
```

### 包含 WebView2

**选项 1**: 在线安装 (推荐)
- 首次运行时自动下载 (~100 KB)
- 需要网络连接

**选项 2**: 离线安装包
- 将 WebView2 Runtime 打包
- 安装包增加 ~100 MB

在 `tauri.conf.json` 中配置:

```json
{
  "tauri": {
    "bundle": {
      "windows": {
        "webviewInstallMode": {
          "type": "downloadBootstrapper"
        }
      }
    }
  }
}
```

## 常见问题

### Q: 构建失败,提示找不到 Visual Studio

**A**: 安装 Visual Studio Build Tools 并选择 "Desktop development with C++"

### Q: MSI 安装后无法启动

**A**: 检查是否安装了 WebView2 Runtime

### Q: Windows Defender 拦截

**A**:
1. 添加到排除列表
2. 获取代码签名证书
3. 提交样本到 Microsoft

### Q: 如何更新版本号?

**A**: 同时更新以下文件:
- `package.json` -> `"version"`
- `src-tauri/Cargo.toml` -> `version`
- `src-tauri/tauri.conf.json` -> `"package.version"`

## 最终交付清单

- [ ] MSI 安装包
- [ ] NSIS 安装包 (可选)
- [ ] 代码签名 (推荐)
- [ ] README.md
- [ ] CHANGELOG.md
- [ ] GitHub Release
- [ ] 安装说明
- [ ] 卸载说明

## 用户使用流程

1. **下载**: 从 GitHub Releases 下载 MSI
2. **安装**: 双击 MSI,按向导安装
3. **启动**: 从开始菜单找到"企业微信多开工具"
4. **使用**: 输入实例数量,点击"启动多开"
5. **卸载**: 设置 > 应用 > 卸载

---

按照此文档构建后,用户只需双击安装包即可使用!
