# duat-catppuccin ![License: AGPL-3.0-or-later](https://img.shields.io/badge/license-AGPL--3.0--or--later-blue) [![duat-catppuccin on crates.io](https://img.shields.io/crates/v/duat-catppuccin)](https://crates.io/crates/duat-catppuccin) [![duat-catppuccin on docs.rs](https://docs.rs/duat-catppuccin/badge.svg)](https://docs.rs/duat-catppuccin) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/AhoyISki/duat-catppuccin)

## Duat Catppuccin

This is an implementation of the [Catppuccin][__link0]
colorschemes for Duat.

When you `plug` this plugin, four colorschemes will be added to
Duat:

* `catppuccin-latte`;
* `catppuccin-`frappe\`;
* `catppuccin-macchiato`;
* `catppuccin-mocha`;

This plugin lets you use its colors to modify other `Form`s with
the `Catppuccin::modify` function. It also has a `no_background`
function, if you don’t want the background to change.

## Installation

Just like other Duat plugins, this one can be installed by calling
`cargo add` in the config directory:

```bash
cargo add duat-catppuccin@"*" --rename catppuccin
```

Or, if you are using a `--git-deps` version of duat, do this:

```bash
cargo add --git https://github.com/AhoyISki/duat-catppuccin --rename catppuccin
```


 [__link0]: https://catppuccin.com
