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

fn main() -> anyhow::Result<()> {
    let cfg_str = std::fs::read_to_string("./examples/example_config.toml")?;
    let cfg: MyConfig = toml::from_str(&cfg_str)?;
    cfg.execute_script("faulty")?;
    let cfg_str = toml::to_string(&cfg)?;
    std::fs::write("./examples/example_config.toml", cfg_str)?;
    Ok(())
}
