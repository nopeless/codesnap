use once_cell::sync::Lazy;
use tiny_skia::{Color, GradientStop};

use crate::config::{Background, DimensionValue, GradientPoint, LinearGradient};

// This file contains the preset background themes that can be used in CodeSnap
// If you want to add more beautiful background themes, please feel free to contribute

pub static BAMBOO: Lazy<Background> = Lazy::new(|| {
    Background::Gradient(LinearGradient {
        start: GradientPoint {
            x: DimensionValue::Num(0.),
            y: DimensionValue::Num(0.),
        },
        end: GradientPoint {
            x: DimensionValue::Max,
            y: DimensionValue::Num(0.),
        },
        stops: vec![
            GradientStop::new(0.22, Color::from_rgba8(107, 203, 165, 255)),
            GradientStop::new(0.95, Color::from_rgba8(202, 244, 194, 255)),
        ],
    })
});

pub static SEA: Lazy<Background> = Lazy::new(|| {
    Background::Gradient(LinearGradient {
        start: GradientPoint {
            x: DimensionValue::Num(0.),
            y: DimensionValue::Num(0.),
        },
        end: GradientPoint {
            x: DimensionValue::Max,
            y: DimensionValue::Num(0.),
        },
        stops: vec![
            GradientStop::new(0.0, Color::from_rgba8(31, 162, 255, 255)),
            GradientStop::new(0.4, Color::from_rgba8(18, 216, 250, 255)),
            GradientStop::new(0.95, Color::from_rgba8(166, 255, 203, 255)),
        ],
    })
});

// The CLASSIC background was default background of CodeSnap.nvim ^^
pub static CLASSIC: Lazy<Background> = Lazy::new(|| {
    Background::Gradient(LinearGradient {
        start: GradientPoint {
            x: DimensionValue::Num(0.),
            y: DimensionValue::Num(0.),
        },
        end: GradientPoint {
            x: DimensionValue::Max,
            y: DimensionValue::Num(0.),
        },
        stops: vec![
            GradientStop::new(0.0, Color::from_rgba8(58, 28, 113, 255)),
            GradientStop::new(0.5, Color::from_rgba8(215, 109, 119, 255)),
            GradientStop::new(0.95, Color::from_rgba8(255, 175, 123, 255)),
        ],
    })
});

pub static GRAPE: Lazy<Background> = Lazy::new(|| {
    Background::Gradient(LinearGradient {
        start: GradientPoint {
            x: DimensionValue::Num(0.),
            y: DimensionValue::Num(0.),
        },
        end: GradientPoint {
            x: DimensionValue::Max,
            y: DimensionValue::Num(0.),
        },
        stops: vec![
            GradientStop::new(0.28, Color::from_rgba8(103, 90, 247, 255)),
            GradientStop::new(0.95, Color::from_rgba8(189, 101, 250, 255)),
        ],
    })
});

pub static PEACH: Lazy<Background> = Lazy::new(|| {
    Background::Gradient(LinearGradient {
        start: GradientPoint {
            x: DimensionValue::Num(0.),
            y: DimensionValue::Num(0.),
        },
        end: GradientPoint {
            x: DimensionValue::Max,
            y: DimensionValue::Num(0.),
        },
        stops: vec![
            GradientStop::new(0.22, Color::from_rgba8(221, 94, 137, 255)),
            GradientStop::new(0.95, Color::from_rgba8(247, 187, 151, 255)),
        ],
    })
});

pub static SUMMER: Lazy<Background> = Lazy::new(|| {
    Background::Gradient(LinearGradient {
        start: GradientPoint {
            x: DimensionValue::Num(0.),
            y: DimensionValue::Num(0.),
        },
        end: GradientPoint {
            x: DimensionValue::Max,
            y: DimensionValue::Num(0.),
        },
        stops: vec![
            GradientStop::new(0.28, Color::from_rgba8(248, 165, 194, 255)),
            GradientStop::new(0.95, Color::from_rgba8(116, 185, 255, 255)),
        ],
    })
});

pub static DUSK: Lazy<Background> = Lazy::new(|| {
    Background::Gradient(LinearGradient {
        start: GradientPoint {
            x: DimensionValue::Num(0.),
            y: DimensionValue::Num(0.),
        },
        end: GradientPoint {
            x: DimensionValue::Max,
            y: DimensionValue::Num(0.),
        },
        stops: vec![
            GradientStop::new(0.22, Color::from_rgba8(255, 98, 110, 255)),
            GradientStop::new(0.95, Color::from_rgba8(255, 190, 113, 255)),
        ],
    })
});
