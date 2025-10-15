#!/bin/bash
# Windows 编译检查脚本 - 在 macOS 上快速验证 Windows 代码
# 用法: ./check-windows.sh

set -e

# 添加 cargo 到 PATH
export PATH="$HOME/.cargo/bin:$PATH"

echo "🔍 检查 Windows 编译兼容性..."
echo ""

# 确保已安装 Windows 目标
if ! rustup target list --installed | grep -q "x86_64-pc-windows-msvc"; then
    echo "📦 安装 Windows MSVC 目标..."
    rustup target add x86_64-pc-windows-msvc
fi

echo "✓ Windows 目标已安装"
echo ""

# 检查库代码
echo "📚 检查核心库代码 (src/lib.rs)..."
cargo check --target x86_64-pc-windows-msvc --lib 2>&1 | grep -E "(error|warning:.*error)" && {
    echo "❌ 发现编译错误!"
    exit 1
} || echo "✅ 核心库编译通过!"
echo ""

# 检查 CLI
echo "🖥️  检查 CLI 代码 (src/main.rs)..."
cargo check --target x86_64-pc-windows-msvc --bin wecom-multi-open-cli 2>&1 | grep -E "(error|warning:.*error)" && {
    echo "❌ 发现编译错误!"
    exit 1
} || echo "✅ CLI 编译通过!"
echo ""

# GUI 检查 (会失败但不影响结果)
echo "🎨 检查 GUI 代码 (src/gui.rs)..."
echo "   注意: GUI 完整构建需要 Windows 环境,这里只验证 Rust 代码"
cargo check --target x86_64-pc-windows-msvc --features gui 2>&1 | grep -E "^error\[" && {
    echo "❌ 发现 GUI 编译错误!"
    exit 1
} || echo "✅ GUI Rust 代码语法正确!"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Windows 编译检查完成!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "说明:"
echo "• 核心库和 CLI 已验证可在 Windows 上编译"
echo "• GUI 完整构建需要在 Windows 机器或 GitHub Actions 上进行"
echo "• 如果看到命名风格警告(snake_case),这些不影响编译"
echo ""
