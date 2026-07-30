use std::fs;
use std::path::Path;
use zed_extension_api::{self as zed, Result};

const GMOD_API_REPO: &str = "luttje/glua-api-snippets";
pub const LIBRARY_DIR: &str = "garrysmod-library";

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

fn get_library_version(path: &Path) -> Option<String> {
    let version_file = path.join(".version");

    fs::read_to_string(version_file)
        .ok()
        .map(|version| version.trim().to_string())
}

fn set_library_version(path: &Path, version: &str) -> Result<()> {
    let version_file = path.join(".version");

    fs::write(version_file, version)
        .map_err(|e| format!("failed to write library version: {e}"))?;

    Ok(())
}

fn is_valid_library(path: &Path) -> bool {
    is_dir(path) && has_lua_files(path)
}

pub fn download_library(library_path: &Path, release: &zed::GithubRelease) -> Result<()> {
    let asset = release
        .assets
        .iter()
        .find(|asset| asset.name.ends_with(".lua.zip"))
        .ok_or_else(|| "no .lua.zip asset found in latest glua-api-snippets release".to_string())?;

    let temp_path = library_path.with_extension("tmp");

    if is_dir(&temp_path) {
        fs::remove_dir_all(&temp_path)
            .map_err(|e| format!("failed to remove temporary library: {e}"))?;
    }

    zed::download_file(
        &asset.download_url,
        temp_path.to_string_lossy().as_ref(),
        zed::DownloadedFileType::Zip,
    )
    .map_err(|e| format!("failed to download GMod API library: {e}"))?;

    if !is_valid_library(&temp_path) {
        let _ = fs::remove_dir_all(&temp_path);

        return Err("downloaded GMod API library is invalid or contains no lua files".into());
    }

    if is_dir(library_path) {
        fs::remove_dir_all(library_path)
            .map_err(|e| format!("failed to remove old GMod library: {e}"))?;
    }

    fs::rename(&temp_path, library_path)
        .map_err(|e| format!("failed to replace GMod library: {e}"))?;

    set_library_version(library_path, &release.version)?;

    Ok(())
}

pub fn ensure_library(current_dir: &str, refresh: bool) -> Result<String> {
    let library_path = Path::new(current_dir).join(LIBRARY_DIR);

    if refresh && is_dir(&library_path) {
        fs::remove_dir_all(&library_path)
            .map_err(|e| format!("failed to remove old GMod library: {e}"))?;
    }

    let release = match zed::latest_github_release(
        GMOD_API_REPO,
        zed::GithubReleaseOptions {
            require_assets: true,
            pre_release: false,
        },
    ) {
        Ok(release) => release,

        Err(error) => {
            if is_valid_library(&library_path) {
                return Ok(library_path.to_string_lossy().into_owned());
            }

            return Err(format!("failed to fetch latest GMod API release: {error}"));
        }
    };

    let current_version = get_library_version(&library_path);

    let needs_update =
        !is_valid_library(&library_path) || current_version.as_deref() != Some(&release.version);

    if needs_update {
        download_library(&library_path, &release)?;
    }

    if !is_valid_library(&library_path) {
        return Err("GMod API library download succeeded but validation failed".into());
    }

    Ok(library_path.to_string_lossy().into_owned())
}
