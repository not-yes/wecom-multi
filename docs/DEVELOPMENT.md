# 开发文档

## 技术栈

### 前端
- **React 18**: UI 框架
- **TypeScript**: 类型安全
- **Vite**: 构建工具
- **CSS 3**: 样式

### 后端
- **Rust**: 核心服务层
- **Tauri 1.5**: 桌面应用框架
- **windows-rs**: Windows API 绑定
- **tokio**: 异步运行时

## 架构设计

### 分层架构

```
┌─────────────────────────────────┐
│  Presentation Layer (React)     │
│  - 用户界面                      │
│  - 状态管理                      │
│  - 用户交互                      │
└────────────┬────────────────────┘
             │ Tauri IPC
             │ (JSON-RPC)
┌────────────┴────────────────────┐
│  Service Layer (Rust)           │
│  - Tauri Commands               │
│  - 进程管理                      │
│  - 状态维护                      │
└────────────┬────────────────────┘
             │ FFI
             │ (windows-rs)
┌────────────┴────────────────────┐
│  System Layer                   │
│  - Windows API                  │
│  - Mutex 操作                    │
│  - 进程控制                      │
└─────────────────────────────────┘
```

### 核心模块

#### 1. Mutex 模块 (`src-tauri/src/lib/mutex.rs`)

负责系统级 Mutex 管理:

```rust
// 关闭指定名称的 Mutex
pub fn close_mutex(mutex_name: &str) -> Result<(), String>

// 检查 Mutex 是否存在
pub fn mutex_exists(mutex_name: &str) -> bool

// 强制关闭企业微信 Mutex
pub fn force_close_wecom_mutex() -> Result<(), String>
```

**工作原理**:
1. 使用 `OpenMutexW` API 打开已存在的 Mutex
2. 使用 `CloseHandle` 关闭句柄
3. 允许新进程创建同名 Mutex

#### 2. Process 模块 (`src-tauri/src/lib/process.rs`)

负责进程生命周期管理:

```rust
// 进程管理器
pub struct ProcessManager {
    processes: Mutex<HashMap<u32, ProcessInfo>>,
}

// 获取默认安装路径
pub fn get_default_wecom_path() -> Option<PathBuf>

// 启动进程
pub fn launch_wecom(path: Option<PathBuf>) -> Result<u32, String>

// 关闭进程
pub fn kill_process(pid: u32) -> Result<(), String>

// 检查进程是否存在
pub fn process_exists(pid: u32) -> bool
```

#### 3. Tauri Commands (`src-tauri/src/main.rs`)

前后端通信接口:

```rust
// 启动多个实例
async fn spawn_wecom(req: SpawnRequest, state: State<'_, AppState>)
    -> Result<SpawnResponse, String>

// 获取默认路径
fn get_default_wecom_path() -> String

// 列出所有进程
fn list_processes(state: State<'_, AppState>) -> Vec<ProcessInfo>

// 关闭单个进程
fn kill_process(pid: u32, state: State<'_, AppState>) -> Result<(), String>

// 关闭所有进程
fn kill_all_processes(state: State<'_, AppState>) -> Result<(), String>

// 选择文件路径
async fn select_wecom_path() -> Result<String, String>
```

## 核心流程

### 多开启动流程

```
用户点击"启动多开"
    ↓
前端调用 spawn_wecom
    ↓
循环 N 次:
    ├─ 关闭 Mutex (force_close_wecom_mutex)
    ├─ 等待 100ms
    ├─ 启动进程 (launch_wecom)
    ├─ 记录 PID (add_process)
    └─ 等待 800ms
    ↓
返回启动结果
    ↓
前端更新 UI 显示
```

### 时间控制

- **100ms**: Mutex 关闭后等待系统释放
- **800ms**: 进程启动后等待 Mutex 重建

这些时间是经验值,可根据实际情况调整。

## 开发环境设置

### 前置要求

1. **Node.js >= 18**
   ```bash
   node --version
   ```

2. **Rust >= 1.70**
   ```bash
   rustc --version
   ```

3. **Windows 10/11** (核心功能需要 Windows)

### 安装依赖

```bash
# 安装前端依赖
npm install

# 或使用 pnpm
pnpm install

# 或使用 yarn
yarn install
```

### 开发模式

```bash
# 启动开发服务器
npm run tauri:dev
```

这会:
1. 启动 Vite 开发服务器 (端口 1420)
2. 编译 Rust 代码
3. 启动 Tauri 应用
4. 支持热重载

### 构建生产版本

```bash
# 构建应用
npm run tauri:build
```

输出位置: `src-tauri/target/release/bundle/`

## 测试

### 单元测试

```bash
# 运行 Rust 测试
cd src-tauri
cargo test

# 运行前端测试 (需要添加测试框架)
npm test
```

### 手动测试清单

- [ ] 启动 2 个实例
- [ ] 启动 5 个实例
- [ ] 验证所有实例独立运行
- [ ] 关闭单个实例
- [ ] 关闭所有实例
- [ ] 自定义企业微信路径
- [ ] 进程列表自动刷新
- [ ] 错误处理 (路径不存在、权限不足等)

## 调试技巧

### Rust 日志

在 `src-tauri/src/main.rs` 中使用:

```rust
println!("调试信息: {:?}", variable);
eprintln!("错误信息: {:?}", error);
```

### 前端调试

1. 打开开发者工具: `Ctrl + Shift + I`
2. 查看控制台日志
3. 使用 React DevTools

### Windows API 调试

使用 Process Explorer 查看:
- 进程句柄
- Mutex 对象
- 进程关系

## 常见问题

### Q: 编译失败,提示找不到 Windows SDK

**A**: 安装 Visual Studio Build Tools 并包含 Windows 10 SDK

### Q: Mutex 关闭失败

**A**:
1. 确保以管理员权限运行
2. 检查是否有安全软件拦截
3. 验证 Mutex 名称是否正确

### Q: 进程启动后立即退出

**A**:
1. 检查企业微信路径是否正确
2. 验证企业微信版本兼容性
3. 查看是否有依赖库缺失

## 贡献指南

### 代码风格

**Rust**:
- 使用 `rustfmt` 格式化
- 遵循 Rust API Guidelines
- 添加必要的文档注释

**TypeScript**:
- 使用 Prettier 格式化
- 遵循 React 最佳实践
- 使用函数式组件

### 提交规范

遵循 Conventional Commits:

```
feat: 添加新功能
fix: 修复 bug
docs: 更新文档
style: 代码格式调整
refactor: 重构代码
test: 添加测试
chore: 构建/工具变更
```

### Pull Request 流程

1. Fork 仓库
2. 创建功能分支: `git checkout -b feature/amazing-feature`
3. 提交更改: `git commit -m 'feat: add amazing feature'`
4. 推送分支: `git push origin feature/amazing-feature`
5. 提交 Pull Request

## 性能优化

### 启动速度优化

- 并行初始化非关键组件
- 延迟加载重资源
- 使用增量编译

### 内存优化

- 及时释放不用的句柄
- 使用 Arc 共享状态
- 避免不必要的克隆

### 二进制大小优化

在 `Cargo.toml` 中:

```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

## 安全考虑

1. **权限管理**: 仅请求必要的系统权限
2. **输入验证**: 验证所有用户输入
3. **路径安全**: 防止路径遍历攻击
4. **进程隔离**: 不干涉其他进程运行

## 许可证

MIT License - 详见 [LICENSE](../LICENSE)
