// GUI 版本 - Tauri 图形界面
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{Manager, WindowEvent};
use wecom_multi_open::{platform, SpawnRequest};

/// 应用状态
#[derive(Default, Clone)]
struct AppState {
    /// 当前运行的实例 PID 列表
    pids: Arc<Mutex<Vec<u32>>>,
}


/// GUI 响应
#[derive(Debug, Serialize, Deserialize)]
struct GuiResponse {
    success: bool,
    message: String,
    pids: Vec<u32>,
}

/// Tauri 命令: 启动多个实例
#[tauri::command]
async fn spawn_instances(
    count: u8,
    state: tauri::State<'_, AppState>,
) -> Result<GuiResponse, String> {
    println!("收到启动请求: {} 个实例", count);

    let req = SpawnRequest {
        count,
        app_path: None,
    };

    match platform::spawn_multiple(req).await {
        Ok(response) => {
            // 保存 PID 到状态
            let mut pids = state.pids.lock().unwrap();
            pids.extend_from_slice(&response.pids);

            Ok(GuiResponse {
                success: true,
                message: format!("成功启动 {} 个实例!", response.success),
                pids: response.pids,
            })
        }
        Err(e) => Ok(GuiResponse {
            success: false,
            message: format!("启动失败: {}", e),
            pids: vec![],
        }),
    }
}

/// Tauri 命令: 关闭所有实例
#[tauri::command]
async fn kill_all_instances(state: tauri::State<'_, AppState>) -> Result<GuiResponse, String> {
    let pids = state.pids.lock().unwrap();
    let count = pids.len();

    let mut killed = 0;
    for &pid in pids.iter() {
        if platform::process_exists(pid) {
            if let Ok(_) = platform::kill_process(pid) {
                killed += 1;
            }
        }
    }

    // 清空 PID 列表
    drop(pids);
    state.pids.lock().unwrap().clear();

    Ok(GuiResponse {
        success: true,
        message: format!("已关闭 {} / {} 个实例", killed, count),
        pids: vec![],
    })
}

/// Tauri 命令: 获取当前运行的实例
#[tauri::command]
async fn get_running_instances(
    state: tauri::State<'_, AppState>,
) -> Result<GuiResponse, String> {
    let mut pids = state.pids.lock().unwrap();

    // 过滤出仍在运行的进程
    let running_pids: Vec<u32> = pids
        .iter()
        .copied()
        .filter(|&pid| platform::process_exists(pid))
        .collect();

    // 更新 PID 列表,移除已经不存在的进程
    *pids = running_pids.clone();

    Ok(GuiResponse {
        success: true,
        message: format!("当前运行 {} 个实例", running_pids.len()),
        pids: running_pids,
    })
}

/// 清理所有子进程
fn cleanup_all_processes(state: &AppState) {
    let pids = state.pids.lock().unwrap();
    let count = pids.len();

    println!("应用退出,清理 {} 个子进程...", count);

    let mut killed = 0;
    for &pid in pids.iter() {
        if platform::process_exists(pid) {
            match platform::kill_process(pid) {
                Ok(_) => {
                    println!("✓ 已关闭进程 {}", pid);
                    killed += 1;
                }
                Err(e) => {
                    eprintln!("✗ 关闭进程 {} 失败: {}", pid, e);
                }
            }
        }
    }

    println!("清理完成: 已关闭 {} / {} 个进程", killed, count);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::default())
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                // 关闭窗口时最小化到托盘而不是退出
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            spawn_instances,
            kill_all_instances,
            get_running_instances,
        ])
        .build(tauri::generate_context!())
        .expect("启动 Tauri 应用失败")
        .run(|app_handle, event| {
            // 监听应用退出事件
            if let tauri::RunEvent::Exit = event {
                println!("应用正在退出,开始清理子进程...");
                let state = app_handle.state::<AppState>();
                cleanup_all_processes(state.inner());
            }
        });
}
