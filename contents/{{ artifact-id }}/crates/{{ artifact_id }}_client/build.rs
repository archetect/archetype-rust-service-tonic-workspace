use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("{{ prefix_name }}.service.bin"))
        .build_server(false)
        .build_client(true)
        .compile(&["../../proto/{{ prefix_name }}/service.proto"],
                 &["../../proto"]
        )
        .unwrap();

    Ok(())
}
