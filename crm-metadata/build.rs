use std::{fs, process::Command};

use anyhow::Result;
use proto_builder_trait::tonic::BuilderAttributes;
fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;

    let builder = tonic_build::configure();
    builder
        .out_dir("src/pb")
        .with_type_attributes(&["MaterializeRequest"], &[r#"#[derive(Eq, Hash)]"#])
        .compile(
            &[
                "../protos/metadata/messages.proto",
                "../protos/metadata/rpc.proto",
            ],
            &["../protos"],
        )
        .unwrap();

    Command::new("cargo")
        .arg("fmt")
        .spawn()
        .expect("executing cargo fmt failed");
    Ok(())
}
