# 贡献指南

感谢你考虑为企业微信多开工具做出贡献!

## 行为准则

本项目遵循 [贡献者公约](https://www.contributor-covenant.org/zh-cn/version/2/1/code_of_conduct/)。
参与本项目即表示你同意遵守其条款。

## 如何贡献

### 报告 Bug

如果你发现了 bug,请:

1. 检查 [Issues](https://github.com/yourusername/wecom-multi-open/issues) 是否已有相关报告
2. 如果没有,创建新 Issue,包含:
   - 清晰的标题和描述
   - 复现步骤
   - 预期行为 vs 实际行为
   - 截图 (如适用)
   - 环境信息:
     - 操作系统版本
     - 企业微信版本
     - 应用版本

### 提出功能建议

我们欢迎新功能建议! 请:

1. 在 [Discussions](https://github.com/yourusername/wecom-multi-open/discussions) 中发起讨论
2. 说明功能的用途和价值
3. 提供使用场景示例
4. 讨论实现方案

### 提交代码

#### 准备工作

1. Fork 本仓库
2. 克隆你的 fork:
   ```bash
   git clone https://github.com/your-username/wecom-multi-open.git
   cd wecom-multi-open
   ```

3. 安装依赖:
   ```bash
   npm install
   ```

4. 创建功能分支:
   ```bash
   git checkout -b feature/amazing-feature
   ```

#### 开发规范

**代码风格**:

Rust:
```bash
# 格式化代码
cargo fmt

# 检查代码
cargo clippy
```

TypeScript/React:
```bash
# 格式化代码
npm run format

# 检查类型
npm run type-check
```

**提交信息**:

遵循 [Conventional Commits](https://www.conventionalcommits.org/zh-hans/):

```
<类型>(<范围>): <描述>

[可选的正文]

[可选的脚注]
```

类型:
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式调整
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建/工具变更

示例:
```
feat(ui): 添加暗色主题支持

- 新增主题切换组件
- 更新 CSS 变量
- 保存用户偏好设置

Closes #123
```

#### 测试

提交前请确保:

```bash
# Rust 测试
cd src-tauri
cargo test

# 前端测试
npm test

# 手动测试
npm run tauri:dev
```

#### 提交 Pull Request

1. 推送到你的 fork:
   ```bash
   git push origin feature/amazing-feature
   ```

2. 在 GitHub 上创建 Pull Request

3. PR 描述应包含:
   - 变更内容说明
   - 相关 Issue 编号
   - 测试结果
   - 截图 (如适用)

4. 等待代码审查

## 开发环境

### 系统要求

- Windows 10/11 (核心功能开发)
- Node.js >= 18
- Rust >= 1.70
- Git

### IDE 推荐

**VS Code** 推荐插件:
- rust-analyzer
- Tauri
- ESLint
- Prettier
- TypeScript

### 项目结构

```
wecom-multi-open/
├── src/                 # React 前端
├── src-tauri/           # Rust 后端
├── scripts/             # 脚本
├── docs/                # 文档
├── tests/               # 测试
└── package.json
```

详见 [开发文档](docs/DEVELOPMENT.md)

## 代码审查

Pull Request 会被审查:

1. **代码质量**: 遵循项目规范
2. **功能完整**: 满足需求且无 bug
3. **测试覆盖**: 包含必要测试
4. **文档更新**: 更新相关文档
5. **向下兼容**: 不破坏现有功能

## 发布流程

仅维护者可发布新版本:

1. 更新版本号 (`package.json`, `Cargo.toml`, `tauri.conf.json`)
2. 更新 `CHANGELOG.md`
3. 创建 Git tag:
   ```bash
   git tag -a v0.2.0 -m "Release v0.2.0"
   git push origin v0.2.0
   ```
4. GitHub Actions 自动构建和发布

## 社区

- **GitHub Discussions**: 讨论功能和想法
- **GitHub Issues**: 报告 bug 和请求功能
- **Pull Requests**: 贡献代码

## 许可证

贡献的代码将遵循项目的 [MIT License](LICENSE)。

## 致谢

感谢所有贡献者! 你们的努力让这个项目更好。

---

有问题? 欢迎在 Discussions 中提问!
