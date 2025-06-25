//! # Duat Catppuccin
//!
//! This is an implementation of the [Catppuccin](https://catppuccin.com)
//! colorschemes for Duat.
//!
//! When you `plug` this plugin, four colorschemes will be added to
//! Duat:
//!
//! * `catppuccin-latte`;
//! * `catppuccin-`frappe`;
//! * `catppuccin-macchiato`;
//! * `catppuccin-mocha`;
//!
//! This plugin lets you use its colors to modify other `Form`s with
//! the `Catppuccin::modify` function. It also has a `no_background`
//! function, if you don't want the background to change.
use std::marker::PhantomData;

use duat_core::form::{self, Form, add_colorscheme};

pub struct Catppuccin<U> {
    no_background: bool,
    modifications: Box<dyn Fn(Colors) + Send + Sync + 'static>,
    _u: PhantomData<U>,
}

impl<U: duat_core::ui::Ui> duat_core::Plugin<U> for Catppuccin<U> {
    fn new() -> Self {
        Self {
            no_background: false,
            modifications: Box::new(|_| {}),
            _u: PhantomData,
        }
    }

    /// Adds the catppuccin colorschemes
    ///
    /// This will add the Latte, Frappe, Macchiato, and Mocha flavors,
    /// modified by the options passed to [`Catppuccin`]
    fn plug(self) {
        let no_bg = self.no_background;
        let m = Box::leak(self.modifications);
        add_colorscheme(ColorScheme::latte(m).no_bg(no_bg));
        add_colorscheme(ColorScheme::frappe(m).no_bg(no_bg));
        add_colorscheme(ColorScheme::macchiato(m).no_bg(no_bg));
        add_colorscheme(ColorScheme::mocha(m).no_bg(no_bg));
    }
}

impl<U> Catppuccin<U> {
    /// Disables the background color
    ///
    /// This can allow you to have, for example, a transparent
    /// terminal.
    pub fn no_background(self) -> Self {
        Self { no_background: true, ..self }
    }

    /// Lets you modify forms, based on the chosen colorscheme
    ///
    /// For example, if you want red delimiters, you can do this:
    ///
    /// ```rust
    /// # use duat_core::form;
    /// # use duat_catppuccin as catppuccin;
    /// # macro_rules! plug { ($($_:tt)*) => {} };
    /// # fn plug() {}
    /// use catppuccin::Catppuccin;
    ///
    /// plug!(Catppuccin::new().modify(|colors| {
    ///     form::set("punctuation.delimiter", colors.red);
    /// }));
    /// ```
    pub fn modify(self, modifications: impl Fn(Colors) + Send + Sync + 'static) -> Self {
        let modifications = Box::new(move |c| {
            modifications(c);
        });
        Self { modifications, ..self }
    }
}

#[derive(Default)]
enum Flavour {
    Latte,
    Frappe,
    Macchiato,
    #[default]
    Mocha,
}

struct ColorScheme {
    flavour: Flavour,
    no_background: bool,
    modifications: &'static (dyn Fn(Colors) + Send + Sync),
}

