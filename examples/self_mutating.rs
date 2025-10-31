use serde::Deserialize;
use serde::Serialize;
use tomlua::TomluaExecute;
use tomlua::tomlua_config;

#[tomlua_config]
#[derive(Serialize, Deserialize)]
struct MyConfig {
    name: String,
    toggle: bool,
}

fn main() {
    let cfg_str = std::fs::read_to_string("./examples/example_config.toml")
        .expect("Failed to read example_config.toml");
    let mut cfg: MyConfig = toml::from_str(&cfg_str).expect("Failed to parse example_config.toml");
    cfg.execute_script("self_mutating", Some("update"))
        .expect("Script failed!");
    let cfg_str = toml::to_string(&cfg).expect("Failed to serialize toml");
    std::fs::write("./examples/example_config.toml", cfg_str).expect("Failed to save updated Toml");
}
