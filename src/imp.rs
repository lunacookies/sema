use crate::palette::{BaseScale, Palette};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rule("editor.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rules(
        &["editor.foreground", "foreground"],
        palette.base(BaseScale::Fg),
    );

    builder.add_workspace_rule(
        "rust_analyzer.inlayHints.foreground",
        palette.base(BaseScale::DarkFg),
    );
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rule(Semantic("keyword"), palette.light_blue());
    builder.add_rule(Semantic("*.controlFlow"), palette.blue());

    builder.add_rules(
        &[
            Semantic("function.declaration"),
            Semantic("method.declaration"),
        ],
        palette.yellow(),
    );

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("enum"),
            Semantic("union"),
            Semantic("typeAlias"),
            Semantic("builtinType"),
            Semantic("interface"),
            Semantic("typeParameter"),
        ],
        palette.yellow(),
    );

    builder.add_rules(
        &[Semantic("*.constant"), Semantic("variable.static")],
        palette.yellow(),
    );

    builder.add_rule(Semantic("number"), palette.magenta());
    builder.add_rules(
        &[Semantic("string"), Semantic("characterLiteral")],
        palette.green(),
    );

    builder.add_rule(Semantic("property"), palette.lavender());

    builder.add_rule(Semantic("enumMember"), palette.lavender());

    builder.add_rule(Semantic("parameter"), palette.lavender());

    builder.add_rules(
        &[Semantic("formatSpecifier"), Semantic("escapeSequence")],
        palette.lavender(),
    );

    builder.add_rule(Semantic("lifetime"), palette.blue());

    builder.add_rule(
        Semantic("comment"),
        (palette.base(BaseScale::BrightFg), FontStyle::Italic),
    );

    builder.add_rule(
        Semantic("comment.documentation"),
        palette.base(BaseScale::BrightFg),
    );

    builder.add_rule(Semantic("*.mutable"), FontStyle::Italic);
}
