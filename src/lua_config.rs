use crate::json;
use std::collections::HashSet;
use zed_extension_api::serde_json::{Map, Value};

const NONSTANDARD_SYMBOLS: [&str; 7] = ["!", "!=", "&&", "||", "//", "/**/", "continue"];

pub fn apply_gmod_defaults(lua: &mut Map<String, Value>) {
    let runtime = json::get_or_insert_object(lua, "runtime");
    runtime
        .entry("version".to_string())
        .or_insert_with(|| Value::String("LuaJIT".into()));

    let special = json::get_or_insert_object(runtime, "special");
    special
        .entry("include".to_string())
        .or_insert_with(|| Value::String("dofile".into()));
    special
        .entry("IncludeCS".to_string())
        .or_insert_with(|| Value::String("dofile".into()));

    let symbols = json::get_or_insert_array(runtime, "nonstandardSymbol");
    for symbol in NONSTANDARD_SYMBOLS {
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

pub fn merge_library_paths(
    lua: &mut Map<String, Value>,
    library_paths: impl IntoIterator<Item = String>,
) {
    let workspace = json::get_or_insert_object(lua, "workspace");
    let libraries = json::get_or_insert_array(workspace, "library");

    let mut seen: HashSet<String> = libraries
        .iter()
        .filter_map(|value| value.as_str().map(str::to_owned))
        .collect();

    for path in library_paths {
        if !seen.contains(&path) {
            seen.insert(path.clone());
            libraries.push(Value::String(path));
        }
    }
}
