pub mod config {
	pub use ::config::ConfigError;
	use serde::Deserialize;
	use std::{path::PathBuf};

	#[derive (Deserialize)]
	pub struct Config {
		pub node_addr: String,
		pub priv_key: PathBuf,
	}

	impl Config {
		// Pull things from .env
		pub fn config_from_env() -> Result<Self, ConfigError> {
			let mut cfg = ::config::Config::new();
			cfg.merge(::config::Environment::new())?;
			cfg.try_into()
		}

		// Put ya own stuff in here
		pub fn config_blank() -> Result<Self, ConfigError> {
			let mut cfg = ::config::Config::new();
			cfg.try_into()
		}
	}
}
