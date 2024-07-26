use std::{fs, process::Command};

use anyhow::Result;
fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;
    let builder = tonic_build::configure();
    builder
        .out_dir("src/pb")
        .compile(&["../protos/crm/crm.proto"], &["../protos"])?;

    Command::new("cargo")
        .arg("fmt")
        .spawn()
        .expect("executing cargo fmt failed");
    Ok(())
}
