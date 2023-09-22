// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod events;
mod functions;
mod proto_helpers;
mod rabbitmq;

use rabbitmq::connection::ConnectionMutex;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

pub struct CancelSignalChannel {
    base_tx: mpsc::Sender<bool>,
    pub rx: Arc<Mutex<mpsc::Receiver<bool>>>,
}

impl CancelSignalChannel {
    pub fn new() -> CancelSignalChannel {
        let (tx, rx) = mpsc::channel(1);
        CancelSignalChannel {
            base_tx: tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }

    pub fn tx(&self) -> mpsc::Sender<bool> {
        self.base_tx.clone()
    }

    pub fn rx_mutex(&self) -> Arc<Mutex<mpsc::Receiver<bool>>> {
        self.rx.clone()
    }
}

fn main() {
    tauri::Builder::default()
        .manage(ConnectionMutex(Arc::new(Mutex::new(None))))
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
