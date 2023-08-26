use anyhow::{anyhow, Result};
use protobuf::{descriptor::FileDescriptorProto, reflect::FileDescriptor};
use std::path::Path;

pub fn get_proto_file_descriptor(
    includes_dir: &Path,
    proto_file: &Path,
) -> Result<FileDescriptor, anyhow::Error> {
    let mut file_descriptor_protos = protobuf_parse::Parser::new()
        .pure()
        .includes(&[includes_dir])
        .input(proto_file)
        .parse_and_typecheck()?
        .file_descriptors;
    let file_descriptor_proto: FileDescriptorProto = file_descriptor_protos
        .pop()
        .ok_or(anyhow!("could not load proto file"))?;
    let file_descriptor: FileDescriptor = FileDescriptor::new_dynamic(file_descriptor_proto, &[])?;
    Ok(file_descriptor)
}
