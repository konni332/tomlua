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

    cfg.update(cfg.execute_script("test_script").expect("Script failed"))
        .expect("Failed to update");

    let cfg_str = toml::to_string_pretty(&cfg).expect("Failed to serialize `MyConfig` struct");
    std::fs::write("./examples/example_config.toml", cfg_str)
        .expect("Failed to update example_config.toml");
}
