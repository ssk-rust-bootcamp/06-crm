use anyhow::Result;
use std::{fs, process::Command};

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;
    let builder = tonic_build::configure();
    builder
        .out_dir("src/pb")
        .compile(
            &[
                "../protos/notification/messages.proto",
                "../protos/notification/rpc.proto",
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
