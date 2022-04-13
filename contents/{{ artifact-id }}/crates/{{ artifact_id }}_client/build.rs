use std::env;
use std::path::PathBuf;

const SERVICE_PROTO: &str = "../../proto/{{ prefix_name }}/{{ suffix_name }}.proto";
const PROTO_DIR: &str = "../../proto";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed={}", PROTO_DIR);

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("{{ prefix_name }}.{{ suffix_name }}.bin"))
        .build_server(false)
        .build_client(true)
        .compile(&[SERVICE_PROTO],
                 &[PROTO_DIR]
        )
        .unwrap();

    Ok(())
}