impl form::ColorScheme for ColorScheme {
    fn apply(&self) {
        let c = match self.flavour {
            Flavour::Latte => LATTE,
            Flavour::Frappe => FRAPPE,
            Flavour::Macchiato => MACCHIATO,
            Flavour::Mocha => MOCHA,
        };

        if self.no_background {
            form::set("default", Form::with(c.text));
        } else {
            form::set("default", Form::with(c.text).on(c.base));
        }

        form::set_many!(
            // Base Duat Forms
            ("accent", Form::with(c.rosewater).bold()),
            ("default.error", Form::with(c.maroon)),
            ("accent.error", Form::with(c.red).bold()),
            ("default.warn", Form::with(c.yellow)),
            ("accent.warn", Form::with(c.peach).bold()),
            ("default.info", Form::with(c.sapphire)),
            ("accent.info", Form::with(c.sky).bold()),
            ("default.debug", Form::with(c.subtext1)),
            ("accent.debug", Form::with(c.lavender).bold()),
            ("caret.main", Form::reverse()),
            ("caret.extra", Form::reverse()),
            ("selection.main", Form::with(c.base).on(c.overlay1)),
            ("selection.extra", Form::with(c.base).on(c.overlay0)),
            // Other Duat Forms
            ("linenum.main", Form::with(c.yellow)),
            ("linenum.wrapped", Form::with(c.teal)),
            ("file", Form::with(c.yellow)),
            ("selections", Form::with(c.blue)),
            ("coord", Form::with(c.peach)),
            ("separator", Form::with(c.teal)),
            ("mode", Form::with(c.green)),
            // Tree sitter Forms
            ("variable", Form::with(c.text)),
            ("variable.builtin", Form::with(c.peach)),
            ("constant", Form::with(c.peach)),
            ("constant.builtin", Form::with(c.peach)),
            ("module", Form::with(c.blue).italic()),
            ("label", Form::with(c.green)),
            ("string", Form::with(c.green)),
            ("string.escape", Form::with(c.peach)),
            ("character", Form::with(c.peach)),
            ("boolean", Form::with(c.peach)),
            ("number", Form::with(c.peach)),
            ("type", Form::with(c.yellow).italic()),
            ("type.builtin", Form::with(c.yellow).reset()),
            ("attribute", Form::with(c.green)),
            ("property", Form::with(c.text)),
            ("function", Form::with(c.blue).reset()),
            ("function.macro", Form::with(c.lavender).italic()),
            ("constructor", Form::with(c.peach)),
            ("operator", Form::with(c.sapphire)),
            ("keyword", Form::with(c.mauve)),
            ("punctuation.bracket", Form::with(c.subtext0)),
            ("punctuation.delimiter", Form::with(c.subtext0)),
            ("comment", Form::with(c.overlay1)),
            ("comment.documentation", Form::with(c.overlay1).bold()),
            ("markup", Form::new()),
            ("markup.strong", Form::with(c.maroon).bold()),
            ("markup.italic", Form::with(c.maroon).italic()),
            ("markup.strikethrough", Form::new().crossed_out()),
            ("markup.underline", Form::underlined()),
            ("markup.heading", Form::with(c.blue).bold()),
            ("markup.math", Form::with(c.yellow)),
            ("markup.quote", Form::with(c.maroon).bold()),
            ("markup.environment", Form::with(c.pink)),
            ("markup.environment.name", Form::with(c.blue)),
            ("markup.link", Form::with(c.lavender).underlined()),
            ("markup.raw", Form::with(c.teal)),
            ("markup.list", Form::with(c.yellow)),
            ("markup.list.checked", Form::with(c.green)),
            ("markup.list.unchecked", Form::with(c.overlay1)),
            // Plugin and Ui Forms
            ("terminal.frame", Form::with(c.subtext0)),
            ("notifs.target", Form::with(c.subtext1)),
            ("notifs.colon", Form::with(c.subtext0)),
            ("prompt", Form::with(c.green)),
            ("prompt.colon", Form::with(c.subtext0)),
            ("default.StatusLine", Form::on(c.surface0)),
            ("default.VertRule", Form::with(c.surface0)),
            ("default.LineNumbers", Form::with(c.overlay0)),
            // For duat-kak
            ("caret.main.Normal", Form::with(c.base).on(c.text)),
            ("caret.extra.Normal", Form::with(c.base).on(c.sapphire)),
            ("caret.main.Insert", Form::with(c.base).on(c.mauve)),
            ("caret.extra.Insert", Form::with(c.base).on(c.yellow)),
        );

        (self.modifications)(c)
    }

    fn name(&self) -> &'static str {
        match self.flavour {
            Flavour::Latte => "catppuccin-latte",
            Flavour::Frappe => "catppuccin-frappe",
            Flavour::Macchiato => "catppuccin-macchiato",
            Flavour::Mocha => "catppuccin-mocha",
        }
    }
}

impl ColorScheme {
    /// Returns the Catppuccin [`ColorScheme`] in the Latte flavour
    fn latte(modifications: &'static (dyn Fn(Colors) + Send + Sync)) -> Self {
        Self {
            flavour: Flavour::Latte,
            no_background: false,
            modifications,
        }
    }

    /// Returns the Catppuccin [`ColorScheme`] in the Frappe flavour
    fn frappe(modifications: &'static (dyn Fn(Colors) + Send + Sync)) -> Self {
        Self {
            flavour: Flavour::Frappe,
            no_background: false,
            modifications,
        }
    }

    /// Returns the Catppuccin [`ColorScheme`] in the Macchiato
    /// flavour
    fn macchiato(modifications: &'static (dyn Fn(Colors) + Send + Sync)) -> Self {
        Self {
            flavour: Flavour::Macchiato,
            no_background: false,
            modifications,
        }
    }

    /// Returns the Catppuccin [`ColorScheme`] in the Mocha flavour
    fn mocha(modifications: &'static (dyn Fn(Colors) + Send + Sync)) -> Self {
        Self {
            flavour: Flavour::Mocha,
            no_background: false,
            modifications,
        }
    }

