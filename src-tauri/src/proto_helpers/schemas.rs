use std::path::Path;

use anyhow::Result;

use crate::proto_helpers::shared;

pub fn get_messges_of_proto(dir_path: &Path, file_path: &Path) -> Result<Vec<String>> {
    let descriptor = shared::get_proto_file_descriptor(dir_path, file_path)?;
    let names: Vec<String> = descriptor
        .messages()
        .map(|message| message.name().to_owned())
        .collect();
    Ok(names)
}
