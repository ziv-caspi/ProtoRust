use std::path::Path;

use serde::de;
use tauri::api::file;

use crate::{
    proto_helpers,
    rabbitmq::{self, connection},
};

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
