<#
.SYNOPSIS
    企业微信多开工具 - PowerShell 脚本版本

.DESCRIPTION
    通过关闭系统 Mutex 实现企业微信多开
    此脚本为降级方案,在无法使用主程序时使用

.PARAMETER Count
    要启动的企业微信实例数量 (默认: 2)

.PARAMETER WeComPath
    企业微信程序路径 (默认: 自动检测)

.EXAMPLE
    .\wecom_multi_open.ps1 -Count 3

.EXAMPLE
    .\wecom_multi_open.ps1 -Count 2 -WeComPath "C:\Program Files (x86)\WXWork\WXWork.exe"
#>

param(
    [Parameter(Mandatory=$false)]
    [int]$Count = 2,

    [Parameter(Mandatory=$false)]
    [string]$WeComPath = ""
)

# 企业微信 Mutex 名称
$MutexName = "Tencent.WeWork.ExclusiveObject"

# 颜色输出函数
function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    Write-Host $Message -ForegroundColor $Color
}

# 查找企业微信安装路径
function Find-WeComPath {
    $possiblePaths = @(
        "C:\Program Files (x86)\WXWork\WXWork.exe",
        "C:\Program Files\WXWork\WXWork.exe",
        "D:\Program Files (x86)\WXWork\WXWork.exe",
        "D:\Program Files\WXWork\WXWork.exe"
    )

    foreach ($path in $possiblePaths) {
        if (Test-Path $path) {
            return $path
        }
    }

    return $null
}

# 关闭 Mutex (使用 .NET Mutex 类尝试)
function Close-Mutex {
    param([string]$Name)

    try {
        # 尝试打开 Mutex
        $mutex = [System.Threading.Mutex]::OpenExisting($Name)
        if ($mutex) {
            $mutex.Close()
            $mutex.Dispose()
            return $true
        }
    } catch {
        # Mutex 不存在或无法打开,这是正常的
        return $true
    }

    return $false
}

# 启动企业微信进程
function Start-WeComInstance {
    param(
        [string]$ExePath,
        [int]$Index
    )

    Write-ColorOutput "正在启动第 $Index 个实例..." "Cyan"

    # 关闭 Mutex
    Close-Mutex -Name $MutexName | Out-Null

    # 等待一小段时间
    Start-Sleep -Milliseconds 100

    try {
        # 启动进程
        $process = Start-Process -FilePath $ExePath -PassThru

        if ($process) {
            Write-ColorOutput "✓ 成功启动实例 $Index (PID: $($process.Id))" "Green"

            # 等待进程完全启动
            Start-Sleep -Milliseconds 800

            return $process.Id
        } else {
            Write-ColorOutput "✗ 启动实例 $Index 失败" "Red"
            return $null
        }
    } catch {
        Write-ColorOutput "✗ 启动实例 $Index 时出错: $_" "Red"
        return $null
    }
}

# 主函数
function Main {
    Write-ColorOutput "`n========================================" "Yellow"
    Write-ColorOutput "  企业微信多开工具 - PowerShell 版本" "Yellow"
    Write-ColorOutput "========================================`n" "Yellow"

    # 检查是否以管理员权限运行
    $isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

    if (-not $isAdmin) {
        Write-ColorOutput "警告: 建议以管理员权限运行此脚本以获得最佳效果" "Yellow"
    }

    # 确定企业微信路径
    if ([string]::IsNullOrEmpty($WeComPath)) {
        Write-ColorOutput "正在查找企业微信安装路径..." "Cyan"
        $WeComPath = Find-WeComPath

        if (-not $WeComPath) {
            Write-ColorOutput "✗ 未找到企业微信安装路径,请手动指定 -WeComPath 参数" "Red"
            exit 1
        }

        Write-ColorOutput "✓ 找到企业微信: $WeComPath" "Green"
    } else {
        if (-not (Test-Path $WeComPath)) {
            Write-ColorOutput "✗ 指定的路径不存在: $WeComPath" "Red"
            exit 1
        }
    }

    # 验证数量
    if ($Count -lt 1 -or $Count -gt 10) {
        Write-ColorOutput "✗ 实例数量必须在 1-10 之间" "Red"
        exit 1
    }

    Write-ColorOutput "`n开始启动 $Count 个企业微信实例...`n" "Cyan"

    # 启动多个实例
    $pids = @()
    for ($i = 1; $i -le $Count; $i++) {
        $pid = Start-WeComInstance -ExePath $WeComPath -Index $i
        if ($pid) {
            $pids += $pid
        }
    }

    # 输出结果
    Write-ColorOutput "`n========================================" "Yellow"
    Write-ColorOutput "启动完成!" "Green"
    Write-ColorOutput "成功: $($pids.Count) / 失败: $($Count - $pids.Count)" "Cyan"

    if ($pids.Count -gt 0) {
        Write-ColorOutput "`n运行中的进程 PID:" "Cyan"
        foreach ($pid in $pids) {
            Write-ColorOutput "  - $pid" "White"
        }
    }

    Write-ColorOutput "========================================`n" "Yellow"
}

# 运行主函数
Main
