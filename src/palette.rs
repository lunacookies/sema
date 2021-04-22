mod base;
mod color;

pub(crate) use base::BaseScale;

use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

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
