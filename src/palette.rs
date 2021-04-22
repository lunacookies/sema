use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), 0.0, 0.0)
    }
}

macro_rules! define_color_method {
    ($name:ident,$hue:literal) => {
        impl Palette {
            pub(crate) fn $name(&self) -> Oklch {
                oklch(0.8, 0.1, $hue)
            }
        }
    };
}

macro_rules! define_light_color_method {
    ($name:ident,$hue:literal) => {
        impl Palette {
            pub(crate) fn $name(&self) -> Oklch {
                oklch(0.9, 0.04, $hue)
            }
        }
    };
}

macro_rules! define_strong_color_method {
    ($name:ident,$hue:literal) => {
        impl Palette {
            pub(crate) fn $name(&self) -> Oklch {
                oklch(0.9, 0.1, $hue)
            }
        }
    };
}

define_strong_color_method!(green, 130.0);
define_color_method!(blue, 230.0);
define_light_color_method!(light_blue, 240.0);
define_light_color_method!(lavender, 285.0);
define_color_method!(magenta, 330.0);

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
