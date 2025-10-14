# v0.3.0 发布总结

## 🎉 重大更新

### 解决的问题

1. ❌ **问题**: GitHub Releases 页面没有可下载的文件
   - ✅ **解决**: 创建 v0.3.0 标签触发自动构建

2. ❌ **问题**: macOS 用户需要命令行操作太复杂
   - ✅ **解决**: 提供 DMG 安装包,双击拖动即可使用

3. ❌ **问题**: Windows 用户需要手动配置
   - ✅ **解决**: 提供 MSI 安装包,双击自动安装

---

## 📦 发布内容

### CLI 版本 (命令行)

**Windows**:
- `wecom-multi-open-cli-windows.exe` (~1.5 MB)
- 使用: 双击运行或命令行 `wecom-multi-open-cli-windows.exe 3`

**macOS Intel**:
- `wecom-multi-open-cli-macos-intel` (~600 KB)
- 使用: `chmod +x wecom-multi-open-cli-macos-intel && ./wecom-multi-open-cli-macos-intel 3`

**macOS M1/M2/M3**:
- `wecom-multi-open-cli-macos-m1` (~600 KB)
- 使用: `chmod +x wecom-multi-open-cli-macos-m1 && ./wecom-multi-open-cli-macos-m1 3`

### GUI 版本 (图形界面) ⭐ 新增

**Windows**:
- `wecom-multi-open_0.3.0_x64.msi` (~8 MB)
- 📥 **双击安装**: 自动安装到程序菜单
- 🚀 **启动**: 从开始菜单搜索"企业微信多开工具"

**macOS Intel**:
- `wecom-multi-open-gui-macos-intel.dmg` (~8 MB)
- 📥 **双击打开**: DMG 文件
- 🖱️ **拖动安装**: 拖动到 Applications 文件夹
- 🚀 **启动**: 从启动台或 Applications 启动

**macOS M1/M2/M3**:
- `wecom-multi-open-gui-macos-m1.dmg` (~8 MB)
- 📥 **双击打开**: DMG 文件
- 🖱️ **拖动安装**: 拖动到 Applications 文件夹
- 🚀 **启动**: 从启动台或 Applications 启动

---

## 🎯 用户体验对比

### 之前 (v0.2.0)

**macOS 用户需要**:
```bash
# 1. 下载文件
curl -L https://github.com/.../wecom-multi-open-macos-m1 -o wecom-multi-open

# 2. 赋予权限
chmod +x wecom-multi-open

# 3. 运行
./wecom-multi-open 3

# 4. (可选) 移到系统路径
sudo mv wecom-multi-open /usr/local/bin/
```

### 现在 (v0.3.0)

**macOS GUI 用户只需**:
```
1. 下载 DMG 文件
2. 双击打开
3. 拖动到 Applications
4. 从启动台点击图标启动
```

**完全图形化操作,无需命令行!** 🎊

---

## ✨ GUI 版本功能

### 主界面
- 🎛️ **实例数量控制**: 滑动条或输入框设置 1-10 个实例
- 🚀 **快捷启动**: 一键启动 2/3/5 个实例
- 📊 **实时监控**: 每 3 秒自动刷新运行状态
- 🔍 **PID 显示**: 显示所有运行中实例的进程 ID

### 系统托盘
- 🖱️ **左键**: 显示/隐藏主窗口
- 🖱️ **右键**: 显示快捷菜单
  - 启动 3 个实例
  - 启动 5 个实例
  - 关闭所有实例
  - 退出程序

### 窗口管理
- ❌ **关闭窗口**: 最小化到托盘 (不退出)
- 🏃 **后台运行**: 托盘中持续监控
- 🎨 **美观界面**: 渐变紫色主题,现代化设计

---

## 🔧 GitHub Actions 更新

### 构建流程

```yaml
jobs:
  build-cli:      # CLI 版本构建
  build-gui:      # GUI 版本构建 (新增)
  release:        # 发布 Release
```

### build-gui 任务内容

1. **安装 Node.js 18**
2. **安装前端依赖** (`npm ci`)
3. **构建前端** (`npm run build`)
4. **安装 Tauri CLI**
5. **构建应用包**:
   - Windows: 生成 MSI 安装包
   - macOS: 生成 DMG 和 .app

### 构建时间预估

- **CLI 版本**: ~3-5 分钟
- **GUI 版本**: ~10-15 分钟 (需要编译前端 + Tauri)
- **总计**: ~15-20 分钟

---

## 📊 文件对比

### 体积

| 版本 | Windows | macOS Intel | macOS M1/M2/M3 |
|------|---------|-------------|----------------|
| **CLI** | 1.5 MB | 600 KB | 600 KB |
| **GUI** | 8 MB | 8 MB | 8 MB |

### 功能

| 功能 | CLI | GUI |
|------|-----|-----|
| 启动多个实例 | ✅ | ✅ |
| 图形界面 | ❌ | ✅ |
| 系统托盘 | ❌ | ✅ |
| 实例管理 | ❌ | ✅ |
| 实时监控 | ❌ | ✅ |
| 快捷操作 | ❌ | ✅ |
| 双击安装 | ❌ | ✅ |

---

## 🚀 快速开始 (用户视角)

### Windows 用户

