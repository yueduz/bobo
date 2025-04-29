use std::sync::Arc;
use tauri::ipc::Channel;
use tokio::sync::Notify;
mod tcp;

// 创建一个全局的Notify对象
lazy_static::lazy_static! {
     static ref NOTIFY: Arc<Notify> = Arc::new(Notify::new());
}
// https://v2.tauri.app/develop/calling-frontend/#channels

// 使用时
#[tauri::command]
async fn start_fifo(on_event: Channel<Vec<u8>>, length: u32) -> String {
    println!("start_fifo: length = {}", length);
    // 创建一个新的 Server 实例，绑定到 "127.0.0.1:8080"
    tcp::start_server(on_event, length as usize, NOTIFY.clone(), "127.0.0.1:9333")
        .await
        .unwrap();
    return "ok".to_string();
}

#[tauri::command]
async fn stop_tcp() {
    println!("sending notification");
    NOTIFY.notify_waiters();
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![start_fifo, stop_tcp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
