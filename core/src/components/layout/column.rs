use crate::components::interface::{
    component::{Component, ComponentContext},
    style::{ComponentAlign, RawComponentStyle, Style},
};

pub struct Column {
    children: Vec<Box<dyn Component>>,
}

impl Component for Column {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self, _context: &ComponentContext) -> RawComponentStyle {
        Style::default().align(ComponentAlign::Column)
    }
}

impl Column {
    pub fn from_children(children: Vec<Box<dyn Component>>) -> Column {
        Column { children }
    }
}
