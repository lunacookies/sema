use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), 0.0, 0.0)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.95, 0.1, 130.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.8, 0.1, 230.0)
    }

    pub(crate) fn light_blue(&self) -> Oklch {
        oklch(0.9, 0.05, 240.0)
    }

    pub(crate) fn lavender(&self) -> Oklch {
        oklch(0.93, 0.032, 285.0)
    }

    pub(crate) fn magenta(&self) -> Oklch {
        oklch(0.85, 0.13, 330.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    Bg,
    Fg,
    BrightFg,
}

impl BaseScale {
    fn value(self) -> f32 {
        match self {
            Self::Bg => 0.0,
            Self::Fg => 0.8,
            Self::BrightFg => 1.0,
        }
    }

    fn lightness(self) -> f32 {
        lerp(self.value(), 0.2..1.0)
    }
}

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h).unwrap(),
    }
}

fn lerp(x: f32, range: Range<f32>) -> f32 {
    x * (range.end - range.start) + range.start
}
