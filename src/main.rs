// CLI 版本 - 命令行多开工具
use std::env;
use wecom_multi_open::{platform, SpawnRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从命令行参数获取启动数量
    let count = env::args()
        .nth(1)
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(3);

    println!("企业微信多开工具 v0.2.0");
    println!("准备启动 {} 个实例...\n", count);

    let req = SpawnRequest {
        count,
        app_path: None,
        app_type: None,           // 默认企业微信
        instance_configs: None,   // CLI不使用实例配置
    };

    match platform::spawn_multiple(req).await {
        Ok(response) => {
            println!("\n✓ 成功启动 {} 个实例!", response.success);
            if response.failed > 0 {
                println!("✗ 失败 {} 个实例", response.failed);
            }
            println!("进程 PID: {:?}", response.pids);
            println!("\n提示:");
            println!("- 每个窗口可以登录不同的企业微信账号");
            println!("- 关闭窗口或在任务管理器中结束进程即可退出");
            println!("\n按 Enter 键退出...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
        }
        Err(e) => {
            eprintln!("✗ 启动失败: {}", e);
            eprintln!("\n可能的原因:");
            eprintln!("1. 企业微信未安装或路径不正确");
            eprintln!("2. 需要管理员权限运行 (Windows)");
            eprintln!("3. 系统资源不足");
            eprintln!("\n按 Enter 键退出...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
        }
    }

    Ok(())
}
