use config::{Config as LibConfig, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    mode: String,
    environment: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let mode = env::var("PROFILE").unwrap_or_else(|_| "development".into());

        let config = LibConfig::builder()
            .add_source(
                File::with_name("../config/config")
            )
            .add_source(
                File::with_name(
                    &format!("../config/{}.yaml", mode)
                ).required(false)
            )
            .add_source(
                Environment::with_prefix("APP")
            )
            .build()?;


        config.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn should_load_default_config_when_profile_env_not_present() {
        env::remove_var("PROFILE");
        let config = Config::new().unwrap();
        assert_eq!(config.mode, "development");
        assert_eq!(config.environment, "local");
    }

    #[rstest]
    #[case("production")]
    #[case("staging")]
    fn should_load_config_from_profile_env(#[case] profile: &str) {
        env::remove_var("PROFILE");
        env::set_var("PROFILE", profile);
        let config = Config::new().unwrap();
        assert_eq!(config.mode, "deployed");
        assert_eq!(config.environment, profile);
    }
}