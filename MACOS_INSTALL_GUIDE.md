# macOS 安装指南

## ⚠️ 重要提示

由于应用未经过 Apple 代码签名和公证，macOS 可能会显示"已损坏"的警告。这是正常的安全机制，不是真的损坏。

## 🛠️ 解决方法

### 方法 1: 移除隔离属性（推荐）

1. 下载 DMG 文件并打开
2. 将应用拖到 Applications 文件夹
3. **不要直接双击打开**，先打开终端执行：

```bash
xattr -cr /Applications/wecom-multi-open.app
```

4. 现在可以正常打开应用了

### 方法 2: 首次打开使用右键菜单

1. 下载 DMG 并安装到 Applications
2. 在 Applications 中找到 `wecom-multi-open.app`
3. **右键点击** 应用，选择"打开"
4. 在弹出的警告中点击"打开"
5. 之后就可以正常双击打开了

### 方法 3: 允许任何来源的应用（不推荐）

```bash
# 临时禁用 Gatekeeper
sudo spctl --master-disable

# 使用完后记得重新启用
sudo spctl --master-enable
```

然后在"系统偏好设置" > "安全性与隐私" > "通用"中选择"任何来源"

## 🔍 为什么会这样？

- 应用未经过 Apple 开发者签名
- 未经过 Apple 公证（notarization）
- macOS Gatekeeper 安全机制默认阻止运行

## ✅ 安全性说明

- 本项目是**开源**的，代码完全透明
- 可以在 GitHub 上查看所有源代码
- 构建过程通过 GitHub Actions 公开可见
- 不包含任何恶意代码

## 📱 CLI 版本（备选）

如果 GUI 版本遇到问题，可以使用 CLI 版本：

```bash
# 下载 CLI 版本
chmod +x wecom-multi-open-cli-macos-m1  # 或 macos-intel

# 运行（启动 3 个实例）
./wecom-multi-open-cli-macos-m1 3
```

CLI 版本同样需要执行 `xattr -cr` 命令：

```bash
xattr -cr wecom-multi-open-cli-macos-m1
```

## 🔮 未来计划

我们计划在未来版本中：
- [ ] 申请 Apple 开发者账号（需要 $99/年）
- [ ] 对应用进行代码签名
- [ ] 提交 Apple 公证
- [ ] 提供完全认证的版本

在此之前，请使用上述方法临时解决。

## ❓ 常见问题

**Q: 执行 xattr 命令后还是打不开？**

A: 尝试完全卸载后重新安装：
```bash
rm -rf /Applications/wecom-multi-open.app
# 重新从 DMG 安装
xattr -cr /Applications/wecom-multi-open.app
```

**Q: 提示"无法验证开发者"？**

A: 这是正常的，使用方法 2（右键打开）即可。

**Q: 是否安全？**

A: 完全安全。所有代码开源，构建过程透明。只是没有 Apple 官方认证而已。

---

**如有问题，请访问**: https://github.com/not-yes/wecom-multi/issues
