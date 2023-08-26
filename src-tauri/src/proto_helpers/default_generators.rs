use anyhow::{anyhow, Result};
use protobuf_json_mapping::PrintOptions;
use std::path::Path;

use super::shared;

pub fn generate_message_default_proto_json(
    includes_dir: &Path,
    proto_file: &Path,
    message_name: &str,
) -> Result<String> {
    let file_descriptor = shared::get_proto_file_descriptor(includes_dir, proto_file)?;

    let mmm_descriptor = file_descriptor
        .message_by_package_relative_name(message_name)
        .ok_or(anyhow!("could not find message name"))?;
    let message = mmm_descriptor.new_instance();

    let json = protobuf_json_mapping::print_to_string_with_options(
        &*message,
        &PrintOptions {
            proto_field_name: true,
            always_output_default_values: true,
            ..Default::default()
        },
    )?;
    let pretty_json = jsonxf::pretty_print(&json).map_err(|err| anyhow!(err))?;
    Ok(pretty_json)
}

pub fn generate_service_method_default_proto_json(
    includes_dir: &Path,
    proto_file: &Path,
    service_name: &str,
    method_name: &str,
) -> Result<String> {
    let descriptor = shared::get_proto_file_descriptor(includes_dir, proto_file)?;

    let service = descriptor
        .services()
        .filter(|service| service.proto().name() == service_name)
        .next()
        .ok_or(anyhow!("could not find specified service"))?;

    let method = service
        .methods()
        .filter(|method| method.proto().name() == method_name)
        .next()
        .ok_or(anyhow!("could not find specified method"))?;

    let input_instance = method.input_type().new_instance();
    let json = protobuf_json_mapping::print_to_string_with_options(
        &*input_instance,
        &PrintOptions {
            proto_field_name: true,
            always_output_default_values: true,
            ..Default::default()
        },
    )?;

    Ok(json)
}
