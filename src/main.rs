mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    let palette = palette::Palette;

    let mut builder = ThemeBuilder::new("Sema".to_string(), Type::Dark);
    imp::add_rules(&mut builder, &palette);
    builder.build().save()?;

    Ok(())
}
