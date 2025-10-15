# 使用指南

## 快速开始

### 1. 选择应用类型

在GUI界面顶部选择要多开的应用:
- **企业微信**: 适用于公司办公场景
- **个人微信**: 适用于个人社交场景

### 2. 选择隔离模式 (仅Windows)

#### 简单模式 (默认)
- ✅ 无需额外软件
- ✅ 启动速度快
- ❌ 所有实例共享数据目录
- ❌ 可能出现数据冲突

**适用场景**: 快速测试、临时使用

#### 沙盒隔离模式 (推荐)
- ✅ 完全数据隔离
- ✅ 安全可靠
- ⚠️ 需要安装 [Sandboxie-Plus](https://github.com/sandboxie-plus/Sandboxie/releases)
- ⚠️ 启动速度略慢

**适用场景**: 长期使用、多账号管理、企业环境

### 3. 设置实例数量

使用 `-` 和 `+` 按钮调整要启动的实例数量 (1-20个)

### 4. 启动实例

点击"启动"按钮,等待实例启动完成

### 5. 管理实例

- **刷新**: 点击刷新按钮查看当前运行状态
- **关闭单个**: 点击实例右侧的垃圾桶图标
- **全部停止**: 点击"全部停止"按钮

---

## macOS 特别说明

macOS版本**自动启用完全数据隔离**,无需手动配置:

- ✅ 每个实例自动分配独立数据目录
- ✅ 每个实例可登录不同账号
- ✅ 数据完全隔离,互不影响

**数据位置**: `~/Library/Containers/WeComInstance{N}/`

---

## Windows Sandboxie 安装

### 下载 Sandboxie-Plus

1. 访问 https://github.com/sandboxie-plus/Sandboxie/releases
2. 下载最新版本的安装包 (例如: `Sandboxie-Plus-x64-v1.xx.x.exe`)
3. 以管理员权限运行安装程序
4. 按照安装向导完成安装

### 验证安装

重启应用后,在"隔离模式"选项中:
- 如果显示 "✓ 已安装",表示检测成功
- 如果显示 "未安装",请检查安装路径是否为默认路径

### 默认安装路径
```
C:\Program Files\Sandboxie-Plus\
```

---

## 常见问题

### Q: 启动失败怎么办?

**Windows简单模式:**
- 检查企业微信/微信是否已正确安装
- 尝试以管理员权限运行本工具
- 查看错误提示信息

**Windows沙盒模式:**
- 确认已安装Sandboxie-Plus
- 检查Sandboxie服务是否正常运行
- 尝试重启Sandboxie服务

**macOS:**
- 确认应用路径正确 (`/Applications/企业微信.app` 或 `/Applications/WeCom.app`)
- 检查应用权限设置

### Q: 如何切换登录不同账号?

启动实例后,每个窗口可以独立登录不同账号:

**简单模式 (Windows):**
- 所有实例共享数据,建议只在不同实例间切换账号使用

**沙盒/macOS模式:**
- 每个实例完全独立,可以同时登录多个不同账号

### Q: 数据存储在哪里?

**Windows简单模式:**
- 企业微信: `C:\Users\{用户名}\AppData\Roaming\Tencent\WXWork\`
- 微信: `C:\Users\{用户名}\Documents\WeChat Files\`

**Windows沙盒模式:**
- 每个实例在独立沙盒中: `C:\Sandbox\{用户名}\WeCom_{N}\`

**macOS:**
- 每个实例独立目录: `~/Library/Containers/WeComInstance{N}/`

### Q: 关闭工具后实例会保留吗?

**默认行为**: 是,关闭工具后实例继续运行

**修改设置**: 可通过"退出设置"修改为退出时关闭所有实例

### Q: 个人微信和企业微信可以同时多开吗?

可以!分别选择应用类型后启动即可:
1. 选择"企业微信",启动2个实例
2. 选择"个人微信",启动3个实例
3. 总共运行5个实例 (2个企业微信 + 3个微信)

### Q: Sandboxie是否免费?

是的,[Sandboxie-Plus](https://github.com/sandboxie-plus/Sandboxie) 是完全开源免费的项目。

---

## 使用技巧

### 企业用户建议

1. **使用Sandboxie模式**: 确保数据安全和隔离
2. **定期清理沙盒**: 避免占用过多磁盘空间
3. **记录账号对应关系**: 标记哪个实例对应哪个账号

### 个人用户建议

1. **macOS优先**: macOS自动隔离,无需配置
2. **Windows快速测试**: 简单模式快速启动
3. **长期使用**: 推荐安装Sandboxie

### 性能优化

- 不要一次启动过多实例 (建议≤10个)
- 沙盒模式会占用更多系统资源
- 定期清理不使用的沙盒数据

---

## 卸载说明

### 卸载本工具

**Windows:**
- 运行卸载程序或直接删除安装目录

**macOS:**
- 将应用拖入废纸篓

### 清理数据

**Windows沙盒数据:**
```
C:\Sandbox\{用户名}\WeCom_*\
```

**macOS实例数据:**
```bash
rm -rf ~/Library/Containers/WeComInstance*
rm -rf ~/Applications/WeComMulti
```

---

## 获取帮助

- **GitHub Issues**: https://github.com/{your-repo}/issues
- **文档**: 查看 `docs/` 目录下的其他文档
- **CLAUDE.md**: 查看开发者文档了解技术细节
