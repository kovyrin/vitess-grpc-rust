fn main() {
    let protos_dir = std::fs::read_dir("proto").unwrap();
    let mut proto_files = Vec::new();

    protos_dir.for_each(|file| {
        let file = file.unwrap();
        let path = file.path();
        let path_str = path.to_str().unwrap();
        if path_str.ends_with(".proto") {
            println!("cargo:rerun-if-changed={}", path_str);
            proto_files.push(path.clone());
        }
    });

    // create directory for generated files
    std::fs::create_dir_all("src/generated").unwrap();

    tonic_build::configure()
        .build_server(false)
        .out_dir("src/generated")
        .compile(&proto_files, &["proto/"])
        .unwrap();
}
