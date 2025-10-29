use std::path::PathBuf;

use mlua::Value;
use serde::{Deserialize, Serialize};

// reexports
pub use tomlua_macros::TomluaExecute;
pub use tomlua_macros::tomlua_config;

// scripts
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Script {
    name: String,
    path: Option<PathBuf>,
    inline: Option<String>,
}

impl Script {
    pub fn new() -> Self {
        Script {
            name: "".into(),
            path: None,
            inline: Some("".into()),
        }
    }
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }
    pub fn inline(&self) -> Option<&String> {
        self.inline.as_ref()
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl mlua::prelude::LuaUserData for Script {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        // getter
        methods.add_method("name", |_, this, ()| Ok(this.name.clone()));
        methods.add_method("path", |_, this, ()| {
            Ok(this.path().map(|p| p.display().to_string()))
        });
        methods.add_method("inline", |_, this, ()| {
            Ok(this.inline().map(|s| s.to_string()))
        });

        // setter
        methods.add_method_mut("set_name", |_, this, name: String| {
            this.name = name;
            Ok(())
        });
        methods.add_method_mut("set_path", |_, this, path: Option<String>| {
            this.path = match path {
                Some(p) => Some(PathBuf::from(p)),
                None => None,
            };
            Ok(())
        });
        methods.add_method_mut("set_inline", |_, this, inline: Option<String>| {
            this.inline = inline;
            Ok(())
        });
    }
}

impl mlua::prelude::FromLua for Script {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            Value::Table(table) => {
                let name: String = table.get("name")?;
                let path: Option<String> = table.get("path").unwrap_or(None);
                let path: Option<PathBuf> = match path {
                    Some(p) => Some(PathBuf::from(p)),
                    None => None,
                };
                let inline: Option<String> = table.get("inline").unwrap_or(None);
                Ok(Script { name, path, inline })
            }
            Value::UserData(ud) => {
                let script: Script = ud.borrow::<Script>()?.clone();
                Ok(script)
            }
            other => Err(mlua::Error::FromLuaConversionError {
                from: other.type_name(),
                to: "Script".into(),
                message: Some("expected table or userdata".into()),
            }),
        }
    }
}
