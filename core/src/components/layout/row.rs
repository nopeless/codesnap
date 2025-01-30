use crate::components::interface::{
    component::{Component, ComponentContext},
    style::{ComponentAlign, RawComponentStyle, Style},
};

pub struct Row {
    children: Vec<Box<dyn Component>>,
}

impl Component for Row {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self, _context: &ComponentContext) -> RawComponentStyle {
        Style::default().align(ComponentAlign::Row)
    }
}

impl Row {
    pub fn from_children(children: Vec<Box<dyn Component>>) -> Row {
        Row { children }
    }
}
