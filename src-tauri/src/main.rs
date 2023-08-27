// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod functions;
mod proto_helpers;
mod rabbitmq;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            functions::load_proto,
            functions::gen_default_json,
            functions::publish_rabbitmq_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
