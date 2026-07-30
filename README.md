# zed-glua

A [Zed](https://zed.dev/) extension that adds **GLua support** (Lua + [Garry's Mod API](https://wiki.facepunch.com/gmod)).

## 🍷 Features

- **Auto Complete** - type `play` and you already see `player.GetAll()`, `player.GetByID()`.
- **Go to Definition** - click on a function and jump to its declaration.
- **Find References** - on a hovered function, select the option and see every place it was called.
- **Rename Symbol** - rename a variable and it changes everywhere in the project.
- **Hover** - hover over a text object and get all available information about it.
- **Diagnostics** - errors and warnings right in the code, such as unknown variables and unused function fields.
- **Auto-detect Addon API** - adds global variables from your addon into a unified knowledge base, so you can use functions and variables from other addons.
- **Syntax Highlighting** - full Lua highlighting plus 200+ variables from Garry's Mod.
- **Code Outline** - on a separate panel, see all functions and variables with their nesting level.
- **EmmyLua Doc** - annotations and type definitions, similar to TypeScript, for convenient documentation.
- **Data Transparency** - on hover, display constant variable values and their types.
- **Auto-Format** - excellent formatting on save thanks to EmmyLuaCodeStyle.
- **And many other improvements.**

## 📦 Installation

To install zed-glua, you can use the **Extension menu** in Zed, or clone the repository and install it as a dev extension with `zed: install dev extension`.

https://github.com/user-attachments/assets/a8f1becb-a7f3-4b63-998a-a5239c2e6286

## 🔧 Configuration

This extension can be configured via your Zed `settings.json`. The default configuration looks like this:

```jsonc
"lsp": {
    "lua-language-server": {
        "settings": {
            "gmod": {
                // Enable GMod support.
                "enabled": true,
                "download_library": true,
                // Check for API updates on startup. If disabled, check
                // automatically once a day.
                "refresh_library": false,
                // Custom path to the API library, if a specific save location
                // is required.
                "library_path": null,
                // Auto-detect GMod addon directory structure
                // (lua/autorun/, lua/entities/, etc.)
                "auto_detect_addon": true,
            },
            "binary": {
                // Do not use system lua-language-server.
                "ignore_system_version": false,
                // The path to the language server binary you want to force
                // the extension to use.
                "path": null,
                // Additional arguments to pass to the language server.
                "args": [],
            },
        },
    },
}
```

## Common Issues

1. **Files are opened as Lua instead of GLua**

Zed may not automatically recognize `.lua` files as GLua, which prevents the extension from working. To fix this, open your Settings and add the following configuration:

```jsonc
"file_types": {
    "GLua": ["lua"]
}
```

2. **Extension doesn't work**

After installing the extension, the language server and API definitions may not load until you restart Zed.

## 🧪 Development

Build the extension:

```sh
cd zed-glua
cargo build --release
```

## 🔐 License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

Copyright (C) 2026 darkfated.
