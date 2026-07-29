use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use zed::lsp::CompletionKind;
use zed::serde_json::{Map, Value};
use zed::settings::LspSettings;
use zed::{serde_json, CodeLabel, CodeLabelSpan, LanguageServerId};
use zed_extension_api::{self as zed, Result};

mod gmod;
mod json;

const LUA_LS_BINARY_DIR: &str = "lua-language-server-binaries";

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Settings {
    #[serde(rename = "Lua")]
    lua: Map<String, Value>,
    gmod: GmodSettings,
    binary: BinarySettings,
    library: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            lua: Map::new(),
            gmod: GmodSettings::default(),
            binary: BinarySettings::default(),
            library: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct GmodSettings {
    enabled: bool,
    download_library: bool,
    refresh_library: bool,
    library_path: Option<String>,
}

impl Default for GmodSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            download_library: true,
            refresh_library: true,
            library_path: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct BinarySettings {
    ignore_system_version: bool,
    path: Option<String>,
    args: Vec<String>,
}

impl Default for BinarySettings {
    fn default() -> Self {
        Self {
            ignore_system_version: false,
            path: None,
            args: Vec::new(),
        }
    }
}

struct GluaExtension {
    cached_binary_path: Option<String>,
}

fn is_file(path: &str) -> bool {
    fs::metadata(path).is_ok_and(|stat| stat.is_file())
}

fn is_dir(path: &str) -> bool {
    fs::metadata(path).is_ok_and(|stat| stat.is_dir())
}

fn get_extension_settings(settings_val: Option<Value>) -> Result<Settings> {
    let Some(mut settings_val) = settings_val else {
        return Ok(Settings::default());
    };

    let Some(settings) = settings_val.as_object_mut() else {
        return Err("invalid lua-language-server settings: `settings` must be an object".into());
    };

    let lua_settings = settings.remove("Lua");
    let mut value = settings.remove("ext").unwrap_or(settings_val);
    if let Value::Object(o) = &mut value {
        o.insert(
            "Lua".to_string(),
            lua_settings.unwrap_or(Value::Object(Map::new())),
        );
    }

    serde_path_to_error::deserialize(value).map_err(|e| e.to_string())
}

fn apply_gmod_defaults(lua: &mut Map<String, Value>) {
    let runtime = json::get_or_insert_object(lua, "runtime");
    runtime
        .entry("version".to_string())
        .or_insert(Value::String("LuaJIT".into()));

    let special = json::get_or_insert_object(runtime, "special");
    special
        .entry("include".to_string())
        .or_insert(Value::String("dofile".into()));
    special
        .entry("IncludeCS".to_string())
        .or_insert(Value::String("dofile".into()));

    let symbols = json::get_or_insert_array(runtime, "nonstandardSymbol");
    for symbol in ["!", "!=", "&&", "||", "//", "/**/", "continue"] {
        if !symbols.iter().any(|value| value.as_str() == Some(symbol)) {
            symbols.push(Value::String(symbol.into()));
        }
    }

    let diagnostics = json::get_or_insert_object(lua, "diagnostics");
    let disabled = json::get_or_insert_array(diagnostics, "disable");
    if !disabled
        .iter()
        .any(|value| value.as_str() == Some("duplicate-set-field"))
    {
        disabled.push(Value::String("duplicate-set-field".into()));
    }
}

fn merge_library_paths(
    lua: &mut Map<String, Value>,
    library_paths: impl IntoIterator<Item = String>,
) {
    let workspace = json::get_or_insert_object(lua, "workspace");
    let libraries = json::get_or_insert_array(workspace, "library");

    let mut seen = HashSet::new();
    for value in libraries.iter() {
        if let Some(path) = value.as_str() {
            seen.insert(path.to_string());
        }
    }

    for path in library_paths {
        if seen.insert(path.clone()) {
            libraries.push(Value::String(path));
        }
    }
}

impl GluaExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
        settings: &Settings,
    ) -> Result<String> {
        if let Some(path) = &settings.binary.path {
            return Ok(path.clone());
        }

        if !settings.binary.ignore_system_version {
            if let Some(path) = worktree.which("lua-language-server") {
                return Ok(path);
            }
        }

        if let Some(path) = &self.cached_binary_path {
            if is_file(path) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "LuaLS/lua-language-server",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "lua-language-server-{version}-{os}-{arch}.{extension}",
            version = release.version,
            os = match platform {
                zed::Os::Mac => "darwin",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "win32",
            },
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X8664 => "x64",
                zed::Architecture::X86 => return Err("unsupported platform x86".into()),
            },
            extension = match platform {
                zed::Os::Mac | zed::Os::Linux => "tar.gz",
                zed::Os::Windows => "zip",
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {asset_name:?}"))?;

        let dir_name = format!("lua-language-server-{}", release.version);
        let version_dir = format!("{LUA_LS_BINARY_DIR}/{dir_name}");
        let binary_path = format!(
            "{version_dir}/bin/lua-language-server{extension}",
            extension = match platform {
                zed::Os::Mac | zed::Os::Linux => "",
                zed::Os::Windows => ".exe",
            },
        );

        if !is_dir(LUA_LS_BINARY_DIR) {
            fs::create_dir(LUA_LS_BINARY_DIR).map_err(|e| {
                format!("failed to create lua-language-server binary directory: {e}")
            })?;
        }

        if !is_file(&binary_path) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                match platform {
                    zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                },
            )
            .map_err(|e| format!("failed to download lua-language-server: {e}"))?;

            let entries = fs::read_dir(LUA_LS_BINARY_DIR)
                .map_err(|e| format!("failed to list lua-language-server binary directory: {e}"))?;
            for entry in entries {
                let entry = entry
                    .map_err(|e| format!("failed to load lua-language-server binary entry: {e}"))?;
                if entry.file_name().to_str() != Some(&dir_name) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    fn resolve_gmod_library_path(&self, settings: &Settings) -> Result<Option<String>> {
        if !settings.gmod.enabled {
            return Ok(None);
        }

        if let Some(path) = &settings.gmod.library_path {
            return Ok(Some(path.clone()));
        }

        if !settings.gmod.download_library {
            return Ok(None);
        }

        let current_dir = std::env::current_dir()
            .map_err(|e| format!("failed to get extension working directory: {e}"))?;
        let current_dir_str = current_dir.display().to_string();

        let library_path = gmod::ensure_library(&current_dir_str, settings.gmod.refresh_library)?;

        Ok(Some(library_path))
    }
}

impl zed::Extension for GluaExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        let settings = get_extension_settings(lsp_settings.settings)?;

        // Ensure the GMod API library is downloaded before the language server starts.
        if settings.gmod.enabled
            && settings.gmod.download_library
            && settings.gmod.library_path.is_none()
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            self.resolve_gmod_library_path(&settings)?;
        }

        let binary_path =
            self.language_server_binary_path(language_server_id, worktree, &settings)?;

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

        apply_gmod_defaults(&mut settings.lua);

        let mut library_paths = Vec::new();

        if let Some(path) = self.resolve_gmod_library_path(&settings)? {
            library_paths.push(path);
        }

        let proj_root = format!("{}/", worktree.root_path());
        for path in &settings.library {
            let resolved = if Path::new(path).is_absolute() {
                path.clone()
            } else {
                format!("{proj_root}{path}")
            };
            library_paths.push(resolved);
        }

        merge_library_paths(&mut settings.lua, library_paths);

        Ok(Some(serde_json::json!({
            "Lua": settings.lua
        })))
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<CodeLabel> {
        match completion.kind? {
            CompletionKind::Method | CompletionKind::Function => {
                let name_len = completion.label.find('(').unwrap_or(completion.label.len());
                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(0..completion.label.len())],
                    filter_range: (0..name_len).into(),
                    code: completion.label,
                })
            }
            CompletionKind::Field => Some(CodeLabel {
                spans: vec![CodeLabelSpan::literal(
                    completion.label.clone(),
                    Some("property".into()),
                )],
                filter_range: (0..completion.label.len()).into(),
                code: Default::default(),
            }),
            _ => None,
        }
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &LanguageServerId,
        symbol: zed::lsp::Symbol,
    ) -> Option<CodeLabel> {
        let prefix = "let a = ";
        let suffix = match symbol.kind {
            zed::lsp::SymbolKind::Method => "()",
            _ => "",
        };
        let code = format!("{prefix}{}{suffix}", symbol.name);
        Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(
                prefix.len()..code.len() - suffix.len(),
            )],
            filter_range: (0..symbol.name.len()).into(),
            code,
        })
    }
}

zed::register_extension!(GluaExtension);
