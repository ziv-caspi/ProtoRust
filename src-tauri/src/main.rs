// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod async_jobs;
mod events;
mod functions;
mod proto_helpers;
mod rabbitmq;

use async_jobs::{CancelSignalChannel, Shared};
use rabbitmq::connection::ConnectionMutex;
use std::sync::Arc;
use tokio::sync::Mutex;

fn main() {
    tauri::Builder::default()
        .manage(ConnectionMutex(Shared::new(None)))
        .manage(CancelSignalChannel::new())
        .invoke_handler(tauri::generate_handler![
            functions::load_proto,
            functions::gen_default_json,
            functions::publish_rabbitmq_message,
            functions::publish_message,
            functions::close_connection,
            functions::create_connection,
            functions::cancel_publish,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