    /// Removes the background color
    ///
    /// This can allow, for example, transparent backgrounds in
    /// terminal interfaces.
    fn no_bg(self, bool: bool) -> Self {
        Self { no_background: bool, ..self }
    }
}

pub struct Colors {
    pub rosewater: &'static str,
    pub flamingo: &'static str,
    pub pink: &'static str,
    pub mauve: &'static str,
    pub red: &'static str,
    pub maroon: &'static str,
    pub peach: &'static str,
    pub yellow: &'static str,
    pub green: &'static str,
    pub teal: &'static str,
    pub sky: &'static str,
    pub sapphire: &'static str,
    pub blue: &'static str,
    pub lavender: &'static str,
    pub text: &'static str,
    pub subtext1: &'static str,
    pub subtext0: &'static str,
    pub overlay2: &'static str,
    pub overlay1: &'static str,
    pub overlay0: &'static str,
    pub surface2: &'static str,
    pub surface1: &'static str,
    pub surface0: &'static str,
    pub base: &'static str,
    pub mantle: &'static str,
    pub crust: &'static str,
}

const LATTE: Colors = Colors {
    rosewater: "#dc8a78",
    flamingo: "#dd7878",
    pink: "#ea76cb",
    mauve: "#8839ef",
    red: "#d20f39",
    maroon: "#e64553",
    peach: "#fe640b",
    yellow: "#df8e1d",
    green: "#40a02b",
    teal: "#179299",
    sky: "#04a5e5",
    sapphire: "#209fb5",
    blue: "#1e66f5",
    lavender: "#7287fd",
    text: "#4c4f69",
    subtext1: "#5c5f77",
    subtext0: "#6c6f85",
    overlay2: "#7c7f93",
    overlay1: "#8c8fa1",
    overlay0: "#9ca0b0",
    surface2: "#acb0be",
    surface1: "#bcc0cc",
    surface0: "#ccd0da",
    base: "#eff1f5",
    mantle: "#e6e9ef",
    crust: "#dce0e8",
};
const FRAPPE: Colors = Colors {
    rosewater: "#f2d5cf",
    flamingo: "#eebebe",
    pink: "#f4b8e4",
    mauve: "#ca9ee6",
    red: "#e78284",
    maroon: "#ea999c",
    peach: "#ef9f76",
    yellow: "#e5c890",
    green: "#a6d189",
    teal: "#81c8be",
    sky: "#99d1db",
    sapphire: "#85c1dc",
    blue: "#8caaee",
    lavender: "#babbf1",
    text: "#c6d0f5",
    subtext1: "#b5bfe2",
    subtext0: "#a5adce",
    overlay2: "#949cbb",
    overlay1: "#838ba7",
    overlay0: "#737994",
    surface2: "#626880",
    surface1: "#51576d",
    surface0: "#414559",
    base: "#303446",
    mantle: "#292c3c",
    crust: "#232634",
};

const MACCHIATO: Colors = Colors {
    rosewater: "#f4dbd6",
    flamingo: "#f0c6c6",
    pink: "#f5bde6",
    mauve: "#c6a0f6",
    red: "#ed8796",
    maroon: "#ee99a0",
    peach: "#f5a97f",
    yellow: "#eed49f",
    green: "#a6da95",
    teal: "#8bd5ca",
    sky: "#91d7e3",
    sapphire: "#7dc4e4",
    blue: "#8aadf4",
    lavender: "#b7bdf8",
    text: "#cad3f5",
    subtext1: "#b8c0e0",
    subtext0: "#a5adcb",
    overlay2: "#939ab7",
    overlay1: "#8087a2",
    overlay0: "#6e738d",
    surface2: "#5b6078",
    surface1: "#494d64",
    surface0: "#363a4f",
    base: "#24273a",
    mantle: "#1e2030",
    crust: "#181926",
};

const MOCHA: Colors = Colors {
    rosewater: "#f5e0dc",
    flamingo: "#f2cdcd",
    pink: "#f5c2e7",
    mauve: "#cba6f7",
    red: "#f38ba8",
    maroon: "#eba0ac",
    peach: "#fab387",
    yellow: "#f9e2af",
    green: "#a6e3a1",
    teal: "#94e2d5",
    sky: "#89dceb",
    sapphire: "#74c7ec",
    blue: "#89b4fa",
    lavender: "#b4befe",
    text: "#cdd6f4",
    subtext1: "#bac2de",
    subtext0: "#a6adc8",
    overlay2: "#9399b2",
    overlay1: "#7f849c",
    overlay0: "#6c7086",
    surface2: "#585b70",
    surface1: "#45475a",
    surface0: "#313244",
    base: "#1e1e2e",
    mantle: "#181825",
    crust: "#11111b",
};
