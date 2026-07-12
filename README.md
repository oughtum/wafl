<h1 align="center">wafl</h1>

`wafl` is a WIP functional language inspired by Nix, as well as Haskell and Rust.
The primary goal of `wafl` is to be used as part of a (not yet developed) larger system for maintaining dotfiles in a declarative and unified way, akin to NixOS/home-manager but without the unnecessary complexity and with a smaller scope.

The language spec can be found in [wafl-interp](https://github.com/oughtum/wafl-interp/blob/main/spec.wfl) which provides a decent overview of the full language syntax as it stands currently, though things are very much subject to change.

`wafl` should also function as a scripting language which can be easily embedded in other applications, whether as a configuration alternative to TOML/YAML/XML etc., as a user-facing language for runtime scripting or for any other number of things scripting languages are commonly used.

If you think `wafl` is a language you'd be interested in using but feel some things could be tweaked, please feel free to [open an issue](https://github.com/oughtum/wafl/issues) and make a suggestion! Though ultimately, this is a personal project, first and foremost, and as such the design of the language is entirely at my own discretion.
