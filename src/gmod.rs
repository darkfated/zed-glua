use std::fs;
use zed_extension_api::{self as zed, Result};

const GMOD_API_REPO: &str = "luttje/glua-api-snippets";
pub const LIBRARY_DIR: &str = "garrysmod-library";

fn is_dir(path: &str) -> bool {
    fs::metadata(path).is_ok_and(|stat| stat.is_dir())
}

fn has_lua_files(path: &str) -> bool {
    let Ok(entries) = fs::read_dir(path) else {
        return false;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "lua") {
            return true;
        }
        if path.is_dir() && has_lua_files(&path.display().to_string()) {
            return true;
        }
    }

    false
}

pub fn download_library() -> Result<()> {
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

    if is_dir(LIBRARY_DIR) {
        fs::remove_dir_all(LIBRARY_DIR)
            .map_err(|e| format!("failed to remove old GMod library: {e}"))?;
    }

    zed::download_file(
        &asset.download_url,
        LIBRARY_DIR,
        zed::DownloadedFileType::Zip,
    )
    .map_err(|e| format!("failed to download GMod API library: {e}"))?;

    Ok(())
}

pub fn ensure_library(current_dir: &str, refresh: bool) -> Result<String> {
    let library_path = format!("{current_dir}/{LIBRARY_DIR}");

    if refresh && is_dir(LIBRARY_DIR) {
        fs::remove_dir_all(LIBRARY_DIR).ok();
    }

    if !has_lua_files(LIBRARY_DIR) {
        download_library()?;
    }

    if !has_lua_files(LIBRARY_DIR) {
        return Err("GMod API library download succeeded but no .lua files were found".into());
    }

    Ok(library_path)
}
