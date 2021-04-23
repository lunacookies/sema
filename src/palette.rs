use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), 0.0, 0.0)
    }

    const LOW_LIGHTNESS: f32 = 0.8;
    const HIGH_LIGHTNESS: f32 = 0.9;
    const LOW_CHROMA: f32 = 0.03;
    const HIGH_CHROMA: f32 = 0.1;

    pub(crate) fn pink(&self) -> Oklch {
        oklch(Self::HIGH_LIGHTNESS, Self::LOW_CHROMA, 0.0)
    }

    pub(crate) fn red(&self) -> Oklch {
        oklch(Self::LOW_LIGHTNESS, Self::HIGH_CHROMA, 30.0)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(Self::HIGH_LIGHTNESS, Self::LOW_CHROMA, 105.0)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(Self::HIGH_LIGHTNESS, Self::HIGH_CHROMA, 130.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(Self::LOW_LIGHTNESS, Self::HIGH_CHROMA, 230.0)
    }

    pub(crate) fn light_blue(&self) -> Oklch {
        oklch(Self::HIGH_LIGHTNESS, Self::LOW_CHROMA, 240.0)
    }

    pub(crate) fn lavender(&self) -> Oklch {
        oklch(Self::HIGH_LIGHTNESS, Self::LOW_CHROMA, 285.0)
    }

    pub(crate) fn magenta(&self) -> Oklch {
        oklch(Self::LOW_LIGHTNESS, Self::HIGH_CHROMA, 330.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    Bg,
    DarkFg,
    DimFg,
    Fg,
    BrightFg,
}

impl BaseScale {
    fn value(self) -> f32 {
        match self {
            Self::Bg => 0.0,
            Self::DarkFg => 0.5,
            Self::DimFg => 0.7,
            Self::Fg => 0.85,
            Self::BrightFg => 1.0,
        }
    }

    fn lightness(self) -> f32 {
        lerp(self.value(), 0.17..1.0)
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
