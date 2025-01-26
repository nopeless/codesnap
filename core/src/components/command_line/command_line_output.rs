use crate::{
    ansi::ANSI,
    components::interface::{
        component::{self, Component},
        render_error,
        style::{self, RawComponentStyle, Size, Style},
    },
    utils::{
        code::calc_wh_with_min_width,
        text::{create_file_system_from_binary, FontRenderer},
    },
};

const CASKAYDIA_COVE_NERD_FONT: &[u8] =
    include_bytes!("../../../assets/fonts/CaskaydiaCoveNerdFont-Regular.ttf");
const FONT_SIZE: f32 = 12.5;

pub struct CommandLineOutput {
    ansi_text: String,
    children: Vec<Box<dyn Component>>,
}

impl Component for CommandLineOutput {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self) -> RawComponentStyle {
        let (w, h) = calc_wh_with_min_width(&self.ansi_text, FONT_SIZE / 2., 20.);

        Style::default().size(Size::Num(w), Size::Num(h))
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &component::ComponentContext,
        render_params: &component::RenderParams,
        style: &style::ComponentStyle,
        _parent_style: &style::ComponentStyle,
    ) -> render_error::Result<()> {
        let ansi = ANSI::from(&self.ansi_text);
        let spans = ansi.colorize();

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
            spans.clone(),
            pixmap,
        );

        Ok(())
    }
}

impl CommandLineOutput {
    pub fn from(ansi_text: &str) -> CommandLineOutput {
        CommandLineOutput {
            ansi_text: ansi_text.to_string(),
            children: vec![],
        }
    }
}
