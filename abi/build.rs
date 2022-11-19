use std::process::Command;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .type_attribute(
            "trigger.DatabaseConfig",
            "#[derive(serde::Serialize,serde::Deserialize)]",
        )
        .type_attribute(
            "trigger.AuthenticationConfig",
            "#[derive(serde::Serialize,serde::Deserialize)]",
        )
        .type_attribute(
            "trigger.ScheduledConfig",
            "#[derive(serde::Serialize,serde::Deserialize)]",
        )
        .type_attribute("trigger.TriggerQuery", "#[derive(serde::Serialize)]")
        .compile(
            &["protos/trigger.proto", "protos/function.proto"],
            &["protos"],
        )
        .unwrap();

    Command::new("cargo").args(["fmt"]).output().unwrap();

    println!("cargo:rerun-if-changed=protos/");
}
