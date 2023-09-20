// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod events;
mod functions;
mod proto_helpers;
mod rabbitmq;

use std::sync::Mutex;

use rabbitmq::connection::ConnectionMutex;

fn main() {
    tauri::Builder::default()
        .manage(ConnectionMutex(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            functions::load_proto,
            functions::gen_default_json,
            functions::publish_rabbitmq_message,
            functions::publish_message,
            functions::close_connection,
            functions::create_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
