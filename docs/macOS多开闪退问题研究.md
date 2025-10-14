# macOS 企业微信多开闪退问题研究报告

## 问题描述

在 macOS 平台上实现企业微信(WeCom)多开时,虽然应用能够成功启动,但会在几秒钟后自动闪退。

## 研究发现

### 1. 微信 4.0 版本后的变化

根据社区反馈,微信从 3.0 升级到 4.0 后,传统的多开方法失效率显著提高。这表明腾讯在新版本中加强了反多开检测机制。

**来源**: CSDN - macOS版本微信 4.0 之后,双开策略

### 2. 成功的多开方案(微信)

针对微信的成功多开方案包含以下5个步骤:

```bash
# 1. 创建微信副本
sudo cp -R /Applications/WeChat.app /Applications/WeChat2.app

# 2. 修改 Bundle Identifier
sudo /usr/libexec/PlistBuddy -c "Set :CFBundleIdentifier com.tencent.xinWeChat2" \
  /Applications/WeChat2.app/Contents/Info.plist

# 3. 代码签名
sudo codesign --force --deep --sign - /Applications/WeChat2.app

# 4. 启动第二个实例
nohup /Applications/WeChat2.app/Contents/MacOS/WeChat >/dev/null 2>&1 &

# 5. 登录第二个实例
```

**关键点**:
- 第二个微信会保留在 Dock 中
- 微信升级后需要删除 WeChat2.app 并重新创建
- 数据不会丢失

### 3. macOS 多实例启动方法

#### 方法A: 使用 `open -n` 命令

```bash
open -n /Applications/ApplicationName.app
# 或简写为
open -na ApplicationName
```

**优点**: 系统标准方法,兼容性好
**缺点**: 某些应用会共享偏好设置,可能导致冲突

#### 方法B: 直接启动可执行文件

```bash
/Applications/AppName.app/Contents/MacOS/AppName &
```

**优点**: 更底层的启动方式,避开某些应用级检测
**缺点**: 可能缺少某些环境变量

#### 方法C: 使用 nohup (推荐)

```bash
nohup /Applications/AppName.app/Contents/MacOS/AppName >/dev/null 2>&1 &
```

**优点**:
- 进程独立于终端
- 忽略 HUP 信号
- 更稳定可靠

**缺点**: 需要从命令行启动

## 当前实现分析

### 现有实现

当前代码在 `src/lib.rs` 的 macOS 平台实现中:

1. ✅ 复制应用到 `~/Applications/WeComMulti/`
2. ✅ 修改 CFBundleIdentifier
3. ✅ 清除隔离属性 (xattr)
4. ✅ 重新签名 (codesign)
5. ⚠️ 使用 `open -n -a` 启动

### 潜在问题

1. **启动方法不够稳定**: 使用 `open` 命令可能触发系统级检测
2. **PID 获取不准确**: `open` 命令返回的子进程 PID 不是实际 WeCom 进程的 PID
3. **缺少进程隔离**: 可能共享某些系统资源或配置文件

## 改进方案

### 方案1: 直接启动可执行文件 (已实施)

```rust
// 获取可执行文件路径
let executable_path = instance_path
    .join("Contents/MacOS")
    .join(app_name);

// 直接启动
Command::new(&executable_path).spawn()
```

**预期效果**:
- 避开 `open` 命令的系统检测
- 获取真实进程 PID
- 更接近原生启动方式

### 方案2: 添加启动参数隔离 (待测试)

```rust
Command::new(&executable_path)
    .env("HOME", format!("/tmp/wecom_instance_{}", instance_id))
    .spawn()
```

**目的**: 为每个实例提供独立的配置目录

### 方案3: 延迟启动并检测进程 (待实施)

```rust
// 启动后等待进程稳定
tokio::time::sleep(Duration::from_secs(2)).await;

// 通过进程名查找真实 PID
let output = Command::new("pgrep")
    .arg("-f")
    .arg(instance_path.to_str().unwrap())
    .output()?;
```

**目的**: 确保获取实际运行的 WeCom 进程 PID

## 其他可能的闪退原因

### 1. 应用内检测机制

企业微信可能实施了以下检测:

- **进程名检测**: 检查是否有多个同名进程
- **共享资源锁**: 使用文件锁或套接字防止多开
- **服务器端设备ID检测**: 检测同一设备多个登录
- **IPC通信检测**: 通过进程间通信发现其他实例

### 2. 系统级限制

- **文件锁**: 某些配置文件被独占访问
- **端口占用**: 应用尝试绑定固定端口
- **共享内存**: 使用固定名称的共享内存段

