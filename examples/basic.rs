use serde::Deserialize;
use serde::Serialize;
use tomlua::TomluaExecute;
use tomlua::tomlua_config;

#[tomlua_config]
#[derive(Serialize, Deserialize, Debug)]
struct MyConfig {
    name: String,
    toggle: bool,
}

fn main() {
    let cfg_str = std::fs::read_to_string("./examples/example_config.toml")
        .expect("Failed to read example_config.toml");
    let mut cfg: MyConfig = toml::from_str(&cfg_str).expect("Failed to parse example_config.toml");
    cfg.execute_script("basic_script", None)
        .expect("Script failed!");
}
