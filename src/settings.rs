use serde::Deserialize;
use zed_extension_api::serde_json::{Map, Value};
use zed_extension_api::Result;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Settings {
    #[serde(rename = "Lua")]
    pub lua: Map<String, Value>,
    pub gmod: GmodSettings,
    pub binary: BinarySettings,
    pub library: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            lua: Map::new(),
            gmod: GmodSettings::default(),
            binary: BinarySettings::default(),
            library: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct GmodSettings {
    pub enabled: bool,
    pub download_library: bool,
    pub refresh_library: bool,
    pub library_path: Option<String>,
}

impl Default for GmodSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            download_library: true,
            refresh_library: true,
            library_path: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct BinarySettings {
    pub ignore_system_version: bool,
    pub path: Option<String>,
    pub args: Vec<String>,
}

impl Default for BinarySettings {
    fn default() -> Self {
        Self {
            ignore_system_version: false,
            path: None,
            args: Vec::new(),
        }
    }
}

pub fn get_extension_settings(settings_val: Option<Value>) -> Result<Settings> {
    let Some(mut settings_val) = settings_val else {
        return Ok(Settings::default());
    };

    let Some(settings) = settings_val.as_object_mut() else {
        return Err("invalid lua-language-server settings: `settings` must be an object".into());
    };

    let lua_settings = settings.remove("Lua");
    let mut value = settings.remove("ext").unwrap_or(settings_val);
    if let Value::Object(o) = &mut value {
        o.insert(
            "Lua".to_string(),
            lua_settings.unwrap_or(Value::Object(Map::new())),
        );
    }

    serde_path_to_error::deserialize(value).map_err(|e| e.to_string())
}
