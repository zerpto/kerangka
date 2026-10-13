use besi::ConfigSection;
use serde::Deserialize;

/// `hello:` in config.yaml, or `HELLO__GREETING` in .env / the environment.
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HelloConfig {
    pub greeting: String,
}

impl Default for HelloConfig {
    fn default() -> Self {
        Self {
            greeting: "Hello".into(),
        }
    }
}

impl ConfigSection for HelloConfig {
    const KEY: &'static str = "hello";
}
