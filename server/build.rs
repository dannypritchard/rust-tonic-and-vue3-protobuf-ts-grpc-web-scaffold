extern crate serde;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        // It is included in the out/user.rs but the compiler says it can not find them.
        .type_attribute(
            "voting.Vote",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .compile(&["_protos/voting.proto"], &["proto/voting"])?;
    Ok(())
}
