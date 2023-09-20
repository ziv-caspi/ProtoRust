use crate::{
    events::TauriEventEmitter,
    proto_helpers,
    rabbitmq::{
        self,
        connection::{self, create_channel, ConnectionMutex},
        publishing::PublishStrategy,
    },
};
use std::path::Path;

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
    connection: tauri::State<'_, ConnectionMutex>,
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
pub async fn close_connection(connection: tauri::State<'_, ConnectionMutex>) -> Result<(), String> {
    *connection
        .0
        .try_lock()
        .map_err(|_| String::from("cannot close connection while it is being used"))? = None;

    Ok(())
}

#[tauri::command]
pub async fn publish_message(
    includes_dir: &str,
    proto_file: &str,
    message_name: &str,
    json: &str,
    routing_key: &str,
    strategy: PublishStrategy,
    connection_mutex: tauri::State<'_, ConnectionMutex>,
    window: tauri::Window,
) -> Result<(), String> {
    let lock = connection_mutex
        .0
        .try_lock()
        .map_err(|_| String::from("cannot change connection while it is being used"))?;
    let conn = lock.as_ref().ok_or("no rabbitmq connection")?;

    let body = proto_helpers::serializer::serialize_json_into_binary(
        Path::new(includes_dir),
        Path::new(proto_file),
        message_name,
        json,
    )
    .map_err(|e| e.to_string())?;

    let emitter = TauriEventEmitter {
        window_handle: window,
    };

    let _ = rabbitmq::publishing::publish_by_strategy(
        strategy,
        &conn.target,
        &conn.channel,
        body,
        routing_key,
        emitter,
    );

    Ok(())
}
