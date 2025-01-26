use cosmic_text::{Attrs, Color, Family, Weight};

use crate::{
    components::interface::{
        component::{Component, ComponentContext, RenderParams},
        render_error,
        style::{ComponentStyle, RawComponentStyle, Size, Style},
    },
    config::CommandLine,
    utils::{
        code::calc_wh_with_min_width,
        color::parse_hex_to_cosmic_color,
        text::{create_file_system_from_binary, FontRenderer},
    },
};

pub struct CommandLineHeader {
    children: Vec<Box<dyn Component>>,
    prompt: String,
    prompt_color: Color,
    command_color: Color,
    command: String,
    args: String,
}

const CASKAYDIA_COVE_NERD_FONT: &[u8] =
    include_bytes!("../../../assets/fonts/CaskaydiaCoveNerdFont-Regular.ttf");
const FONT_SIZE: f32 = 12.5;

impl Component for CommandLineHeader {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self) -> RawComponentStyle {
        let parsed_line = format!("{}{}", self.prompt, self.command);
        let (w, h) = calc_wh_with_min_width(parsed_line.as_str(), FONT_SIZE / 2., 20.);

        Style::default().size(Size::Num(w), Size::Num(h))
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &ComponentContext,
        render_params: &RenderParams,
        style: &ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        let foreground_color = context.theme_provider.theme.settings.foreground.unwrap();
        let attr = Attrs::new().family(Family::Name("Caskaydia Cove Nerd Font"));
        let spans = vec![
            (self.prompt.as_str(), attr.color(self.prompt_color)),
            (
                self.command.as_str(),
                attr.color(self.command_color).weight(Weight::BOLD),
            ),
            (
                self.args.as_str(),
                attr.color(Color::rgba(
                    foreground_color.r,
                    foreground_color.g,
                    foreground_color.b,
                    foreground_color.a,
                )),
            ),
        ];

        FontRenderer::new(
            FONT_SIZE,
            20.,
            context.scale_factor,
            create_file_system_from_binary(
                CASKAYDIA_COVE_NERD_FONT,
                &context.take_snapshot_params.fonts_folder,
            ),
        )
        .draw_text(
            render_params.x,
            render_params.y,
            style.width,
            style.height,
            spans,
            pixmap,
        );

        Ok(())
    }
}

impl CommandLineHeader {
    pub fn from(command_line: &CommandLine, command: &str) -> CommandLineHeader {
        let command = command.split_whitespace().collect::<Vec<&str>>();

        CommandLineHeader {
            prompt: format!("{} ", command_line.prompt),
            command: format!("{} ", command[0]),
            prompt_color: parse_hex_to_cosmic_color(&command_line.prompt_color),
            command_color: parse_hex_to_cosmic_color(&command_line.command_color),
            children: vec![],
            args: command[1..].join(" "),
        }
    }
}
