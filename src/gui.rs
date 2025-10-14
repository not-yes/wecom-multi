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

/// Tauri 命令: 关闭单个实例
#[tauri::command]
async fn kill_instance(
    pid: u32,
    state: tauri::State<'_, AppState>,
) -> Result<GuiResponse, String> {
    if platform::process_exists(pid) {
        match platform::kill_process(pid) {
            Ok(_) => {
                // 从状态中移除此 PID
                let mut pids = state.pids.lock().unwrap();
                pids.retain(|&p| p != pid);

                Ok(GuiResponse {
                    success: true,
                    message: format!("已关闭实例 {}", pid),
                    pids: pids.clone(),
                })
            }
            Err(e) => Ok(GuiResponse {
                success: false,
                message: format!("关闭实例失败: {}", e),
                pids: vec![],
            }),
        }
    } else {
        // 进程不存在,从列表中移除
        let mut pids = state.pids.lock().unwrap();
        pids.retain(|&p| p != pid);

        Ok(GuiResponse {
            success: true,
            message: format!("实例 {} 已不存在", pid),
            pids: pids.clone(),
        })
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
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // 应用原生窗口效果
            // 注意: 在非透明窗口下,vibrancy 效果可能不会生效
            // 目前使用 CSS 样式模拟平台特定的视觉效果

            #[cfg(target_os = "macos")]
            {
                // macOS 的 vibrancy 效果需要透明窗口
                // 当前配置下已禁用,使用 CSS glass-effect 代替
                // use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
                // let _ = apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None);
            }

            #[cfg(target_os = "windows")]
            {
                // Windows 11 的 Mica 效果需要特定配置
                // 当前配置下已禁用,使用 CSS glass-effect 代替
                // use window_vibrancy::apply_mica;
                // let _ = apply_mica(&window, Some(true))
                //     .or_else(|_| {
                //         window_vibrancy::apply_blur(&window, Some((18, 18, 18, 125)))
                //     });
            }

            Ok(())
        })
        .on_window_event(|_window, event| match event {
            WindowEvent::CloseRequested { .. } => {
                // 允许正常关闭窗口
                // window.hide().unwrap();
                // api.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            spawn_instances,
            kill_instance,
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
