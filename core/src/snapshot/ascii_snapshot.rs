use std::cmp::max;

use crate::{
    config::{Code, RawCode, SnapshotConfig},
    utils::code::{calc_max_line_number_length, calc_wh, prepare_code},
};

use super::snapshot_data::SnapshotData;

const SPACE_BOTH_SIDE: usize = 2;

pub struct ASCIISnapshot {
    config: SnapshotConfig,
    code: RawCode,
}

fn optional(component: String, is_view: bool) -> String {
    if is_view {
        component
    } else {
        "".to_string()
    }
}

impl ASCIISnapshot {
    pub fn raw_data(&self) -> Result<SnapshotData, anyhow::Error> {
        Ok(SnapshotData::Text(self.generate_snapshot()))
    }
}

impl ASCIISnapshot {
    pub fn from_config(config: SnapshotConfig) -> anyhow::Result<Self> {
        match config.code {
            Code::Raw(ref raw_code) => Ok(ASCIISnapshot {
                config: config.clone(),
                code: raw_code.clone(),
            }),
            _ => Err(anyhow::anyhow!("The code content is not raw")),
        }
    }

    fn generate_snapshot(&self) -> String {
        let code = prepare_code(&self.code.content);
        let (width, height) = calc_wh(&code, 1., 1.);
        let calc_line_number_width = |start_line_number: u32| {
            calc_max_line_number_length(height as usize, start_line_number)
        };
        let len = self
            .code
            .clone()
            .file_path
            .and_then(|x| Some(x.len()))
            .unwrap_or(0);
        let frame_width = max(width as usize, len + SPACE_BOTH_SIDE);
        let frame_width = match self.code.line_number {
            Some(ref line_number) => {
                frame_width + SPACE_BOTH_SIDE + calc_line_number_width(line_number.start_number)
            }
            None => frame_width,
        };
        let line = format!("│{}│\n", "─".repeat(frame_width));
        let frame_width_with_content = frame_width - 1;
        let top_frame = format!("╭{}╮\n", "─".repeat(frame_width));
        let bottom_frame = format!("╰{}╯", "─".repeat(frame_width));
        let code = code
            .lines()
            .enumerate()
            .map(|(i, line)| {
                format!(
                    "│ {:1$} │\n",
                    match self.code.line_number {
                        Some(ref line_number) => format!(
                            "{:1$} {line}",
                            line_number.start_number as usize + i,
                            calc_line_number_width(line_number.start_number),
                        ),
                        None => line.to_string(),
                    },
                    frame_width_with_content - 1
                )
            })
            .collect::<String>();
        let text_line = |text: &str| format!("│ {:1$}│\n", text, frame_width_with_content);
        let breadcrumbs = optional(
            format!(
                "{}{line}",
                text_line(&self.code.file_path.clone().unwrap_or(String::from("")))
            ),
            self.code.breadcrumbs.is_some(),
        );

        format!("{top_frame}{breadcrumbs}{code}{bottom_frame}")
    }
}
