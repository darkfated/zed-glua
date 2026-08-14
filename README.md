# zed-glua

This Zed extension provides **GLua support** by configuring [Lua Language Server](https://github.com/LuaLS/lua-language-server) with the automatically downloaded [Garry's Mod API](https://github.com/luttje/glua-api-snippets).

## 📦 Installation

To install the extension, clone this repository and install it as a dev extension using `zed: install dev extension` (Ctrl+Shift+P).

https://github.com/user-attachments/assets/a8f1becb-a7f3-4b63-998a-a5239c2e6286

> [!WARNING]
> **Rust** and **Visual Studio** with the required C++ components are required to load the extension.
>
> **Visual Studio:**
>
> ```powershell
> winget install --id Microsoft.VisualStudio.2022.Community --source winget --force --override "--add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.VisualStudio.Component.VC.Tools.ARM64 --add Microsoft.VisualStudio.Component.Windows11SDK.22621 --addProductLang En-us"
> ```
>
> **Rust:**
>
> ```powershell
> winget install Rustlang.Rustup
> ```

## 🔧 Configuration

This extension can be configured via your Zed `settings.json`. The default configuration:

```jsonc
"lsp": {
    "lua-language-server": {
        "settings": {
            "gmod": {
                // Enable Gmod support.
                "enabled": true,
                "download_library": true,
                // Check for API updates on startup. If disabled, check
                // automatically once a day.
                "refresh_library": false,
                // Custom path to the API library, if a specific save location
                // is required.
                "library_path": null,
                // Auto-detect Gmod addon directory structure
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

## Issues

1. **Files are opened as Lua**

If you have the standard Lua extension installed, it may conflict with the GLua extension and cause `.lua` files to be opened as Lua instead of GLua.

To fix this, make GLua the default language for `.lua` files by adding the following configuration to your **Zed settings**:

```jsonc
"file_types": {
    "GLua": ["lua"]
}
```

2. **Extension doesn't work**

After installing the extension, restart Zed if the language server or API definitions are not loaded.

## 🔐 License

This project is under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

Copyright (C) 2026 darkfated.
