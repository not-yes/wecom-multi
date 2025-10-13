# 更新日志

所有重要的变更都会记录在此文件中。

本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

### 计划中的功能
- [ ] 系统托盘支持
- [ ] 开机自启动
- [ ] 实例配置保存
- [ ] 进程守护模式
- [ ] 多语言支持

## [0.1.0] - 2025-01-14

### 新增
- 基础 GUI 界面 (Tauri + React)
- 企业微信多开核心功能
- Mutex 关闭逻辑
- 进程启动和管理
- 进程列表查看
- 单个/批量关闭进程
- 自动检测企业微信路径
- 手动选择企业微信路径
- PowerShell 降级脚本
- 完整的项目文档

### 技术栈
- 前端: React 18 + TypeScript + Vite
- 后端: Rust + Tauri 1.5 + windows-rs
- 构建: npm + Cargo

### 已知问题
- 仅支持 Windows 平台
- 需要管理员权限以获得最佳效果
- 某些企业微信版本可能不兼容

---

## 版本说明

### 主版本 (Major)
当做了不兼容的 API 修改

### 次版本 (Minor)
当做了向下兼容的功能性新增

### 修订号 (Patch)
当做了向下兼容的问题修正

---

**格式参考**: [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)
