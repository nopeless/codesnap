mod code;
mod code_config;
mod config;
mod egg;
mod highlight;
mod logger;
mod range;
mod watermark;
mod window;

use std::fs;
use std::fs::read_to_string;
use std::path::Path;

use anyhow::bail;
use anyhow::Context;
use clap::value_parser;
use clap::Parser;
use code::create_code;
use code_config::create_code_config;
use codesnap::assets::Assets;
use codesnap::assets::AssetsURL;
use codesnap::config::CodeSnap;
use codesnap::config::SnapshotConfig;
use config::CodeSnapCLIConfig;
use egg::say;
use theme_converter::{parser::Parser as ThemeParser, vscode};
use watermark::create_watermark;
use window::create_window;

// use std::thread;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub const STDIN_CODE_DEFAULT_CHAR: &'static str = "-";

/// CodeSnap is a CLI tool to generate beautiful snapshots of your code from terminal.
#[derive(Parser)]
#[command(bin_name = "codesnap")]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct CLI {
    /// Path to the file to snapshot
    #[arg(short = 'f', long)]
    from_file: Option<String>,

    /// Code snippet for snapshot
    #[arg(short = 'c', long, default_missing_value = STDIN_CODE_DEFAULT_CHAR, require_equals=false, num_args=0..=1, value_parser=value_parser!(String), value_name="Code")]
    from_code: Option<String>,

    #[arg(long)]
    from_clipboard: bool,

    /// Output path for the snapshot.
    /// Available value:
    ///
    /// - clipboard: Copy the snapshot to clipboard
    /// - file path: Save the snapshot to the file path
    ///
    /// Currently CodeSnap supports SVG format and PNG format
    /// If output is directory, CodeSnap will generate a temporary file name to save the snapshot
    /// to the directory.
    #[arg(short, long)]
    output: String,

    /// Executing a command and taking the output as the code snippet.
    #[arg(long, short, num_args=1..)]
    execute: Vec<String>,

    /// Skip run the command to get output, just take the command as the input
    #[arg(long)]
    skip: bool,

    /// You can set the range of the code snippet to display
    /// for example, display the 3rd to 5th:
    /// 3:5
    /// The syntax is similar to the range in Python or Golang, so basically you can use 3: to
    /// represent the 3rd to the end, or use :5 to represent the start to the 5th.
    /// This option is useful when you use the `from_file` option as the input, and you just want
    /// to display part of the code snippet.
    #[arg(long)]
    range: Option<String>,

    /// Font family for the code snippet
    #[arg(long)]
    code_font_family: Option<String>,

    /// Code theme for the code snippet
    #[arg(long)]
    code_theme: Option<String>,

    /// Breadcrumbs is a useful and unique feature in CodeSnap, it shows the path of the file
    /// so that users can know where the code snippet comes from.
    #[arg(long, default_value = "false")]
    has_breadcrumbs: Option<bool>,

    #[arg(long, default_value = "false")]
    has_line_number: bool,

    /// Breadcrumbs separator is the character to separate the path in breadcrumbs
    /// Default is `/`
    #[arg(long)]
    breadcrumbs_separator: Option<String>,

    /// Breadcrumbs font family
    #[arg(long)]
    breadcrumbs_font_family: Option<String>,

    /// Breadcrumbs font color
    #[arg(long)]
    breadcrumbs_color: Option<String>,

    /// Set start line number to display line numbers
    #[arg(long)]
    start_line_number: Option<u32>,

    /// Line number font color
    #[arg(long, default_value = "#495162")]
    line_number_color: String,

    /// Delete lines will be marked with a red line
    #[arg(long, short, num_args=1..)]
    delete_line: Vec<String>,

    /// Delete line color
    #[arg(long, default_value = "#ff6b6b30")]
    delete_line_color: String,

    /// New lines will be marked with a green line
    #[arg(long, short, num_args=1..)]
    add_line: Vec<String>,

    /// New line color
    #[arg(long, default_value = "#2ecc7130")]
    add_line_color: String,

    /// Convenient version of `--raw-highlight-lines` option, you can set the highlight range
    /// with a simple syntax, for example, highlight the 3rd to 5th lines:
    /// 3:5
    #[arg(long)]
    highlight_range: Option<String>,

    #[arg(long)]
    relative_highlight_range: bool,

    /// Highlight color for the highlighted code lines
    #[arg(long, default_value = "#ffffff10")]
    highlight_range_color: String,

    /// CodeSnap allows users to highlight multiple lines with custom highlight color in the code snippet
    /// The content of highlight_lines is JSON string, for example highlight the first line and the
    /// 3rd to 5th lines:
    /// "[
    ///   [1, "#ff0000"],
    ///   [3, 5, "#00ff00"]
    /// ]"
    ///
    #[arg(long)]
    raw_highlight_lines: Option<String>,

    /// Set the language of the code snippet, If you using the `file` option, CodeSnap will
    /// automatically detect the language from the file extension.
    #[arg(long, short)]
    language: Option<String>,

    /// Used to detect the language of the code snippet, if you want to set language manually,
    /// use `--language` or `-l` option.
    #[arg(long)]
    file_path: Option<String>,

    /// Set watermark for the code snippet
    #[arg(long, short)]
    watermark: Option<String>,

    /// Watermark font family
    #[arg(long)]
    watermark_font_family: Option<String>,

    /// Watermark font color
    #[arg(long)]
    watermark_color: Option<String>,

    /// Set window shadow radius
    #[arg(long)]
    shadow_radius: Option<f32>,

    #[arg(long)]
    shadow_color: Option<String>,

    /// Display MacOS style window bar
    #[arg(long)]
    mac_window_bar: Option<bool>,

    /// Display window border
    #[arg(long, default_value_t = true)]
    has_border: bool,

    /// Window border color
    #[arg(long, default_value = "#ffffff30")]
    border_color: String,

    /// Set horizontal margin of window
    #[arg(long)]
    margin_x: Option<f32>,

    /// Set vertical margin of window
    #[arg(long)]
    margin_y: Option<f32>,

    /// Set title of the window
    #[arg(long)]
    title: Option<String>,

    /// CodeSnap supports scaling the snapshot, the default scale factor is 3 for better quality
    #[arg(long, default_value_t = 3)]
    scale_factor: u8,

    /// Title font family
    #[arg(long)]
    title_font_family: Option<String>,

    /// Title font color
    #[arg(long)]
    title_color: Option<String>,

    /// Set background color of the snapshot
    #[arg(long)]
    background: Option<String>,

    #[arg(long, value_parser=["ascii", "image"], default_value="image")]
    r#type: String,

    #[arg(long)]
    config: Option<String>,
}

