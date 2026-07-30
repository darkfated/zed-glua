# zed-glua

A [Zed](https://zed.dev/) extension that adds **GLua support** (Lua +
[Garry's Mod API](https://wiki.facepunch.com/gmod)).

## ✈️ Installation

To install zed-glua, you can use the extension menu in Zed, or clone the repository and install it as a dev extension with `zed: install dev extension`.

## 🔧 Configuration

This extension can be configured via your Zed `settings.json`. The default configuration looks like this:

```jsonc
{
    "lsp": {
        "lua-language-server": {
            "settings": {
                "gmod": {
                    // Enable GMod support.
                    "enabled": true,
                    "download_library": true,
                    // Check for API updates on startup.
                    "refresh_library": true,
                    // Custom path to the API library, if a specific save location is required.
                    "library_path": null,
                },
                "binary": {
                    // Do not use system lua-language-server.
                    "ignore_system_version": false,
                    // The path to the language server binary you want to force the extension
                    // to use.
                    "path": null,
                    // Additional arguments to pass to the language server.
                    "args": [],
                },
            },
        },
    },
}
```

## 🧪 Testing

Build the extension:

```sh
cd zed-glua
cargo build --release
```

## 🔐 License
This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

Copyright (C) 2026 darkfated.
