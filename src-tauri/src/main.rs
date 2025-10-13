// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lib;

use lib::mutex;
use lib::process::{ProcessManager, ProcessInfo};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;

/// 全局状态
pub struct AppState {
    process_manager: Arc<ProcessManager>,
}

/// 多开请求参数
#[derive(Debug, Deserialize)]
struct SpawnRequest {
    count: u8,
    wecom_path: Option<String>,
}

/// 多开响应
#[derive(Debug, Serialize)]
struct SpawnResponse {
    pids: Vec<u32>,
    success: usize,
    failed: usize,
}

/// 启动多个企业微信实例
#[tauri::command]
async fn spawn_wecom(
    req: SpawnRequest,
    state: State<'_, AppState>,
) -> Result<SpawnResponse, String> {
    let mut pids = Vec::new();
    let mut success = 0;
    let mut failed = 0;

    let path = req.wecom_path.map(PathBuf::from);

    for i in 0..req.count {
        println!("正在启动第 {} 个实例...", i + 1);

        // 1. 关闭 Mutex
        if let Err(e) = mutex::force_close_wecom_mutex() {
            eprintln!("关闭 Mutex 失败: {}", e);
        }

        // 等待一小段时间确保 Mutex 被释放
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // 2. 启动进程
        match lib::process::launch_wecom(path.clone()) {
            Ok(pid) => {
                println!("成功启动实例，PID: {}", pid);
                state.process_manager.add_process(pid);
                pids.push(pid);
                success += 1;
            }
            Err(e) => {
                eprintln!("启动实例失败: {}", e);
                failed += 1;
            }
        }

        // 等待进程完全启动并创建自己的 Mutex
        tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
    }

    Ok(SpawnResponse {
        pids,
        success,
        failed,
    })
}

/// 获取默认企业微信路径
#[tauri::command]
fn get_default_wecom_path() -> String {
    lib::process::get_default_wecom_path()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default()
}

/// 列出所有管理的进程
#[tauri::command]
fn list_processes(state: State<'_, AppState>) -> Vec<ProcessInfo> {
    let procs = state.process_manager.list_processes();

    // 过滤掉已经不存在的进程
    procs
        .into_iter()
        .filter(|p| lib::process::process_exists(p.pid))
        .collect()
}

/// 关闭指定进程
#[tauri::command]
fn kill_process(pid: u32, state: State<'_, AppState>) -> Result<(), String> {
    lib::process::kill_process(pid)?;
    state.process_manager.remove_process(pid);
    Ok(())
}

/// 关闭所有管理的进程
#[tauri::command]
fn kill_all_processes(state: State<'_, AppState>) -> Result<(), String> {
    let procs = state.process_manager.list_processes();

    for proc in procs {
        if let Err(e) = lib::process::kill_process(proc.pid) {
            eprintln!("关闭进程 {} 失败: {}", proc.pid, e);
        }
    }

    state.process_manager.clear();
    Ok(())
}

/// 选择企业微信路径（打开文件对话框）
#[tauri::command]
async fn select_wecom_path() -> Result<String, String> {
    use tauri::api::dialog::blocking::FileDialogBuilder;

    let path = FileDialogBuilder::new()
        .add_filter("可执行文件", &["exe"])
        .set_title("选择企业微信程序")
        .pick_file();

    match path {
        Some(p) => Ok(p.to_string_lossy().to_string()),
        None => Ok(String::new()),
    }
}

fn main() {
    let process_manager = Arc::new(ProcessManager::new());

    tauri::Builder::default()
        .manage(AppState {
            process_manager: process_manager.clone(),
        })
        .invoke_handler(tauri::generate_handler![
            spawn_wecom,
            get_default_wecom_path,
            list_processes,
            kill_process,
            kill_all_processes,
            select_wecom_path,
        ])
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用时出错");
}
