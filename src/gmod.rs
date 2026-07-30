use crate::settings::Settings;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use zed_extension_api::{self as zed, Result};

const GMOD_API_REPO: &str = "luttje/glua-api-snippets";
pub const LIBRARY_DIR: &str = "garrysmod-library";

const RELEASE_FILE: &str = "release.json";
const RELEASE_CHECK_INTERVAL: u64 = 60 * 60 * 24;

fn is_dir(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_dir()
}

fn has_lua_files(path: impl AsRef<Path>) -> bool {
    let Ok(entries) = fs::read_dir(path) else {
        return false;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "lua") {
            return true;
        }
        if path.is_dir() && has_lua_files(&path) {
            return true;
        }
    }

    false
}

fn is_valid_library(path: &Path) -> bool {
    is_dir(path) && has_lua_files(path)
}

#[derive(Debug, Serialize, Deserialize)]
struct ReleaseCache {
    version: String,
    checked: u64,
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn read_release_cache(path: &Path) -> Option<ReleaseCache> {
    let file = path.join(RELEASE_FILE);

    let content = fs::read_to_string(file).ok()?;

    serde_json::from_str(&content).ok()
}

fn write_release_cache(path: &Path, release: &zed::GithubRelease) -> Result<()> {
    let cache = ReleaseCache {
        version: release.version.clone(),
        checked: current_timestamp(),
    };

    let content = serde_json::to_string_pretty(&cache)
        .map_err(|e| format!("failed to serialize release cache: {e}"))?;

    fs::write(path.join(RELEASE_FILE), content)
        .map_err(|e| format!("failed to write release cache: {e}"))?;

    Ok(())
}

fn release_check_expired(cache: &ReleaseCache) -> bool {
    current_timestamp().saturating_sub(cache.checked) >= RELEASE_CHECK_INTERVAL
}

fn download_library(library_path: &Path, release: &zed::GithubRelease) -> Result<()> {
    let asset = release
        .assets
        .iter()
        .find(|asset| asset.name.ends_with(".lua.zip"))
        .ok_or_else(|| "no .lua.zip asset found in latest glua-api-snippets release".to_string())?;

    let temp_path = library_path.with_extension("tmp");

    if is_dir(&temp_path) {
        fs::remove_dir_all(&temp_path)
            .map_err(|e| format!("failed to remove temp library: {e}"))?;
    }

    zed::download_file(
        &asset.download_url,
        temp_path.to_string_lossy().as_ref(),
        zed::DownloadedFileType::Zip,
    )
    .map_err(|e| format!("failed to download GMod API library: {e}"))?;

    if !is_valid_library(&temp_path) {
        let _ = fs::remove_dir_all(&temp_path);

        return Err("downloaded GMod API library validation failed".into());
    }

    if is_dir(library_path) {
        fs::remove_dir_all(library_path)
            .map_err(|e| format!("failed to remove old library: {e}"))?;
    }

    fs::rename(&temp_path, library_path).map_err(|e| format!("failed to replace library: {e}"))?;

    write_release_cache(library_path, release)?;

    Ok(())
}

fn ensure_library(current_dir: &str, refresh: bool) -> Result<String> {
    let library_path = Path::new(current_dir).join(LIBRARY_DIR);

    let cache = read_release_cache(&library_path);

    let need_check_release = refresh || cache.as_ref().is_none_or(release_check_expired);

    let release = if need_check_release {
        match zed::latest_github_release(
            GMOD_API_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) {
            Ok(release) => Some(release),

            Err(error) => {
                if is_valid_library(&library_path) {
                    return Ok(library_path.to_string_lossy().into_owned());
                }

                return Err(format!("failed to check GMod API release: {error}"));
            }
        }
    } else {
        None
    };

    let needs_update = match &release {
        Some(release) => cache
            .as_ref()
            .map(|cache| cache.version != release.version)
            .unwrap_or(true),

        None => !is_valid_library(&library_path),
    };

    if needs_update {
        if let Some(release) = release {
            download_library(&library_path, &release)?;
        }
    }

    if !is_valid_library(&library_path) {
        return Err("GMod API library is missing or invalid".into());
    }

    Ok(library_path.to_string_lossy().into_owned())
}

pub struct GmodLibrary {
    cached_path: Option<String>,
}

impl GmodLibrary {
    pub fn new() -> Self {
        Self { cached_path: None }
    }

    pub fn resolve(&mut self, settings: &Settings) -> Result<Option<String>> {
        if !settings.gmod.enabled {
            return Ok(None);
        }

        if let Some(path) = &settings.gmod.library_path {
            return Ok(Some(path.clone()));
        }

        if !settings.gmod.download_library {
            return Ok(None);
        }

        if !settings.gmod.refresh_library {
            if let Some(path) = &self.cached_path {
                return Ok(Some(path.clone()));
            }
        }

        let current_dir = std::env::current_dir()
            .map_err(|e| format!("failed to get extension working directory: {e}"))?;
        let current_dir_str = current_dir.display().to_string();

        let library_path = ensure_library(&current_dir_str, settings.gmod.refresh_library)?;

        self.cached_path = Some(library_path.clone());
        Ok(Some(library_path))
    }
}
