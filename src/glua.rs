use std::path::Path;
use zed::settings::LspSettings;
use zed::{serde_json, CodeLabel, LanguageServerId};
use zed_extension_api::{self as zed, Result};

mod gmod;
mod labels;
mod lsp;
mod settings;
mod util;

use gmod::library::GmodLibrary;
use lsp::server::LuaLsBinary;
use settings::get_extension_settings;

struct GluaExtension {
    lua_ls_binary: LuaLsBinary,
    gmod_library: GmodLibrary,
}

impl zed::Extension for GluaExtension {
    fn new() -> Self {
        Self {
            lua_ls_binary: LuaLsBinary::new(),
            gmod_library: GmodLibrary::new(),
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        let settings = get_extension_settings(lsp_settings.settings)?;

        if settings.gmod.enabled
            && settings.gmod.download_library
            && settings.gmod.library_path.is_none()
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            self.gmod_library.resolve(language_server_id, &settings)?;
        }

        let binary_path =
            self.lua_ls_binary
                .resolve(language_server_id, worktree, &settings.binary)?;

        Ok(zed::Command {
            command: binary_path,
            args: settings.binary.args,
            env: vec![],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(
            LspSettings::for_worktree(language_server_id.as_ref(), worktree)
                .ok()
                .and_then(|lsp_settings| lsp_settings.initialization_options.clone()),
        )
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        let mut settings = get_extension_settings(lsp_settings.settings)?;

        lsp::config::apply_gmod_lua_defaults(&mut settings.lua);

        let proj_root = worktree.root_path();
        let mut library_paths = Vec::with_capacity(settings.library.len() + 10);

        if let Some(path) = self.gmod_library.resolve(language_server_id, &settings)? {
            library_paths.push(path);
        }

        if settings.gmod.enabled && settings.gmod.auto_detect_addon {
            let detected = gmod::addon::detect_addon_paths(&proj_root);
            library_paths.extend(detected);
        }

        library_paths.extend(settings.library.iter().map(|path| {
            if Path::new(path).is_absolute() {
                path.clone()
            } else {
                format!("{proj_root}/{path}")
            }
        }));

        lsp::config::merge_library_paths(&mut settings.lua, library_paths);

        Ok(Some(serde_json::json!({
            "Lua": settings.lua
        })))
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<CodeLabel> {
        labels::label_for_completion(completion)
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &LanguageServerId,
        symbol: zed::lsp::Symbol,
    ) -> Option<CodeLabel> {
        labels::label_for_symbol(symbol)
    }
}

zed::register_extension!(GluaExtension);
