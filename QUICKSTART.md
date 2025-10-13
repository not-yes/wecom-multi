# 快速开始指南

## 5 分钟上手企业微信多开工具

### 前置条件

确保你已安装:

1. **Node.js** (>= 18)
   ```bash
   node --version
   ```

2. **Rust** (>= 1.70) - 仅在 Windows 上构建时需要
   ```bash
   rustc --version
   ```

3. **企业微信** - 已安装在 Windows 系统上

### 方法一: 使用 PowerShell 脚本 (推荐,最简单)

**适用场景**: 快速测试,无需编译

```powershell
# 1. 打开 PowerShell
# 2. 导航到项目目录
cd scripts

# 3. 启动 2 个企业微信实例
.\wecom_multi_open.ps1 -Count 2

# 或启动 3 个实例
.\wecom_multi_open.ps1 -Count 3

# 或指定路径
.\wecom_multi_open.ps1 -Count 2 -WeComPath "C:\Path\To\WXWork.exe"
```

**遇到权限问题?**
```powershell
# 临时允许脚本执行
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass

# 然后再运行脚本
.\wecom_multi_open.ps1 -Count 2
```

### 方法二: 使用 GUI 应用 (完整功能)

**适用场景**: 需要图形界面和进程管理

#### Step 1: 安装依赖

```bash
npm install
```

#### Step 2: 开发模式运行

```bash
npm run tauri:dev
```

这会:
- 启动 Vite 开发服务器
- 编译 Rust 代码
- 打开应用窗口

#### Step 3: 使用应用

1. 在"启动实例数量"输入框输入数字 (如 `2`)
2. 点击"启动多开"按钮
3. 等待启动完成
4. 查看运行中的实例列表

#### Step 4: 构建生产版本 (可选)

```bash
npm run tauri:build
```

构建输出位于: `src-tauri/target/release/bundle/`

## 常见问题

### Q1: 脚本无法运行,提示禁止执行脚本

**解决**:
```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
```

### Q2: npm install 失败

**解决**:
```bash
# 清理缓存
npm cache clean --force

# 重新安装
npm install
```

### Q3: 企业微信路径未找到

**解决**:
- 确认企业微信已安装
- 手动指定路径: `-WeComPath "C:\Program Files\WXWork\WXWork.exe"`
- 或在 GUI 中点击"浏览"按钮选择

### Q4: 启动后立即退出

**可能原因**:
- 企业微信版本不兼容
- 路径错误
- 权限不足

**解决**:
1. 以管理员身份运行
2. 检查企业微信版本
3. 查看系统事件日志

### Q5: Rust 编译错误

**解决**:

Windows 需要安装:
- Visual Studio Build Tools
- Windows 10 SDK

下载地址: https://visualstudio.microsoft.com/downloads/

## 使用技巧

### Tip 1: 建议启动数量

根据电脑配置:
- **4GB 内存**: 2-3 个实例
- **8GB 内存**: 3-5 个实例
- **16GB+ 内存**: 5-10 个实例

### Tip 2: 分批启动

如需启动多个实例,建议分批:
```powershell
# 第一批
.\wecom_multi_open.ps1 -Count 3

# 等待 30 秒

# 第二批
.\wecom_multi_open.ps1 -Count 3
```

### Tip 3: 创建快捷方式

**PowerShell 脚本快捷方式**:

1. 右键桌面 > 新建 > 快捷方式
2. 位置输入:
   ```
   powershell.exe -ExecutionPolicy Bypass -File "C:\path\to\scripts\wecom_multi_open.ps1" -Count 2
   ```
3. 命名为"企业微信多开"

### Tip 4: 监控资源

使用任务管理器 (Ctrl+Shift+Esc) 监控:
- 内存使用
- CPU 占用
- 网络流量

## 下一步

- 阅读 [用户指南](docs/USER_GUIDE.md) 了解详细功能
- 阅读 [开发文档](docs/DEVELOPMENT.md) 了解技术细节
- 查看 [贡献指南](CONTRIBUTING.md) 参与开发

## 需要帮助?

- 查看 [用户指南](docs/USER_GUIDE.md)
- 提交 [GitHub Issue](https://github.com/yourusername/wecom-multi-open/issues)
- 在 [Discussions](https://github.com/yourusername/wecom-multi-open/discussions) 中提问

---

**祝使用愉快!** 🎉
