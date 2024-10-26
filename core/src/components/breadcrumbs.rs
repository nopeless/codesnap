use cosmic_text::{Attrs, Color, Family};
use regex::Regex;

use crate::{
    config, edges::margin::Margin, utils::code::calc_wh_with_min_width, utils::text::FontRenderer,
};

use super::interface::{
    component::Component,
    style::{ComponentStyle, RawComponentStyle, Size},
};

const LINE_HEIGHT: f32 = 15.;

pub struct Breadcrumbs {
    children: Vec<Box<dyn Component>>,
    path: Option<String>,
    config: Option<config::Breadcrumbs>,
}

impl Component for Breadcrumbs {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self) -> RawComponentStyle {
        let style = RawComponentStyle::default();

        if self.config.is_some() {
            if let Some(path) = &self.path {
                let (w, h) = calc_wh_with_min_width(&path, 8., LINE_HEIGHT);

                return style
                    .size(Size::Num(w), Size::Num(h))
                    .margin(Margin::default());
            }
        }

        style
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &super::interface::component::ComponentContext,
        render_params: &super::interface::component::RenderParams,
        style: &super::interface::style::ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> super::interface::render_error::Result<()> {
        if self.config.is_some() {
            if let Some(ref path) = self.path {
                let attrs = Attrs::new()
                    .color(Color::rgb(128, 132, 139))
                    .family(Family::Name(&context.take_snapshot_params.code.font_family));

                FontRenderer::new(
                    12.,
                    LINE_HEIGHT,
                    context.scale_factor,
                    context.take_snapshot_params.fonts_folder.clone(),
                )
                .draw_text(
                    render_params.x,
                    render_params.y,
                    style.width,
                    LINE_HEIGHT,
                    vec![(path, attrs)],
                    pixmap,
                );
            }
        }

        Ok(())
    }
}

impl Breadcrumbs {
    pub fn from_path(
        file_path: Option<String>,
        config: Option<config::Breadcrumbs>,
    ) -> Breadcrumbs {
        Breadcrumbs {
            children: vec![],
            path: file_path,
            config,
        }
    }

    pub fn parse_separator(&self, path: &str, separator: &str) -> String {
        Regex::new("/")
            .unwrap()
            .replace_all(path, separator)
            .to_string()
    }
}