#### 方式 1: GUI 版本 (推荐)
```
1. 访问: https://github.com/not-yes/wecom-multi/releases
2. 下载: wecom-multi-open_0.3.0_x64.msi
3. 双击安装
4. 从开始菜单启动
```

#### 方式 2: CLI 版本
```
1. 下载: wecom-multi-open-cli-windows.exe
2. 双击运行 (默认 3 个实例)
```

### macOS 用户

#### 方式 1: GUI 版本 (推荐)
```
1. 访问: https://github.com/not-yes/wecom-multi/releases
2. 下载 DMG:
   - M1/M2/M3: wecom-multi-open-gui-macos-m1.dmg
   - Intel: wecom-multi-open-gui-macos-intel.dmg
3. 双击打开 DMG
4. 拖动到 Applications
5. 从启动台启动
```

#### 方式 2: CLI 版本
```
1. 下载对应芯片版本
2. chmod +x wecom-multi-open-cli-macos-*
3. ./wecom-multi-open-cli-macos-* 3
```

#### 确定芯片类型
```bash
uname -m
# arm64 = M1/M2/M3
# x86_64 = Intel
```

---

## 🎯 Release 页面内容

### 下载列表 (共 9 个文件)

**CLI 版本** (3 个):
- wecom-multi-open-cli-windows.exe
- wecom-multi-open-cli-macos-intel
- wecom-multi-open-cli-macos-m1

**GUI 版本** (6 个):
- wecom-multi-open_0.3.0_x64.msi (Windows 安装包)
- wecom-multi-open-gui-macos-intel.dmg (macOS Intel 安装包)
- wecom-multi-open-gui-macos-m1.dmg (macOS M1/M2/M3 安装包)
- wecom-multi-open-gui-macos-intel.app (macOS Intel App)
- wecom-multi-open-gui-macos-m1.app (macOS M1/M2/M3 App)

---

## ⏱️ 构建状态

### 触发条件
- ✅ 已创建标签: `v0.3.0`
- ✅ 已推送到 GitHub
- ✅ GitHub Actions 已触发

### 查看进度
访问: https://github.com/not-yes/wecom-multi/actions

### 预计完成时间
- 开始时间: 标签推送后立即开始
- 预计时长: 15-20 分钟
- Release 发布: 构建完成后自动创建

---

## 📝 更新日志

### v0.3.0 (2025-01-XX)

#### 新增功能
- ✅ GUI 图形界面版本 (Tauri + React)
- ✅ 系统托盘支持
- ✅ 实例实时监控
- ✅ 快捷启动按钮
- ✅ DMG 安装包 (macOS)
- ✅ MSI 安装包 (Windows)

#### 用户体验改进
- ✅ macOS 用户可双击安装,无需命令行
- ✅ Windows 用户可双击安装,自动添加到程序菜单
- ✅ 图形界面更直观易用
- ✅ 系统托盘后台运行

#### 技术改进
- ✅ 分离 CLI 和 GUI 构建流程
- ✅ 自动生成安装包
- ✅ 优化构建缓存
- ✅ 改进 Release 说明

### v0.2.0 (2025-01-XX)

#### 新增功能
- ✅ 跨平台支持 (Windows + macOS)
- ✅ macOS 使用 `open -n` 命令实现
- ✅ 统一的抽象接口
- ✅ 条件编译支持

### v0.1.0 (2025-01-XX)

#### 初始版本
- ✅ Windows CLI 版本
- ✅ PowerShell 备用脚本
- ✅ Mutex 关闭机制

---

## 🔮 下一步计划

### v0.4.0 (计划中)
- [ ] 开机自启功能
- [ ] 配置文件保存
- [ ] 主题切换 (亮色/暗色)
- [ ] 多语言支持 (中英文)
- [ ] 自定义企业微信路径

### v0.5.0 (计划中)
- [ ] 实例配置管理
- [ ] 自动更新检查
- [ ] 性能监控
- [ ] 日志查看

### v1.0.0 (长期)
- [ ] 插件系统
- [ ] REST API 支持
- [ ] 云同步配置
- [ ] 微信多开支持

---

## 📚 相关链接

- **GitHub 仓库**: https://github.com/not-yes/wecom-multi
- **Releases 页面**: https://github.com/not-yes/wecom-multi/releases
- **Actions 构建**: https://github.com/not-yes/wecom-multi/actions
- **Issue 反馈**: https://github.com/not-yes/wecom-multi/issues

---

## 🎉 总结

### 核心改进

1. ✅ **用户体验**:
   - 从"需要命令行"到"双击安装"
   - 从"纯文本"到"图形界面"
   - 从"手动管理"到"自动监控"

2. ✅ **分发方式**:
   - 从"单独二进制"到"安装包"
   - 从"手动权限"到"自动安装"
   - 从"命令行运行"到"图标启动"

3. ✅ **功能完整性**:
   - CLI 版本: 适合技术用户,轻量快速
   - GUI 版本: 适合普通用户,功能完整

### 用户反馈预期

**以前**:
> "需要打开终端输入命令,太复杂了"

**现在**:
> "下载 DMG,拖动安装,点击图标就能用了!"

---

**v0.3.0 - 让企业微信多开变得更简单!** 🚀
