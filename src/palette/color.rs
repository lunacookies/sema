use super::{oklch, Palette};
use tincture::Oklch;

const LOW_LIGHTNESS: f32 = 0.8;
const HIGH_LIGHTNESS: f32 = 0.9;

const LOW_CHROMA: f32 = 0.04;
const HIGH_CHROMA: f32 = 0.1;

impl Palette {
    pub(crate) fn green(&self) -> Oklch {
        oklch(HIGH_LIGHTNESS, HIGH_CHROMA, 130.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(LOW_LIGHTNESS, HIGH_CHROMA, 230.0)
    }

    pub(crate) fn light_blue(&self) -> Oklch {
        oklch(HIGH_LIGHTNESS, LOW_CHROMA, 240.0)
    }

    pub(crate) fn lavender(&self) -> Oklch {
        oklch(HIGH_LIGHTNESS, LOW_CHROMA, 285.0)
    }

    pub(crate) fn magenta(&self) -> Oklch {
        oklch(LOW_LIGHTNESS, HIGH_CHROMA, 330.0)
    }
}
