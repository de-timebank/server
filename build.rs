const SERIAL_DESERIAL_ATTR: &str = "#[derive(serde::Serialize, serde::Deserialize)]";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .protoc_arg("--experimental_allow_proto3_optional")
        .type_attribute(".", SERIAL_DESERIAL_ATTR)
        .include_file("proto.rs")
        .compile(
            &[
                "proto/auth.proto",
                "proto/user.proto",
                "proto/collection/rating.proto",
                "proto/collection/service-request.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
