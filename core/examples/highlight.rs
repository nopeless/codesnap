use codesnap::config::{CodeBuilder, CodeSnap, Content, HighlightLine};

pub fn main() -> anyhow::Result<()> {
    let code_content = Content::Code(
        CodeBuilder::default()
            .content(
                r##"pub fn main() {
    println!("Hello, world!");
    println!("Hello, CodeSnap!");
}"##,
            )
            .language("rust")
            .highlight_lines(vec![
                HighlightLine::Single(2, "#ff6b6b30".to_string()),
                HighlightLine::Single(3, "#2ecc7130".to_string()),
            ])
            .build()?,
    );

    let snapshot = CodeSnap::from_default_theme()?
        .content(code_content)
        .build()?
        .create_snapshot()?;

    // Copy the snapshot data to the clipboard
    snapshot.raw_data()?.copy()
}
