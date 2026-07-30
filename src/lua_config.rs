use crate::json;
use std::collections::HashSet;
use zed_extension_api::serde_json::{Map, Value};

const NONSTANDARD_SYMBOLS: [&str; 7] = ["!", "!=", "&&", "||", "//", "/**/", "continue"];

fn push_unique(array: &mut Vec<Value>, value: &str) {
    if !array.iter().any(|item| item.as_str() == Some(value)) {
        array.push(Value::String(value.into()));
    }
}

pub fn apply_gmod_lua_defaults(lua: &mut Map<String, Value>) {
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
        push_unique(symbols, symbol);
    }

    let diagnostics = json::get_or_insert_object(lua, "diagnostics");
    let disabled = json::get_or_insert_array(diagnostics, "disable");
    push_unique(disabled, "duplicate-set-field");
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
        if seen.insert(path.clone()) {
            libraries.push(Value::String(path));
        }
    }
}
