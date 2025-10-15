// GUI 版本 - Tauri 图形界面

// Prevents additional console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{Manager, WindowEvent};
use wecom_multi_open::{platform, SpawnRequest, AppType};

#[cfg(target_os = "windows")]
use wecom_multi_open::wecom_manager::{WeComManager, WeComInstance};

/// 隔离模式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum IsolationMode {
    Simple,    // 简单模式 (无隔离)
    Sandboxie, // Sandboxie沙盒模式 (Windows)
}

/// 应用状态
#[derive(Clone)]
struct AppState {
    /// 当前运行的实例 PID 列表
    pids: Arc<Mutex<Vec<u32>>>,
    /// 退出时是否保留实例 (true = 保留,false = 关闭)
    keep_on_exit: Arc<Mutex<bool>>,
    /// Sandboxie 实例列表 (仅Windows)
    #[cfg(target_os = "windows")]
    sandboxie_instances: Arc<Mutex<Vec<WeComInstance>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            pids: Arc::new(Mutex::new(Vec::new())),
            keep_on_exit: Arc::new(Mutex::new(true)), // 默认保留实例
            #[cfg(target_os = "windows")]
            sandboxie_instances: Arc::new(Mutex::new(Vec::new())),
        }
    }
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
    app_type: Option<String>,
    isolation_mode: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<GuiResponse, String> {
    // 解析应用类型
    let app_type_enum = match app_type.as_deref() {
        Some("wechat") | Some("WeChat") => AppType::WeChat,
        _ => AppType::WeCom, // 默认企业微信
    };

    let app_name = match app_type_enum {
        AppType::WeCom => "企业微信",
        AppType::WeChat => "微信",
    };

    // 解析隔离模式
    let isolation = match isolation_mode.as_deref() {
        Some("sandboxie") => IsolationMode::Sandboxie,
        _ => IsolationMode::Simple,
    };

    println!("收到启动请求: {} {} 个实例 (模式: {:?})", app_name, count, isolation);

    // Windows + Sandboxie 模式
    #[cfg(target_os = "windows")]
    if matches!(isolation, IsolationMode::Sandboxie) {
        match WeComManager::new() {
            Ok(manager) => {
                match manager.spawn_multiple(count) {
                    Ok(instances) => {
                        // 保存Sandboxie实例信息
                        let mut sb_instances = state.sandboxie_instances.lock().unwrap();
                        let mut pids = state.pids.lock().unwrap();

                        for instance in &instances {
                            if let Some(pid) = instance.pid {
                                pids.push(pid);
                            }
                        }

                        sb_instances.extend(instances.clone());

                        return Ok(GuiResponse {
                            success: true,
                            message: format!("✅ Sandboxie模式: 成功启动 {} 个隔离实例!", instances.len()),
                            pids: pids.clone(),
                        });
                    }
                    Err(e) => {
                        return Ok(GuiResponse {
                            success: false,
                            message: format!("Sandboxie启动失败: {}. 请检查是否已安装Sandboxie-Plus", e),
                            pids: vec![],
                        });
                    }
                }
            }
            Err(e) => {
                return Ok(GuiResponse {
                    success: false,
                    message: format!("无法初始化Sandboxie: {}. 请先安装Sandboxie-Plus: https://github.com/sandboxie-plus/Sandboxie/releases", e),
                    pids: vec![],
                });
            }
        }
    }

    // 简单模式 (所有平台)
    // 检测已存在的进程
    #[cfg(target_os = "windows")]
    let existing_pids = platform::find_processes_by_type(app_type_enum.clone());
    #[cfg(not(target_os = "windows"))]
    let existing_pids = Vec::new();

    if !existing_pids.is_empty() {
        println!("⚠ 检测到 {} 个已运行的{}实例: {:?}", existing_pids.len(), app_name, existing_pids);

        // 将已存在的进程添加到管理列表
        let mut pids = state.pids.lock().unwrap();
        for &pid in &existing_pids {
            if !pids.contains(&pid) {
                pids.push(pid);
            }
        }
    }

    let req = SpawnRequest {
        count,
        app_path: None,
        app_type: Some(app_type_enum),
        instance_configs: None,
    };

    match platform::spawn_multiple(req).await {
        Ok(response) => {
            // 保存新启动的 PID 到状态
            let mut pids = state.pids.lock().unwrap();
            pids.extend_from_slice(&response.pids);

            let total_instances = pids.len();

            let mode_desc = if matches!(isolation, IsolationMode::Sandboxie) {
                "隔离模式"
            } else {
                "简单模式"
            };

            Ok(GuiResponse {
                success: true,
                message: format!("✅ {}: 成功启动 {} 个新实例! 当前共 {} 个实例运行", mode_desc, response.success, total_instances),
                pids: pids.clone(),
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

/// Tauri 命令: 设置退出时是否保留实例
#[tauri::command]
async fn set_keep_on_exit(
    keep: bool,
    state: tauri::State<'_, AppState>,
) -> Result<GuiResponse, String> {
    *state.keep_on_exit.lock().unwrap() = keep;

    Ok(GuiResponse {
        success: true,
        message: format!("已设置: 退出时{}", if keep { "保留实例" } else { "关闭所有实例" }),
        pids: vec![],
    })
}

/// Tauri 命令: 获取退出设置
#[tauri::command]
async fn get_keep_on_exit(
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    Ok(*state.keep_on_exit.lock().unwrap())
}

/// Tauri 命令: 检测Sandboxie是否可用
#[tauri::command]
async fn check_sandboxie_available() -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        match WeComManager::new() {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(false)
    }
}

/// Tauri 命令: 清除路径缓存
#[tauri::command]
async fn clear_path_cache() -> Result<GuiResponse, String> {
    #[cfg(target_os = "windows")]
    {
        platform::clear_path_cache();
        Ok(GuiResponse {
            success: true,
            message: "✓ 已清除路径缓存,下次启动将重新检测".to_string(),
            pids: vec![],
        })
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(GuiResponse {
            success: false,
            message: "此功能仅支持Windows平台".to_string(),
            pids: vec![],
        })
    }
}

/// 清理所有子进程
fn cleanup_all_processes(state: &AppState) {
    let keep_on_exit = *state.keep_on_exit.lock().unwrap();

    if keep_on_exit {
        println!("应用退出,保留所有企业微信实例运行");
        return;
    }

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
            let _window = app.get_webview_window("main").unwrap();

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
            set_keep_on_exit,
            get_keep_on_exit,
            check_sandboxie_available,
            clear_path_cache,
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