fn output_snapshot(cli: &CLI, snapshot: &SnapshotConfig) -> anyhow::Result<String> {
    // Save snapshot to clipboard
    if cli.output == "clipboard" {
        match cli.r#type.as_str() {
            "ascii" => {
                snapshot.create_ascii_snapshot()?.raw_data()?.copy()?;
            }
            "image" => {
                snapshot.create_snapshot()?.raw_data()?.copy()?;
            }
            _ => {
                bail!("Invalid snapshot type");
            }
        }

        return Ok("Snapshot copied to clipboard".to_string());
    }

    let image_snapshot = snapshot.create_snapshot()?;

    // Save snapshot to file
    match cli.output.as_str() {
        output if output.ends_with(".png") => {
            image_snapshot.png_data()?.save(&cli.output)?;
        }
        output if output.ends_with(".svg") => {
            image_snapshot.svg_data()?.save(&cli.output)?;
        }
        output if output.ends_with(".html") => {
            image_snapshot.html_data()?.save(&cli.output)?;
        }
        _ => {
            bail!("Unsupported output format");
        }
    };

    Ok(format!("Snapshot saved to {} successful!", cli.output))
}

async fn generate_snapshot_with_config(cli: &CLI, codesnap: CodeSnap) -> anyhow::Result<()> {
    let snapshot = create_snapshot_config(&cli, codesnap).await?;
    let snapshot_type = cli.r#type.clone();

    if snapshot_type == "ascii" && cli.output != "clipboard" {
        logger::warn("ASCII snapshot only supports copying to clipboard");
        return Ok(());
    }

    let message = with_spinner(|| output_snapshot(&cli, &snapshot))?;

    logger::success(&message);

    Ok(())
}

