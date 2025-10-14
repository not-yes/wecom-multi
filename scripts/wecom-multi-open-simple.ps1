# wecom-multi-open.ps1
# 企业微信多开工具 - PowerShell 简化版
# 使用 Sysinternals Handle 工具关闭 Mutex

param(
    [int]$Count = 2
)

$ErrorActionPreference = "Stop"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  企业微信多开工具 - PowerShell 版本" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 1. 下载 Sysinternals Handle.exe（一次性）
$handleZip = "$env:TEMP\handle.zip"
$handleDir = "$env:TEMP\Handle"
$handleExe = "$handleDir\handle.exe"

if (!(Test-Path $handleExe)) {
    Write-Host "正在下载 Handle 工具..." -ForegroundColor Yellow
    try {
        Invoke-WebRequest -Uri "https://download.sysinternals.com/files/Handle.zip" -OutFile $handleZip
        Expand-Archive -Path $handleZip -DestinationPath $handleDir -Force
        Write-Host "✓ Handle 工具下载完成" -ForegroundColor Green
    } catch {
        Write-Host "✗ Handle 工具下载失败: $_" -ForegroundColor Red
        Write-Host "请手动下载: https://download.sysinternals.com/files/Handle.zip" -ForegroundColor Yellow
        exit 1
    }
}

# 2. 查找企业微信路径
$wecomPaths = @(
    "${env:ProgramFiles(x86)}\WXWork\WXWork.exe",
    "${env:ProgramFiles}\WXWork\WXWork.exe",
    "D:\Program Files (x86)\WXWork\WXWork.exe",
    "D:\Program Files\WXWork\WXWork.exe"
)

$wecom = $null
foreach ($path in $wecomPaths) {
    if (Test-Path $path) {
        $wecom = $path
        break
    }
}

if (-not $wecom) {
    Write-Host "✗ 未找到企业微信安装路径" -ForegroundColor Red
    Write-Host "请确认已安装企业微信" -ForegroundColor Yellow
    exit 1
}

Write-Host "企业微信路径: $wecom" -ForegroundColor Green
Write-Host ""

# 3. 关闭 Mutex 函数
function Close-WeComMutex {
    try {
        # 查找企业微信进程的 Mutex 句柄
        $output = & $handleExe -accepteula -nobanner -p WXWork.exe 2>&1

        $output | Where-Object {
            $_ -match "ExclusiveObject"
        } | ForEach-Object {
            $line = $_ -split '\s+'
            if ($line.Length -ge 5) {
                $pid = $line[2]
                $handle = $line[4] -replace ':'
                # 关闭句柄
                & $handleExe -accepteula -nobanner -c $handle -p $pid -y 2>&1 | Out-Null
            }
        }
    } catch {
        # 静默失败,继续尝试启动
    }
}

# 4. 启动多个实例
Write-Host "准备启动 $Count 个实例..." -ForegroundColor Cyan
Write-Host ""

$successCount = 0
for ($i = 1; $i -le $Count; $i++) {
    Write-Host "[$i/$Count] 正在启动实例..." -ForegroundColor Cyan

    # 关闭 Mutex
    Close-WeComMutex
    Start-Sleep -Milliseconds 100

    try {
        # 启动进程
        $process = Start-Process -FilePath $wecom -PassThru

        if ($process) {
            Write-Host "  ✓ 启动成功 (PID: $($process.Id))" -ForegroundColor Green
            $successCount++
        } else {
            Write-Host "  ✗ 启动失败" -ForegroundColor Red
        }
    } catch {
        Write-Host "  ✗ 启动失败: $_" -ForegroundColor Red
    }

    # 等待进程完全启动
    if ($i -lt $Count) {
        Start-Sleep -Milliseconds 800
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "启动完成!" -ForegroundColor Green
Write-Host "成功: $successCount / 失败: $($Count - $successCount)" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "提示:" -ForegroundColor Yellow
Write-Host "- 每个窗口可以登录不同的企业微信账号" -ForegroundColor White
Write-Host "- 关闭窗口或在任务管理器中结束进程即可退出" -ForegroundColor White
Write-Host ""
