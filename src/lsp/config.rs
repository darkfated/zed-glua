use crate::util::json;
use zed_extension_api::serde_json::{Map, Value};

const NONSTANDARD_SYMBOLS: &[&str] = &["!=", "&&", "||", "//", "/**/", "continue", "!"];

const GMOD_DIAGNOSTIC_DISABLES: &[&str] = &[
    "duplicate-set-field",
    "lowercase-global",
];

const GMOD_DIAGNOSTIC_WARNINGS: &[&str] = &[
    "undefined-field", "undefined-global", "unused-local", "unused-function",
    "redundant-parameter", "cast-local-type", "param-type-mismatch",
    "return-type-mismatch",
];

fn push_unique(array: &mut Vec<Value>, value: &str) {
    if !array.iter().any(|item| item.as_str() == Some(value)) {
        array.push(Value::String(value.into()));
    }
}

fn apply_runtime_config(lua: &mut Map<String, Value>) {
    let runtime = json::get_or_insert_object(lua, "runtime");
    runtime.entry("version".to_string()).or_insert_with(|| Value::String("LuaJIT".into()));
    runtime.entry("unicodeName".to_string()).or_insert_with(|| Value::Bool(true));

    let special = json::get_or_insert_object(runtime, "special");
    special.entry("include".to_string()).or_insert_with(|| Value::String("dofile".into()));
    special.entry("IncludeCS".to_string()).or_insert_with(|| Value::String("dofile".into()));

    let symbols = json::get_or_insert_array(runtime, "nonstandardSymbol");
    for symbol in NONSTANDARD_SYMBOLS {
        push_unique(symbols, symbol);
    }
}

fn apply_diagnostics_config(lua: &mut Map<String, Value>) {
    let diagnostics = json::get_or_insert_object(lua, "diagnostics");

    let disabled = json::get_or_insert_array(diagnostics, "disable");
    for diag in GMOD_DIAGNOSTIC_DISABLES {
        push_unique(disabled, diag);
    }

    let severity = json::get_or_insert_object(diagnostics, "severity");
    for diag in GMOD_DIAGNOSTIC_WARNINGS {
        severity.entry(diag.to_string()).or_insert_with(|| Value::String("Warning".into()));
    }

    let group_severity = json::get_or_insert_object(diagnostics, "groupSeverity");
    group_severity.entry("undefined".to_string()).or_insert_with(|| Value::String("Warning".into()));
    group_severity.entry("unused".to_string()).or_insert_with(|| Value::String("Hint".into()));
}

fn apply_completion_config(lua: &mut Map<String, Value>) {
    let completion = json::get_or_insert_object(lua, "completion");
    completion.entry("autoRequire".to_string()).or_insert_with(|| Value::Bool(true));
    completion.entry("displayParameter".to_string()).or_insert_with(|| Value::Bool(true));
    completion.entry("callSnippet".to_string()).or_insert_with(|| Value::String("Replace".into()));
    completion.entry("showParams".to_string()).or_insert_with(|| Value::Bool(true));
    completion.entry("postfix".to_string()).or_insert_with(|| Value::String("@".into()));
}

fn apply_hover_config(lua: &mut Map<String, Value>) {
    let hover = json::get_or_insert_object(lua, "hover");
    hover.entry("viewString".to_string()).or_insert_with(|| Value::Bool(true));
    hover.entry("viewNumber".to_string()).or_insert_with(|| Value::Bool(true));
    hover.entry("fieldInHover".to_string()).or_insert_with(|| Value::Bool(true));
    hover.entry("preview".to_string()).or_insert_with(|| Value::Bool(true));
}

fn apply_hints_config(lua: &mut Map<String, Value>) {
    let hints = json::get_or_insert_object(lua, "hints");
    hints.entry("awaitArgType".to_string()).or_insert_with(|| Value::Bool(true));
    hints.entry("paramName".to_string()).or_insert_with(|| Value::Bool(true));
    hints.entry("paramType".to_string()).or_insert_with(|| Value::Bool(true));
    hints.entry("returnType".to_string()).or_insert_with(|| Value::Bool(true));
    hints.entry("enumEnumValues".to_string()).or_insert_with(|| Value::Bool(true));
}

fn apply_misc_config(lua: &mut Map<String, Value>) {
    let semantic = json::get_or_insert_object(lua, "semantic");
    semantic.entry("enable".to_string()).or_insert_with(|| Value::Bool(true));

    let format = json::get_or_insert_object(lua, "format");
    format.entry("enable".to_string()).or_insert_with(|| Value::Bool(true));

    let signature = json::get_or_insert_object(lua, "signatureHelp");
    signature.entry("enable".to_string()).or_insert_with(|| Value::Bool(true));

    let workspace = json::get_or_insert_object(lua, "workspace");
    workspace.entry("checkThirdParty".to_string()).or_insert_with(|| Value::Bool(false));
    workspace.entry("library".to_string()).or_insert_with(|| Value::Array(Vec::new()));
}

pub fn apply_gmod_lua_defaults(lua: &mut Map<String, Value>) {
    apply_runtime_config(lua);
    apply_diagnostics_config(lua);
    apply_completion_config(lua);
    apply_hover_config(lua);
    apply_hints_config(lua);
    apply_misc_config(lua);
}

pub fn merge_library_paths(
    lua: &mut Map<String, Value>,
    library_paths: impl IntoIterator<Item = String>,
) {
    let workspace = json::get_or_insert_object(lua, "workspace");
    let libraries = json::get_or_insert_array(workspace, "library");

    let mut seen: std::collections::HashSet<String> = libraries
        .iter()
        .filter_map(|value| value.as_str().map(str::to_owned))
        .collect();

    for path in library_paths {
        if seen.insert(path.clone()) {
            libraries.push(Value::String(path));
        }
    }
}
