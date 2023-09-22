use anyhow::Result;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self},
    Mutex,
};

use crate::{
    events::TauriEventEmitter,
    rabbitmq::{self, connection::RabbitMqConnection, publishing::PublishStrategy},
};

pub struct Shared<T>(pub Arc<Mutex<T>>);

impl<T> Shared<T> {
    pub fn new(t: T) -> Shared<T> {
        Shared(Arc::new(Mutex::new(t)))
    }
}

pub struct CancelSignalChannel {
    base_tx: mpsc::Sender<bool>,
    pub rx: Shared<mpsc::Receiver<bool>>,
}

impl CancelSignalChannel {
    pub fn new() -> CancelSignalChannel {
        let (tx, rx) = mpsc::channel(1);
        CancelSignalChannel {
            base_tx: tx,
            rx: Shared::new(rx),
        }
    }

    pub fn tx(&self) -> mpsc::Sender<bool> {
        self.base_tx.clone()
    }

    pub fn rx_mutex(&self) -> Arc<Mutex<mpsc::Receiver<bool>>> {
        self.rx.0.clone()
    }

    pub fn get_token(&self) -> CancelToken {
        CancelToken {
            rx: self.rx.0.clone(),
        }
    }
}

pub struct CancelToken {
    rx: Arc<Mutex<mpsc::Receiver<bool>>>,
}

impl CancelToken {
    pub fn reset(&self) {
        let receiver = &mut *self.rx.try_lock().unwrap();
        loop {
            if let Err(e) = receiver.try_recv() {
                break;
            }
        }
    }

    pub fn should_cancel(&self) -> bool {
        let receiver = &mut *self.rx.try_lock().unwrap();
        if let Ok(_) = receiver.try_recv() {
            return true;
        }

        return false;
    }
}

pub struct AppState {
    pub rabbit_mutex: Shared<Option<RabbitMqConnection>>,
    pub cancel_token: CancelToken,
    pub window: tauri::Window,
}

pub struct PublishParams {
    pub includes_dir: String,
    pub proto_file: String,
    pub message_name: String,
    pub json: String,
    pub routing_key: String,
    pub strategy: PublishStrategy,
    pub body: Vec<u8>,
}

pub async fn start_publishing(app_state: AppState, publish_params: PublishParams) -> Result<()> {
    tokio::spawn(async move {
        let connection_guard = app_state.rabbit_mutex.0.try_lock().unwrap();
        let connection = connection_guard
            .as_ref()
            .ok_or("no rabbitmq connection")
            .unwrap();

        let emitter = TauriEventEmitter {
            window_handle: app_state.window,
        };

        let mut cancel = app_state.cancel_token;
        let _ = rabbitmq::publishing::publish_by_strategy(
            publish_params.strategy,
            &connection.target,
            &connection.channel,
            publish_params.body,
            &publish_params.routing_key,
            emitter,
            &mut cancel,
        )
        .await;
    });
    Ok(())
}