# PowerShell 脚本使用说明

## 概述

这是企业微信多开工具的 PowerShell 降级版本,当主程序无法使用时可以使用此脚本。

## 使用方法

### 基本用法

```powershell
# 启动 2 个实例 (默认)
.\wecom_multi_open.ps1

# 启动 3 个实例
.\wecom_multi_open.ps1 -Count 3

# 指定企业微信路径
.\wecom_multi_open.ps1 -Count 2 -WeComPath "C:\Program Files\WXWork\WXWork.exe"
```

### 以管理员权限运行

为了获得最佳效果,建议以管理员权限运行:

1. 右键点击 PowerShell
2. 选择"以管理员身份运行"
3. 导航到脚本目录
4. 运行脚本

### 执行策略

如果遇到"无法加载,因为在此系统上禁止运行脚本"的错误,需要临时允许脚本执行:

```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
```

然后再运行脚本。

## 参数说明

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| Count | int | 2 | 要启动的实例数量 (1-10) |
| WeComPath | string | 自动检测 | 企业微信程序路径 |

## 工作原理

1. 查找或验证企业微信安装路径
2. 循环启动指定数量的实例:
   - 关闭独占 Mutex
   - 启动企业微信进程
   - 等待进程完全启动
3. 输出启动结果和进程 PID

## 限制

- 仅支持 Windows 平台
- 建议以管理员权限运行
- 与主程序相比,功能较为基础
- 不提供进程管理和守护功能

## 故障排除

### 问题: 脚本无法运行

**解决**: 检查 PowerShell 执行策略,使用 `Set-ExecutionPolicy` 临时允许脚本执行

### 问题: 未找到企业微信路径

**解决**: 使用 `-WeComPath` 参数手动指定企业微信程序路径

### 问题: 启动失败

**解决**:
1. 确保企业微信已正确安装
2. 以管理员权限运行脚本
3. 检查是否有其他安全软件阻止

## 示例

```powershell
# 示例 1: 启动 3 个实例
.\wecom_multi_open.ps1 -Count 3

# 示例 2: 指定路径启动 2 个实例
.\wecom_multi_open.ps1 -WeComPath "D:\WXWork\WXWork.exe"

# 示例 3: 完整参数
.\wecom_multi_open.ps1 -Count 5 -WeComPath "C:\Program Files (x86)\WXWork\WXWork.exe"
```

## 注意事项

- 建议启动 2-5 个实例,过多可能影响性能
- 每次启动后建议等待所有实例完全加载
- 关闭实例请直接在任务管理器中结束进程
