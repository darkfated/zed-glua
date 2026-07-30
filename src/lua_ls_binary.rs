use crate::fs_util::{is_dir, is_file};
use crate::settings::BinarySettings;
use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result};

const LUA_LS_BINARY_DIR: &str = "lua-language-server-binaries";

fn exe_suffix(platform: zed::Os) -> &'static str {
    match platform {
        zed::Os::Windows => ".exe",
        zed::Os::Mac | zed::Os::Linux => "",
    }
}

fn archive_extension(platform: zed::Os) -> &'static str {
    match platform {
        zed::Os::Mac | zed::Os::Linux => "tar.gz",
        zed::Os::Windows => "zip",
    }
}

fn asset_os_name(platform: zed::Os) -> &'static str {
    match platform {
        zed::Os::Mac => "darwin",
        zed::Os::Linux => "linux",
        zed::Os::Windows => "win32",
    }
}

pub struct LuaLsBinary {
    cached_binary_path: Option<String>,
}

impl LuaLsBinary {
    pub fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    pub fn resolve(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
        binary_settings: &BinarySettings,
    ) -> Result<String> {
        if let Some(path) = &binary_settings.path {
            if !is_file(path) {
                return Err(format!(
                    "configured lua-language-server binary path does not exist: {path}"
                ));
            }
            return Ok(path.clone());
        }

        if !binary_settings.ignore_system_version {
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
        let arch_name = match arch {
            zed::Architecture::Aarch64 => "arm64",
            zed::Architecture::X8664 => "x64",
            zed::Architecture::X86 => return Err("unsupported platform x86".into()),
        };

        let asset_name = format!(
            "lua-language-server-{version}-{os}-{arch}.{extension}",
            version = release.version,
            os = asset_os_name(platform),
            arch = arch_name,
            extension = archive_extension(platform),
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {asset_name:?}"))?;

        let dir_name = format!("lua-language-server-{}", release.version);
        let version_dir = format!("{LUA_LS_BINARY_DIR}/{dir_name}");
        let binary_path = format!(
            "{version_dir}/bin/lua-language-server{}",
            exe_suffix(platform),
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

            Self::prune_old_versions(&dir_name)?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    fn prune_old_versions(current_dir_name: &str) -> Result<()> {
        let entries = fs::read_dir(LUA_LS_BINARY_DIR)
            .map_err(|e| format!("failed to list lua-language-server binary directory: {e}"))?;

        for entry in entries {
            let entry = entry
                .map_err(|e| format!("failed to load lua-language-server binary entry: {e}"))?;
            if entry.file_name().to_str() != Some(current_dir_name) {
                let _ = fs::remove_dir_all(entry.path());
            }
        }

        Ok(())
    }
}
