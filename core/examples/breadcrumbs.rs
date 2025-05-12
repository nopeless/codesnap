use codesnap::config::{BreadcrumbsBuilder, CodeBuilder, CodeConfigBuilder, CodeSnap, Content};

pub fn main() -> anyhow::Result<()> {
    let code_content = Content::Code(
        CodeBuilder::default()
            .content(r#"pub fn main() {}"#)
            .language("rust")
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
        .content(code_content)
        .code_config(code_config)
        .build()?
        .create_snapshot()?;

    // Copy the snapshot data to the clipboard
    snapshot.raw_data()?.copy()
}
