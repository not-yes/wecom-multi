@echo off
chcp 65001 >nul
echo ========================================
echo   企业微信多开工具 - 一键构建
echo ========================================
echo.

REM 检查 Rust 是否安装
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [错误] 未检测到 Rust 环境
    echo.
    echo 请先安装 Rust:
    echo 1. 访问 https://rustup.rs/
    echo 2. 下载并运行安装程序
    echo 3. 重启终端后再次运行此脚本
    echo.
    pause
    exit /b 1
)

echo [1/3] 检查环境...
cargo --version
echo.

echo [2/3] 开始编译 (首次编译需要 5-10 分钟)...
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [错误] 编译失败
    echo 请检查错误信息或寻求帮助
    pause
    exit /b 1
)
echo.

echo [3/3] 编译完成!
echo.
echo 可执行文件位置:
echo target\release\wecom-multi-open.exe
echo.
echo 文件大小:
for %%A in (target\release\wecom-multi-open.exe) do echo %%~zA 字节
echo.

echo ========================================
echo 使用方法:
echo ========================================
echo.
echo 1. 双击运行 (默认启动 3 个实例)
echo    target\release\wecom-multi-open.exe
echo.
echo 2. 启动指定数量 (例如 5 个)
echo    target\release\wecom-multi-open.exe 5
echo.
echo 3. 复制 EXE 到桌面或其他位置使用
echo.

pause