### 3. 代码签名问题

虽然我们进行了重新签名,但ad-hoc签名可能导致:
- Gatekeeper 拦截
- 沙箱限制
- 权限不足

## 建议的测试步骤

1. **测试直接启动**: 验证改进后的启动方法是否更稳定
2. **监控系统日志**:
   ```bash
   log stream --predicate 'process == "WeChat"' --level debug
   ```
3. **检查崩溃报告**: 查看 `~/Library/Logs/DiagnosticReports/`
4. **网络监控**: 使用 Wireshark 查看是否有异常网络检测
5. **进程监控**: 使用 `fs_usage` 监控文件访问模式

## 社区方案状态

根据最新搜索结果(2024-2025):

- ✅ **微信多开**: 有成功案例,使用上述5步方案
- ⚠️ **企业微信多开**: 社区讨论较少,成功案例不明确
- ⚠️ **iOS应用**: M1/M2 Mac 上运行 iOS 应用的多开更困难

## 参考资料

1. CSDN - macOS版本微信 4.0 之后,双开策略
2. Stack Overflow - Launch multiple instances of application in Mac
3. macOS 开发者文档 - Launch Services
4. 企业微信开发者社区 - 闪退问题讨论

## 测试结果与新发现

### 测试1: 直接启动可执行文件 (2025-10-14)

**结果**: ✅ 三个实例均成功启动,不再闪退

**新问题**: 🔴 **只能有一个实例登录,其他实例登录会导致之前的实例自动退出**

**根本原因分析**:

1. **数据库锁定冲突**:
   ```
   Contact.db sqlite error 5: database is locked
   CorpCircle.db sqlite error 5: database is locked
   ```
   - 所有实例共享同一个数据库文件
   - 导致数据库锁定冲突

2. **共享资源冲突**:
   - 日志目录共享: `/Users/wangke/Documents/GYLog/0/`
   - Extension 锁定: `LOCK: File currently in use`
   - 配置文件共享

3. **服务器端会话管理**:
   - 企业微信服务器检测到同一账号多次登录
   - 自动踢掉旧会话,保持单一会话

### 改进方案3: 独立数据目录隔离 (已实施)

为每个实例创建完全独立的数据目录:

```rust
// 为每个实例创建独立的 HOME 目录
let instance_home = format!("{}/Library/Containers/WeComInstance{}", home, instance_id);

// 使用独立的 HOME 环境变量启动
Command::new(&executable_path)
    .env("HOME", &instance_home)
    .spawn()
```

**预期效果**:
- ✅ 每个实例使用独立的数据库文件
- ✅ 每个实例使用独立的配置和日志
- ✅ 避免文件锁定冲突
- ⚠️ 服务器端多账号登录限制仍然存在

**注意**: 这个方案解决了本地数据冲突,但**无法解决服务器端的单一会话限制**。企业微信服务器会检测并限制同一账号的多次登录。

## 重要结论

### 技术上可行的多开场景

1. **多账号登录** ✅
   - 每个实例使用不同的企业微信账号
   - 完全独立,互不影响

2. **单账号多设备** ⚠️
   - 理论上可行,但受服务器端会话管理限制
   - 可能需要修改设备标识

### 技术上不可行的场景

1. **同一账号,同一设备,多次登录** ❌
   - 服务器端强制单一会话
   - 新登录会踢掉旧会话
   - 这是企业微信的安全策略,无法通过客户端绕过

## 下一步行动

1. ✅ 实施直接启动可执行文件方案
2. ✅ 测试改进后的启动稳定性
3. ✅ 实施环境变量隔离方案
4. ⏳ 测试多账号登录场景
5. ⏳ 文档化使用限制和最佳实践
6. ⏳ 更新 UI 提示用户每个实例需要不同账号

---

**更新时间**: 2025年10月14日 15:46
**状态**: 环境变量隔离未完全生效,企业微信仍使用系统API获取用户目录
**当前问题**:
1. 设置 HOME 环境变量后,企业微信仍使用 `/Users/wangke/Documents/GYLog/` (通过系统API)
2. 这表明企业微信使用 `getpwuid()` 而非 `getenv("HOME")` 来获取用户目录
3. **根本限制**: macOS 不允许非特权进程欺骗 `getpwuid()` 的返回值

**重要发现**:
- **环境变量方法不可行** - 企业微信使用系统 API 直接获取用户信息,无法通过环境变量欺骗
- **唯一可行方案**: 使用 macOS 的**用户切换**功能,为每个实例创建真实的 macOS 用户账户
- **或者**: 接受数据共享,但要求每个实例使用不同的企业微信账号
