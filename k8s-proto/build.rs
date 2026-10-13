use std::{error::Error, path::PathBuf, env};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut proto_files = vec![];

    proto_files.extend(get_etcd_proto_path());

    let proto_base_path = get_proto_base_path();
    let proto_base_path_str = proto_base_path.to_str().unwrap();
    let proto_root = &[proto_base_path_str];

    let build_server = cfg!(feature = "build-server");
    let build_client = cfg!(feature = "build-client");

    tonic_prost_build::configure()
    .file_descriptor_set_path(out_dir.join("proto_file_descriptor.bin"))
    .build_server(build_server)
    .build_client(build_client)
    .compile_protos(&proto_files, proto_root)
    .expect("Failed to compile proto files");

    Ok(())

}

fn get_proto_base_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    //let build_type = env::var("PROFILE").unwrap();
    std::path::Path::new(&manifest_dir_string).join("proto")
}

fn get_etcd_proto_path() -> Vec<&'static str> {
    vec![
        "etcd/messages/kv.proto",
        "etcd/rpc.proto"
    ]
}