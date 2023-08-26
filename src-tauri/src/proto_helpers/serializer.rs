use anyhow::{anyhow, Result};
use std::path::Path;

use crate::proto_helpers::shared;

pub fn serialize_json_into_binary(
    includes_dir: &Path,
    proto_file: &Path,
    message_name: &str,
    json: &str,
) -> Result<Vec<u8>> {
    let descriptor = shared::get_proto_file_descriptor(includes_dir, proto_file)?;
    let message_descriptor = descriptor
        .message_by_package_relative_name(message_name)
        .ok_or(anyhow!("could not specified message name"))?;

    let mut instance = message_descriptor.new_instance();
    protobuf_json_mapping::merge_from_str(&mut *instance, json)?;

    println!("{}", protobuf_json_mapping::print_to_string(&*instance)?);

    let bytes = instance.write_to_bytes_dyn()?;
    Ok(bytes)
}
