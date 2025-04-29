use codesnap::config::{CodeSnap, CommandLineContentBuilder, Content};
use std::process::Command;

pub fn main() -> anyhow::Result<()> {
    let command = "echo \"Hello, World!\"";
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    let command_line_content = CommandLineContentBuilder::default()
        .full_command(command)
        .content(String::from_utf8_lossy(&output.stdout).into_owned())
        .build()
        .unwrap();

    let command_content = Content::CommandOutput(vec![command_line_content]);
    let snapshot = CodeSnap::from_default_theme()?
        .content(command_content)
        .build()?
        .create_snapshot()?;

    // Copy the snapshot data to the clipboard
    snapshot.raw_data()?.copy()
}
