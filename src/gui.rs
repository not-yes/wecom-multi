// GUI 版本 - Tauri 图形界面
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    Window, WindowEvent,
};
use wecom_multi_open::{platform, SpawnRequest, SpawnResponse};

/// 应用状态
#[derive(Default)]
struct AppState {
    /// 当前运行的实例 PID 列表
    pids: Arc<Mutex<Vec<u32>>>,
}

/// GUI 启动请求
#[derive(Debug, Serialize, Deserialize)]
struct GuiSpawnRequest {
    count: u8,
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
    let pids = state.pids.lock().unwrap();

    // 过滤出仍在运行的进程
    let running_pids: Vec<u32> = pids
        .iter()
        .copied()
        .filter(|&pid| platform::process_exists(pid))
        .collect();

    Ok(GuiResponse {
        success: true,
        message: format!("当前运行 {} 个实例", running_pids.len()),
        pids: running_pids,
    })
}

/// 创建系统托盘
fn create_system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let show = CustomMenuItem::new("show".to_string(), "显示窗口");
    let launch_3 = CustomMenuItem::new("launch_3".to_string(), "启动 3 个实例");
    let launch_5 = CustomMenuItem::new("launch_5".to_string(), "启动 5 个实例");
    let kill_all = CustomMenuItem::new("kill_all".to_string(), "关闭所有实例");

    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(launch_3)
        .add_item(launch_5)
        .add_item(kill_all)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

/// 处理系统托盘事件
fn handle_system_tray_event(app: &tauri::AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick { .. } => {
            // 左键点击显示窗口
            if let Some(window) = app.get_window("main") {
                window.show().unwrap();
                window.set_focus().unwrap();
            }
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "show" => {
                    if let Some(window) = app.get_window("main") {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                "launch_3" => {
                    // 启动 3 个实例
                    let app_handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let state = app_handle.state::<AppState>();
                        let _ = spawn_instances(3, state).await;
                    });
                }
                "launch_5" => {
                    // 启动 5 个实例
                    let app_handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let state = app_handle.state::<AppState>();
                        let _ = spawn_instances(5, state).await;
                    });
                }
                "kill_all" => {
                    // 关闭所有实例
                    let app_handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let state = app_handle.state::<AppState>();
                        let _ = kill_all_instances(state).await;
                    });
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .system_tray(create_system_tray())
        .on_system_tray_event(handle_system_tray_event)
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested { api, .. } => {
                // 关闭窗口时最小化到托盘而不是退出
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            spawn_instances,
            kill_all_instances,
            get_running_instances,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
