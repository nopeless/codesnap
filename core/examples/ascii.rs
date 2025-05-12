use codesnap::config::{BreadcrumbsBuilder, CodeBuilder, CodeConfigBuilder, CodeSnap, Content};

pub fn main() -> anyhow::Result<()> {
    let code_content = Content::Code(
        CodeBuilder::default()
            .content(
                r#"pub fn main() {
    println!("Hello, world!");
}"#,
            )
            .language("rust")
            .start_line_number(10u32)
            .file_path("core/examples/breadcrumbs.rs")
            .build()?,
    );

    let breadcrumbs = BreadcrumbsBuilder::default()
        .enable(true)
        .separator(" > ")
        .build()?;
    let code_config = CodeConfigBuilder::default()
        .breadcrumbs(breadcrumbs)
        .build()?;

    let snapshot = CodeSnap::from_default_theme()?
        .code_config(code_config)
        .content(code_content)
        .build()?
        .create_ascii_snapshot()?;

    // Copy the snapshot data to the clipboard
    snapshot.raw_data()?.copy()
}
