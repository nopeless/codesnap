use cosmic_text::{Attrs, Family, Weight};

use crate::{
    components::interface::{
        component::{Component, ComponentContext, RenderParams},
        render_error,
        style::{ComponentStyle, RawComponentStyle, Size, Style},
    },
    utils::{
        code::calc_wh_with_min_width,
        color::parse_hex_to_cosmic_color,
        text::{create_file_system_from_binary, FontRenderer},
    },
};

pub struct CommandLineHeader {
    children: Vec<Box<dyn Component>>,
    full_command: String,
}

const CASKAYDIA_COVE_NERD_FONT: &[u8] =
    include_bytes!("../../../assets/fonts/CaskaydiaCoveNerdFont-Regular.ttf");
const FONT_SIZE: f32 = 12.5;

impl Component for CommandLineHeader {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self, context: &ComponentContext) -> RawComponentStyle {
        let parsed_line = format!(
            "{} {}",
            context.take_snapshot_params.command_output_config.prompt, self.full_command
        );
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
        // let foreground_color = context.theme_provider.theme.settings.foreground.unwrap();
        let attr = Attrs::new().family(Family::Name("Caskaydia Cove Nerd Font"));
        let command_config = context.take_snapshot_params.command_output_config.clone();
        let spans = vec![
            (
                command_config.prompt.as_str(),
                attr.color(parse_hex_to_cosmic_color(&command_config.prompt_color)),
            ),
            (
                self.full_command.as_str(),
                attr.color(parse_hex_to_cosmic_color(&command_config.command_color))
                    .weight(Weight::BOLD),
            ),
            // (
            //     self.args.as_str(),
            //     attr.color(Color::rgba(
            //         foreground_color.r,
            //         foreground_color.g,
            //         foreground_color.b,
            //         foreground_color.a,
            //     )),
            // ),
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
    pub fn from(full_command: &str) -> CommandLineHeader {
        CommandLineHeader {
            full_command: full_command.to_string(),
            children: vec![],
        }
    }
}
