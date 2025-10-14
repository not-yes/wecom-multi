# Windows 版企业微信多开工具

## 🎯 特性

- ✅ **真正的多开**: 使用 Sandboxie-Plus 沙盒技术,每个实例完全隔离
- ✅ **账号独立**: 每个实例可登录不同账号,互不干扰
- ✅ **可视化区分**: 不同颜色边框标识不同实例
- ✅ **图形界面**: 简洁易用的 GUI 操作
- ✅ **自动管理**: 一键启动/停止,自动清理

## 📋 前置要求

### 1. Sandboxie-Plus (必需)

**下载安装**: https://sandboxie-plus.com/downloads/

选择"Sandboxie-Plus"版本(推荐),或经典版"Sandboxie"

**安装要求**:
- 安装到默认路径: `C:\Program Files\Sandboxie-Plus\`
- 确保安装完成后服务正常运行

### 2. 企业微信

正常安装企业微信到系统,程序会自动检测安装路径。

支持的默认路径:
- `C:\Program Files (x86)\WXWork\WXWork.exe`
- `C:\Program Files\WXWork\WXWork.exe`

## 🚀 快速开始

### 方式 1: 下载预编译版本 (推荐)

1. 访问 [Releases 页面](../../releases)
2. 下载最新的 Windows 版本
   - `.msi` - 安装包(推荐)
   - `.exe` - 便携版
3. 运行程序

### 方式 2: 从源码编译

```powershell
# 克隆仓库
git clone <repo-url>
cd mutil_wechat

# 切换到 Windows 分支
git checkout feature/windows-support

# 编译 (需要安装 Rust 和 Node.js)
npm install
npm run tauri build

# 产物位置
# target/release/wecom-multi-open.exe
```

## 💻 使用方法

### GUI 版本

1. 启动程序 `wecom-multi-open.exe`
2. 在界面上选择要启动的实例数量
3. 点击"启动实例"按钮
4. 等待实例逐个启动(每个间隔 2 秒)
5. 在每个企业微信窗口登录不同账号

**界面说明**:
- 实例列表显示当前运行的所有实例
- 每个实例有不同颜色的沙盒边框
- 可以单独停止某个实例
- 可以一键停止所有实例

### CLI 版本 (高级)

```powershell
# 启动 3 个实例
wecom-multi-open-cli 3

# 查看帮助
wecom-multi-open-cli --help
```

## 🔧 工作原理

### 技术架构

```
┌─────────────────────────────────────────┐
│         Tauri Frontend (React)          │
│  - 用户界面                              │
│  - 实例管理                              │
└─────────────┬───────────────────────────┘
              │ Tauri IPC
┌─────────────▼───────────────────────────┐
│         Rust Backend                    │
│  - Sandboxie 集成                       │
│  - 进程管理                              │
└─────────────┬───────────────────────────┘
              │ CLI Commands
