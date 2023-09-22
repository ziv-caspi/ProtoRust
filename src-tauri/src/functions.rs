use crate::{
    events::TauriEventEmitter,
    proto_helpers,
    rabbitmq::{
        self,
        connection::{self, create_channel, ConnectionMutex},
        publishing::PublishStrategy,
    },
    CancelSignalChannel,
};
use std::{path::Path, sync::Arc};
use tauri::State;
use tokio::sync::mpsc::{Receiver, Sender};

#[tauri::command]
pub async fn load_proto(deps_path: &str, file_path: &str) -> Result<Vec<String>, String> {
    let messages =
        proto_helpers::schemas::get_messges_of_proto(&Path::new(deps_path), &Path::new(file_path));

    match messages {
        Ok(vec) => return Ok(vec),
        Err(e) => return Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn gen_default_json(
    deps_path: &str,
    file_path: &str,
    message_name: &str,
) -> Result<String, String> {
    match proto_helpers::default_generators::generate_message_default_proto_json(
        &Path::new(deps_path),
        &Path::new(file_path),
        message_name,
    ) {
        Ok(json) => return Ok(json),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn publish_rabbitmq_message(
    includes_dir: &str,
    proto_file: &str,
    message_name: &str,
    params: rabbitmq::connection::RabbitMqParamaters,
    json: &str,
) -> Result<(), String> {
    let body = proto_helpers::serializer::serialize_json_into_binary(
        Path::new(includes_dir),
        Path::new(proto_file),
        message_name,
        json,
    )
    .map_err(|e| e.to_string())?;

    rabbitmq::connection::basic_publish_once(params, body)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn create_connection(
    params: rabbitmq::connection::RabbitMqParamaters,
    connection: State<'_, ConnectionMutex>,
) -> Result<(), String> {
    let channel = create_channel(&params)
        .await
        .map_err(|_| String::from("could not connect to rabbitmq"))?;

    *connection
        .0
        .try_lock()
        .map_err(|_| String::from("cannot change connection while it is being used"))? =
        Some(channel);

    Ok(())
}

#[tauri::command]
pub async fn close_connection(connection: State<'_, ConnectionMutex>) -> Result<(), String> {
    *connection
        .0
        .try_lock()
        .map_err(|_| String::from("cannot close connection while it is being used"))? = None;

    Ok(())
}

#[tauri::command]
pub async fn publish_message(
    includes_dir: String,
    proto_file: String,
    message_name: String,
    json: String,
    routing_key: String,
    strategy: PublishStrategy,
    connection_mutex: State<'_, ConnectionMutex>,
    cancel_channel: State<'_, CancelSignalChannel>,
    window: tauri::Window,
) -> Result<(), String> {
    println!("{:?}", strategy);

    let body = proto_helpers::serializer::serialize_json_into_binary(
        Path::new(&includes_dir),
        Path::new(&proto_file),
        &message_name,
        &json,
    )
    .map_err(|e| e.to_string())?;

    let emitter = TauriEventEmitter {
        window_handle: window,
    };

    let safe_ref = Arc::clone(&connection_mutex.0);
    let cancel = cancel_channel.rx_mutex();
    tokio::spawn(async move {
        let mut cancel_guard = cancel.try_lock().unwrap();
        let connection_guard = safe_ref
            .try_lock()
            .map_err(|_| String::from("cannot change connection while it is being used"))
            .unwrap();
        let connection = connection_guard
            .as_ref()
            .ok_or("no rabbitmq connection")
            .unwrap();

        println!("accuired connection. target: {:?}", connection.target);
        println!("trying to publish");
        let _ = rabbitmq::publishing::publish_by_strategy(
            strategy,
            &connection.target,
            &connection.channel,
            body,
            &routing_key,
            emitter,
            &mut *cancel_guard,
        )
        .await;
    });

    Ok(())
}

#[tauri::command]
pub async fn cancel_publish(cancel_channel: State<'_, CancelSignalChannel>) -> Result<(), String> {
    println!("got cancel command");
    let sender = cancel_channel.tx();
    sender.send(true).await.map_err(|e| e.to_string())?;
    Ok(())
}
