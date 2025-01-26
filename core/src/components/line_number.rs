use super::interface::{
    component::{Component, ComponentContext, RenderParams},
    render_error,
    style::{ComponentStyle, RawComponentStyle, Size, Style},
};
use crate::{
    config::RawCode,
    edges::margin::Margin,
    utils::{
        code::CHAR_WIDTH,
        text::{create_file_system_by_fonts_folder, FontRenderer},
    },
};
use cosmic_text::{Attrs, Color, Family};

const FONT_SIZE: f32 = 14.;

#[derive(Default)]
pub struct LineNumber {
    children: Vec<Box<dyn Component>>,
    line_height: f32,
    render_condition: bool,
    line_number_content: Vec<String>,
    number_of_digit: usize,
    raw_code: RawCode,
}

impl Component for LineNumber {
    fn render_condition(&self) -> bool {
        return self.render_condition;
    }

    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self) -> RawComponentStyle {
        Style::default()
            .size(
                Size::Num(CHAR_WIDTH * self.number_of_digit as f32),
                Size::Num(self.line_number_content.len() as f32 * self.line_height),
            )
            .margin(Margin {
                right: 10.,
                ..Margin::default()
            })
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &ComponentContext,
        render_params: &RenderParams,
        style: &ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        FontRenderer::new(
            FONT_SIZE,
            self.line_height,
            context.scale_factor,
            create_file_system_by_fonts_folder(&context.take_snapshot_params.fonts_folder),
        )
        .draw_text(
            render_params.x,
            render_params.y,
            style.width,
            style.height,
            vec![(
                &self.line_number_content.join("\n"),
                Attrs::new()
                    .color(Color::rgb(73, 81, 98))
                    .family(Family::Name(&self.raw_code.font_family)),
            )],
            pixmap,
        );

        Ok(())
    }
}

impl LineNumber {
    pub fn new(raw_code: RawCode, line_height: f32) -> LineNumber {
        match raw_code.line_number {
            None => LineNumber::default(),
            Some(ref line_number) => {
                let lines = raw_code.content.split("\n").collect::<Vec<&str>>();
                let start_line_number = line_number.start_number;
                let max_line_number = lines.len() as u32 + start_line_number;
                let number_of_digit = (max_line_number - 1).to_string().len();

                LineNumber {
                    line_number_content: (start_line_number..max_line_number)
                        .map(|line_number| {
                            format!(
                                "{:>width$}",
                                line_number.to_string(),
                                width = number_of_digit
                            )
                        })
                        .collect::<Vec<String>>(),
                    number_of_digit,
                    children: vec![],
                    render_condition: true,
                    line_height,
                    raw_code,
                }
            }
        }
    }
}
