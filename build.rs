fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/registry/pb")
        .compile(
            &["proto/helloworld/helloworld.proto"],
            &["proto/helloworld"],
        )?;
    Ok(())
}
