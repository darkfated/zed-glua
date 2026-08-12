use std::path::Path;

pub fn detect_addon_paths(worktree_root: &str) -> Vec<String> {
    let lua_dir = Path::new(worktree_root).join("lua");

    if lua_dir.is_dir() {
        vec![lua_dir.to_string_lossy().replace('\\', "/")]
    } else {
        Vec::new()
    }
}