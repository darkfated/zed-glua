use std::path::Path;

pub fn detect_addon_paths(worktree_root: &str) -> Vec<String> {
    let root = Path::new(worktree_root);
    let lua_dir = root.join("lua");

    if !lua_dir.is_dir() {
        return Vec::new();
    }

    let mut dirs = Vec::new();
    let mut stack = vec![lua_dir.clone()];

    while let Some(current) = stack.pop() {
        if !current.is_dir() {
            continue;
        }

        if current != lua_dir {
            dirs.push(current.to_string_lossy().replace('\\', "/"));
        }

        if let Ok(entries) = std::fs::read_dir(&current) {
            for entry in entries.flatten() {
                if entry.file_type().is_ok_and(|ft| ft.is_dir()) {
                    stack.push(entry.path());
                }
            }
        }
    }

    dirs
}
