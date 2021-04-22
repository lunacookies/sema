use super::{oklch, Palette};
use tincture::Oklch;

macro_rules! define_color_method {
    ($name:ident,$ty:expr,$hue:literal) => {
        impl Palette {
            pub(crate) fn $name(&self) -> Oklch {
                oklch($ty.lightness(), $ty.chroma(), $hue)
            }
        }
    };
}

define_color_method!(green, ColorType::Strong, 130.0);
define_color_method!(blue, ColorType::Regular, 230.0);
define_color_method!(light_blue, ColorType::Light, 240.0);
define_color_method!(lavender, ColorType::Light, 285.0);
define_color_method!(magenta, ColorType::Regular, 330.0);

enum ColorType {
    Regular,
    Light,
    Strong,
}

impl ColorType {
    fn lightness(self) -> f32 {
        match self {
            Self::Regular => 0.8,
            Self::Light => 0.9,
            Self::Strong => 0.9,
        }
    }

    fn chroma(self) -> f32 {
        match self {
            Self::Regular => 0.1,
            Self::Light => 0.04,
            Self::Strong => 0.1,
        }
    }
}
