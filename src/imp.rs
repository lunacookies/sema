use crate::palette::{BaseScale, Palette};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rules(
        &[
            "editor.background",
            "editorInlayHint.background",
            "editorInlayHint.parameterBackground",
            "editorInlayHint.typeBackground",
        ],
        palette.base(BaseScale::Bg),
    );
    builder.add_workspace_rules(
        &["editor.foreground", "foreground"],
        palette.base(BaseScale::Fg),
    );

    builder.add_workspace_rule(
        "editor.lineHighlightBackground",
        palette.base(BaseScale::LightBg),
    );

    builder.add_workspace_rule(
        "editor.selectionBackground",
        palette.base(BaseScale::LighterBg),
    );

    builder.add_workspace_rules(
        &["editorCursor.foreground", "terminalCursor.foreground"],
        palette.base(BaseScale::BrightFg),
    );
    builder.add_workspace_rules(
        &["editorCursor.background", "terminalCursor.background"],
        palette.base(BaseScale::Bg),
    );

    builder.add_workspace_rules(
        &["activityBar.background", "sideBar.background"],
        palette.base(BaseScale::Bg),
    );
    builder.add_workspace_rule("activityBar.foreground", palette.base(BaseScale::BrightFg));
    builder.add_workspace_rule(
        "activityBar.inactiveForeground",
        palette.base(BaseScale::DarkFg),
    );

    builder.add_workspace_rule("statusBar.foreground", palette.base(BaseScale::DarkFg));
    builder.add_workspace_rules(
        &[
            "statusBar.background",
            "statusBar.debuggingBackground",
            "statusBar.noFolderBackground",
        ],
        palette.base(BaseScale::Bg),
    );

    builder.add_workspace_rule(
        "editorLineNumber.foreground",
        palette.base(BaseScale::DarkFg),
    );
    builder.add_workspace_rule(
        "editorLineNumber.activeForeground",
        palette.base(BaseScale::BrightFg),
    );

    builder.add_workspace_rule("editorWidget.background", palette.base(BaseScale::LightBg));

    builder.add_workspace_rule(
        "list.highlightForeground",
        palette.base(BaseScale::BrightFg),
    );
    builder.add_workspace_rules(
        &["list.focusBackground", "list.activeSelectionBackground"],
        palette.base(BaseScale::LighterBg),
    );

    builder.add_workspace_rules(
        &["editorGroupHeader.tabsBackground", "tab.inactiveBackground"],
        palette.base(BaseScale::Bg),
    );
    builder.add_workspace_rule("tab.activeBackground", palette.base(BaseScale::LightBg));
    builder.add_workspace_rule("tab.inactiveForeground", palette.base(BaseScale::DimFg));
    builder.add_workspace_rule("tab.activeForeground", palette.base(BaseScale::BrightFg));

    builder.add_workspace_rules(
        &["titleBar.activeBackground", "titleBar.inactiveBackground"],
        palette.base(BaseScale::Bg),
    );
    builder.add_workspace_rule("titleBar.activeForeground", palette.base(BaseScale::Fg));
    builder.add_workspace_rule(
        "titleBar.inactiveForeground",
        palette.base(BaseScale::DarkFg),
    );

    builder.add_workspace_rule("breadcrumb.foreground", palette.base(BaseScale::DimFg));
    builder.add_workspace_rule(
        "breadcrumb.focusForeground",
        palette.base(BaseScale::BrightFg),
    );

    builder.add_workspace_rule("terminal.foreground", palette.base(BaseScale::Fg));
    builder.add_workspace_rule("terminal.ansiBlack", palette.base(BaseScale::LightBg));
    builder.add_workspace_rule("terminal.ansiBrightBlack", palette.base(BaseScale::DimFg));
    builder.add_workspace_rule("terminal.ansiRed", palette.red());
    builder.add_workspace_rule("terminal.ansiBrightRed", palette.red());
    builder.add_workspace_rule("terminal.ansiGreen", palette.green());
    builder.add_workspace_rule("terminal.ansiBrightGreen", palette.light_green());
    builder.add_workspace_rule("terminal.ansiYellow", palette.yellow());
    builder.add_workspace_rule("terminal.ansiBrightYellow", palette.yellow());
    builder.add_workspace_rule("terminal.ansiBlue", palette.blue());
    builder.add_workspace_rule("terminal.ansiBrightBlue", palette.light_blue());
    builder.add_workspace_rule("terminal.ansiMagenta", palette.pink());
    builder.add_workspace_rule("terminal.ansiBrightMagenta", palette.pink());
    builder.add_workspace_rule("terminal.ansiCyan", palette.lavender());
    builder.add_workspace_rule("terminal.ansiBrightCyan", palette.lavender());
    builder.add_workspace_rule("terminal.ansiWhite", palette.base(BaseScale::Fg));
    builder.add_workspace_rule(
        "terminal.ansiBrightWhite",
        palette.base(BaseScale::BrightFg),
    );

    builder.add_workspace_rule("focusBorder", palette.base(BaseScale::DarkFg));

    builder.add_workspace_rules(
        &[
            "rust_analyzer.inlayHints.foreground",
            "editorInlayHint.foreground",
            "editorInlayHint.parameterForeground",
            "editorInlayHint.typeForeground",
        ],
        palette.base(BaseScale::DimFg),
    );
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rules(
        &[
            Semantic("keyword"),
            Textmate("keyword"),
            Textmate("storage"),
            Textmate("variable.language.self"),
            Textmate("variable.language.this"),
            Textmate("keyword.type.go"),
        ],
        palette.light_blue(),
    );
    builder.add_rules(
        &[Semantic("*.controlFlow"), Textmate("keyword.control")],
        palette.blue(),
    );

    builder.add_rules(
        &[Semantic("function.trait"), Semantic("method.trait")],
        palette.blue(),
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
            Textmate("entity.name.type"),
            Textmate("keyword.type"),
            Textmate("storage.type.cs"),
            Textmate("storage.type.java"),
            Textmate("storage.type.boolean.go"),
            Textmate("storage.type.byte.go"),
            Textmate("storage.type.error.go"),
            Textmate("storage.type.numeric.go"),
            Textmate("storage.type.rune.go"),
            Textmate("storage.type.string.go"),
            Textmate("storage.type.uintptr.go"),
        ],
        palette.yellow(),
    );
    builder.add_rules(
        &[
            Semantic("interface"),
            Semantic("typeAlias.trait"),
            Semantic("typeParameter"),
            Textmate("entity.name.type.interface"),
            Textmate("entity.name.type.type-parameter"),
        ],
        palette.blue(),
    );

    builder.add_rules(
        &[
            Semantic("variable.constant"),
            Semantic("variable.static"),
            Textmate("constant"),
        ],
        palette.pink(),
    );
    builder.add_rule(Semantic("variable.constant.trait"), palette.blue());

    builder.add_rules(
        &[Semantic("number"), Textmate("constant.numeric")],
        palette.magenta(),
    );
    builder.add_rules(
        &[
            Semantic("string"),
            Semantic("characterLiteral"),
            Textmate("string"),
        ],
        palette.green(),
    );

    builder.add_rules(
        &[
            Semantic("property"),
            Textmate("entity.name.variable.field"),
            Textmate("entity.name.variable.property"),
            Textmate("variable.other.member"),
            Textmate("variable.other.object.property"),
            Textmate("variable.other.readwrite.instance"),
            Textmate("support.type.property-name"),
        ],
        palette.lavender(),
    );

    builder.add_rules(
        &[
            Semantic("enumMember"),
            Semantic("boolean"),
            Textmate("entity.name.variable.enum-member"),
            Textmate("constant.other.enum"),
            Textmate("variable.other.enummember"),
            Textmate("entity.name.type.option"),
            Textmate("entity.name.type.result"),
        ],
        palette.lavender(),
    );

    builder.add_rules(
        &[
            Semantic("macro"),
            Textmate("entity.name.function.macro"),
            Textmate("entity.name.function.preprocessor"),
        ],
        palette.light_green(),
    );

    builder.add_rules(
        &[
            Semantic("formatSpecifier"),
            Semantic("escapeSequence"),
            Textmate("constant.other.placeholder"),
            Textmate("punctuation.definition.interpolation"),
            Textmate("punctuation.section.embedded"),
            Textmate("constant.character.escape"),
        ],
        palette.lavender(),
    );

    builder.add_rule(Semantic("lifetime"), palette.blue());

    builder.add_rules(
        &[Semantic("comment"), Textmate("comment")],
        (palette.base(BaseScale::BrightFg), FontStyle::Italic),
    );

    builder.add_rules(
        &[
            Semantic("comment.documentation"),
            Textmate("comment.line.documentation"),
            Textmate("comment.block.documentation"),
            Textmate("comment.block.javadoc"),
        ],
        palette.base(BaseScale::BrightFg),
    );

    builder.add_rules(
        &[
            Semantic("*.attribute"),
            Textmate("entity.name.function.decorator"),
            Textmate("storage.type.annotation"),
            Textmate("punctuation.definition.annotation"),
        ],
        palette.base(BaseScale::DimFg),
    );

    builder.add_rule(Textmate("keyword.operator"), palette.base(BaseScale::Fg));

    builder.add_rule(Semantic("unresolvedReference"), palette.red());

    builder.add_rule(Textmate("markup.deleted"), palette.red());
    builder.add_rule(Textmate("markup.inserted"), palette.green());
    builder.add_rule(Textmate("markup.changed"), palette.blue());
    builder.add_rule(Textmate("meta.diff"), palette.base(BaseScale::DarkFg));

    builder.add_rule(
        Textmate("comment.line.number-sign.git-commit"),
        palette.base(BaseScale::DimFg),
    );
    builder.add_rule(
        Textmate("invalid.deprecated.line-too-long.git-commit"),
        palette.pink(),
    );
    builder.add_rule(
        Textmate("invalid.illegal.line-too-long.git-commit"),
        palette.red(),
    );

    builder.add_rules(
        &[
            Semantic("magit-ref-name"),
            Semantic("magit-remote-ref-name"),
        ],
        palette.yellow(),
    );
    builder.add_rule(Semantic("magit-tag-name"), palette.magenta());
    builder.add_rule(Textmate("magit.entity"), palette.green());
    builder.add_rule(Textmate("magit.header"), palette.light_blue());
    builder.add_rule(Textmate("magit.subheader"), palette.blue());

    builder.add_rule(Semantic("*.unsafe"), (palette.red(), FontStyle::Bold));
    builder.add_rule(Semantic("*.mutable"), FontStyle::Underline);
    builder.add_rule(Semantic("*.consuming"), FontStyle::Italic);
    builder.add_rule(Semantic("*.public.declaration"), palette.magenta());
}
