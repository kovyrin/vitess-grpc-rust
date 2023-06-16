fn main() {
    let proto_files = std::fs::read_dir("proto")
        .unwrap()
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();

    // create directory for generated files
    std::fs::create_dir_all("src/generated").unwrap();

    tonic_build::configure()
        .build_server(false)
        .out_dir("src/generated")
        .compile(&proto_files, &["proto/"])
        .unwrap();
}
