use std::fs;
use std::path::Path;
use zed_extension_api::{self as zed, Result};

const GMOD_API_REPO: &str = "luttje/glua-api-snippets";
pub const LIBRARY_DIR: &str = "garrysmod-library";

fn is_dir(path: impl AsRef<Path>) -> bool {
    fs::metadata(path).is_ok_and(|stat| stat.is_dir())
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

pub fn download_library(library_path: &Path, version: &str) -> Result<()> {
    let release = zed::latest_github_release(
        GMOD_API_REPO,
        zed::GithubReleaseOptions {
            require_assets: true,
            pre_release: false,
        },
    )?;

    let asset = release
        .assets
        .iter()
        .find(|asset| asset.name.ends_with(".lua.zip"))
        .ok_or_else(|| "no .lua.zip asset found in latest glua-api-snippets release".to_string())?;

    if is_dir(library_path) {
        fs::remove_dir_all(library_path)
            .map_err(|e| format!("failed to remove old GMod library: {e}"))?;
    }

    zed::download_file(
        &asset.download_url,
        library_path.to_string_lossy().as_ref(),
        zed::DownloadedFileType::Zip,
    )
    .map_err(|e| format!("failed to download GMod API library: {e}"))?;

    set_library_version(library_path, version)?;

    Ok(())
}

pub fn ensure_library(current_dir: &str, refresh: bool) -> Result<String> {
    let library_path = Path::new(current_dir).join(LIBRARY_DIR);

    if refresh && is_dir(&library_path) {
        fs::remove_dir_all(&library_path)
            .map_err(|e| format!("failed to remove old GMod library: {e}"))?;
    }

    let release = zed::latest_github_release(
        GMOD_API_REPO,
        zed::GithubReleaseOptions {
            require_assets: true,
            pre_release: false,
        },
    )?;

    let current_version = get_library_version(&library_path);

    let needs_update =
        !has_lua_files(&library_path) || current_version.as_deref() != Some(&release.version);

    if needs_update {
        download_library(&library_path, &release.version)?;
    }

    if !has_lua_files(&library_path) {
        return Err("GMod API library download succeeded but no .lua files were found".into());
    }

    Ok(library_path.to_string_lossy().into_owned())
}
