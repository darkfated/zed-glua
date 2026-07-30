# zed-glua

A [Zed](https://zed.dev/) extension that adds **GLua support** (Lua + [Garry's Mod API](https://wiki.facepunch.com/gmod)).

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
