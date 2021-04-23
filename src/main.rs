mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    build_theme(&palette::Palette::default(), Type::Dark, "sema")?;
    build_theme(&palette::Palette::chroma(), Type::Dark, "sema chroma")?;
    build_theme(&palette::Palette::soft(), Type::Dark, "sema soft")?;
    build_theme(
        &palette::Palette::soft_chroma(),
        Type::Dark,
        "sema soft chroma",
    )?;

    build_theme(&palette::Palette::light(), Type::Light, "sema light")?;
    build_theme(
        &palette::Palette::light_chroma(),
        Type::Light,
        "sema light chroma",
    )?;
    build_theme(
        &palette::Palette::light_soft(),
        Type::Light,
        "sema light soft",
    )?;
    build_theme(
        &palette::Palette::light_soft_chroma(),
        Type::Light,
        "sema light soft chroma",
    )?;

    Ok(())
}

fn build_theme(palette: &palette::Palette, ty: Type, name: &str) -> io::Result<()> {
    let mut builder = ThemeBuilder::new(name.to_string(), ty);
    imp::add_rules(&mut builder, palette);
    builder.build().save()?;

    Ok(())
}
