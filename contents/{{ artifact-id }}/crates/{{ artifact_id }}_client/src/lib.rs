pub mod proto {
    tonic::include_proto!("{{ prefix_name }}.service");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("{{ prefix_name }}.service");
}
