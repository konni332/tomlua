#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Script '{script}' failed: {error}")]
    LuaRunTimeError { error: String, script: String },

    #[error("Empty script error: neither inline nor path found for script '{0}'")]
    EmptyScript(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid script path: {0}")]
    InvalidPath(String),

    #[error("Invalid inline lua in '{script}': {error}")]
    InvalidInline { error: String, script: String },

    #[error("Lua error: {0}")]
    LuaError(String),
}

impl From<mlua::Error> for Error {
    fn from(value: mlua::Error) -> Self {
        Self::LuaError(value.to_string())
    }
}

pub fn lua_error_message(e: &mlua::Error) -> String {
    match e {
        mlua::Error::SyntaxError { message, .. } => message
            .splitn(2, ": ")
            .nth(1)
            .unwrap_or(message)
            .to_string(),
        mlua::Error::RuntimeError(msg) => msg.clone(),
        mlua::Error::MemoryError(msg) => msg.clone(),
        mlua::Error::CallbackError { cause, .. } => lua_error_message(cause),
        _ => e.to_string(),
    }
}