┌─────────────▼───────────────────────────┐
│         Sandboxie-Plus                  │
│  - 沙盒隔离                              │
│  - 数据隔离                              │
└─────────────────────────────────────────┘
```

### 核心机制

1. **沙盒创建**: 为每个实例创建独立的 Sandboxie 沙盒
   ```
   WeCom_1, WeCom_2, WeCom_3, ...
   ```

2. **隔离启动**: 在各自沙盒中启动企业微信
   ```powershell
   Start.exe /box:WeCom_1 "C:\Program Files\WXWork\WXWork.exe"
   ```

3. **数据隔离**: 每个沙盒有独立的:
   - 注册表视图
   - 文件系统视图
   - 进程空间
   - 网络连接

4. **可视化**: 不同颜色边框标识
   - 实例 1: 红色边框
   - 实例 2: 绿色边框
   - 实例 3: 蓝色边框
   - ...

## 📊 性能参考

| 实例数 | 内存占用 | 启动时间 | 推荐配置 |
|--------|----------|---------|---------|
| 2      | ~1.5 GB  | 10秒    | 4GB RAM |
| 3      | ~2.2 GB  | 15秒    | 8GB RAM |
| 5      | ~3.5 GB  | 25秒    | 8GB RAM |
| 10     | ~7 GB    | 50秒    | 16GB RAM|

**注意**:
- 每个企业微信实例约占用 350-500 MB 内存
- 启动时间取决于系统配置和磁盘速度
- 建议不要同时启动超过 10 个实例

## ⚠️ 常见问题

### Q1: 提示"未找到 Sandboxie-Plus 安装"

**解决方案**:
1. 确认已安装 Sandboxie-Plus
2. 检查安装路径是否为默认路径
3. 确认这两个文件存在:
   - `C:\Program Files\Sandboxie-Plus\SbieIni.exe`
   - `C:\Program Files\Sandboxie-Plus\Start.exe`

### Q2: 提示"未找到企业微信安装路径"

**解决方案**:
1. 确认已正常安装企业微信
2. 检查是否在以下路径:
   - `C:\Program Files (x86)\WXWork\WXWork.exe`
   - `C:\Program Files\WXWork\WXWork.exe`
3. 如果安装在其他路径,可以在设置中指定

### Q3: 沙盒创建失败

**解决方案**:
1. 以管理员身份运行程序
2. 检查 Sandboxie 服务是否运行:
   - 打开"服务"(services.msc)
   - 找到"SbieSvc"服务
   - 确保状态为"正在运行"
3. 重启 Sandboxie Control

### Q4: 实例启动后立即关闭

**解决方案**:
1. 检查企业微信路径是否正确
2. 手动测试 Sandboxie:
   ```powershell
   cd "C:\Program Files\Sandboxie-Plus"
   .\Start.exe /box:TestBox "C:\Program Files\WXWork\WXWork.exe"
   ```
3. 查看 Sandboxie Control 中的错误日志

### Q5: 实例登录后相互踢出

**解决方案**:
这不应该发生!如果出现此问题:
1. 确认每个实例在不同的沙盒中运行
2. 检查 Sandboxie Control,查看沙盒列表
3. 可能是 Sandboxie 配置问题,尝试重新安装 Sandboxie-Plus

### Q6: 性能问题/卡顿

**解决方案**:
1. 减少同时运行的实例数量
2. 增加系统内存
3. 关闭不必要的后台程序
4. 使用 SSD 硬盘

## 🛡️ 安全与隐私

- ✅ 所有数据本地存储,不上传
- ✅ 使用开源的 Sandboxie-Plus
- ✅ 不修改企业微信程序本身
- ✅ 不收集任何用户信息

## 🆚 与其他方案对比

| 方案 | 数据隔离 | 稳定性 | 账号互踢 | 实现复杂度 |
|------|---------|--------|---------|-----------|
| **Sandboxie** | ✅ 完全隔离 | ⭐⭐⭐⭐⭐ | ❌ 不会 | 简单 |
| 修改 Mutex | ⚠️ 共享数据 | ⭐⭐⭐ | ✅ 会 | 中等 |
| 虚拟机 | ✅ 完全隔离 | ⭐⭐⭐⭐ | ❌ 不会 | 复杂 |
| Android 模拟器 | ✅ 完全隔离 | ⭐⭐⭐⭐ | ❌ 不会 | 简单 |

## 📖 相关文档

- [开发方案](docs/Windows版本开发方案.md) - 技术实现细节
- [测试指南](docs/Windows测试指南.md) - 测试流程和问题排查
- [跨平台研究](docs/跨平台多开方案深度研究报告.md) - 方案对比分析

## 🤝 贡献

欢迎提交 Issue 和 Pull Request!

### 开发流程

1. Fork 仓库
2. 创建特性分支: `git checkout -b feature/my-feature`
3. 提交更改: `git commit -am 'Add some feature'`
4. 推送分支: `git push origin feature/my-feature`
5. 提交 Pull Request

### 编译要求

- Rust 1.70+
- Node.js 18+
- Windows 10/11
- Sandboxie-Plus (开发测试)

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

**依赖项许可证**:
- Sandboxie-Plus: GPL-3.0
- Tauri: MIT/Apache-2.0
- Rust crates: 见各自许可证

## ⚖️ 免责声明

- 本工具仅供学习和合法合规使用
- 用户应遵守企业微信服务协议
- 作者不对使用本工具造成的任何后果负责
- 请勿用于商业用途或非法活动

## 🙏 致谢

- [Sandboxie-Plus](https://github.com/sandboxie-plus/Sandboxie) - 核心沙盒技术
- [Tauri](https://tauri.app/) - 跨平台 GUI 框架
- [Rust](https://www.rust-lang.org/) - 系统编程语言

---

**版本**: v0.3.0-alpha
**平台**: Windows 10/11 (x64)
**更新**: 2024-01
