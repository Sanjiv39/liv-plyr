mod proxy;
use proxy::{proxy_request, start_media_proxy};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![greet, proxy_request])
        .setup(|app| {
            let app_handle = app.handle();

            // Start a local proxy server for media
            tauri::async_runtime::spawn(start_media_proxy());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