// If the code theme is URL, download it and return the path
async fn parse_code_theme(path: &str, code_theme: &str) -> anyhow::Result<String> {
    let assets_url = AssetsURL::from_url(code_theme);

    match assets_url {
        Ok(assets_url) => {
            let assets = Assets::from(path);
            let assets_store_path_str = assets.download(code_theme).await?;
            let assets_store_path = Path::new(&assets_store_path_str);
            let extension = assets_store_path.extension().unwrap_or_default();

            // If the code theme is JSON file, we treat it as a VSCode theme file
            if extension == "json" {
                let root = vscode::VSCodeThemeParser::from_config(&assets_store_path_str)
                    .unwrap()
                    .parse(&assets_url.name);
                let path = Path::new(&assets_store_path)
                    .with_file_name(format!("{}.{}", &assets_url.name, "tmTheme"));

                plist::to_writer_xml(&mut fs::File::create(path).unwrap(), &root)?;
            }

            Ok(assets_url.name)
        }
        Err(_) => {
            // If the code theme is not a URL, we will use it as a local file
            Ok(code_theme.to_string())
        }
    }
}

async fn create_snapshot_config(
    cli: &CLI,
    mut codesnap: CodeSnap,
) -> anyhow::Result<SnapshotConfig> {
    // Build screenshot config
    let mut codesnap = codesnap
        .map_code_config(|code_config| create_code_config(&cli, code_config))?
        .map_code(|raw_code| create_code(&cli, raw_code))?
        .map_watermark(|watermark| create_watermark(&cli, watermark))?
        .map_window(|window| create_window(&cli, window))?
        .scale_factor(cli.scale_factor)
        .build()?;

    let remote_themes_path = home::home_dir()
        .context("Unable to get your home dir")?
        .join(".config")
        .join("CodeSnap")
        .join("remote_themes");
    std::fs::create_dir_all(&remote_themes_path)?;

    let remote_themes_path = remote_themes_path
        .to_str()
        .context("Invalid remote theme path")?;

    let mut themes_folders = codesnap.themes_folders;
    // The remote themes folder is used to store the themes downloaded from the internet
    themes_folders.push(remote_themes_path.to_owned());

    codesnap.themes_folders = themes_folders;
    codesnap.fonts_folders = codesnap.fonts_folders;
    codesnap.line_number_color = cli.line_number_color.clone();
    codesnap.title = cli.title.clone();
    codesnap.theme = parse_code_theme(
        &remote_themes_path,
        cli.code_theme
            .clone()
            .unwrap_or(codesnap.theme.clone())
            .as_ref(),
    )
    .await?;

    Ok(codesnap)
}

fn with_spinner<T>(cb: impl Fn() -> T) -> T {
    let pb = ProgressBar::new_spinner();

    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&[
                "⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣯", "⣷", "⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠠",
                "⠐", "⠈",
            ]),
    );
    pb.set_message("Generating...");

    let result = cb();

    pb.finish_and_clear();
    result
}

async fn generate_snapshot() -> anyhow::Result<()> {
    let cli = CLI::parse();

    // Create CodeSnap config from config, if the user does not have a config file, we will create
    // a default CodeSnap config to $HOME/.codesnap/config.json for the user.
    let codesnap_cli_config = if let Some(ref config) = cli.config {
        let content = read_to_string(config)?;

        CodeSnapCLIConfig::from(&content)?
    } else {
        CodeSnapCLIConfig::from(&config::get_config_content()?)?
    };

    generate_snapshot_with_config(&cli, codesnap_cli_config.snapshot_config).await?;

    if codesnap_cli_config.print_eggs {
        say();
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = generate_snapshot().await {
        logger::error(&err.to_string());
        return;
    };
}
