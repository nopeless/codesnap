use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TakeSnapshotParams {
    // Whether to display the MacOS style title bar
    pub mac_window_bar: bool,
    // Wartermark of the code snapshot
    pub watermark: String,
    // Editor title
    pub title: Option<String>,
    pub code_font_family: String,
    pub watermark_font_family: String,
    pub code: String,
    pub code_file_path: String,
    pub extension: Option<String>,
    pub save_path: Option<String>,
    pub themes_folder: String,
    pub fonts_folder: String,
    pub theme: String,
    pub bg_theme: String,
    pub bg_color: Option<String>,
    // Breadcrumbs path
    pub file_path: String,
    pub breadcrumbs_separator: String,
    pub has_breadcrumbs: bool,
    pub start_line_number: Option<usize>,
    pub highlight_start_line_number: Option<usize>,
    pub highlight_end_line_number: Option<usize>,
    pub min_width: Option<f32>,
    pub bg_x_padding: f32,
    pub bg_y_padding: f32,
    pub bg_padding: Option<f32>,
}
