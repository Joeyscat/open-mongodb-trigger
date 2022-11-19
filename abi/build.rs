use std::process::Command;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["protos/trigger.proto", "function.proto"], &["protos"])
        .unwrap();

    Command::new("cargo").args(["fmt"]).output().unwrap();

    println!("cargo:rerun-if-changed=protos/");
}
