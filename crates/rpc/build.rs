fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut b = tonic_build::configure().protoc_arg("--experimental_allow_proto3_optional");

    if cfg!(feature = "server") {
        b = b.build_server(true).out_dir("src/generated_full");
    } else {
        b = b.build_server(false).out_dir("src/generated");
    }

    b.compile(&["../../proto/example.proto"], &["../../proto"])?;

    println!("cargo:rerun-if-changed=../../proto");

    Ok(())
}
