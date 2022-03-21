use std::collections::HashMap;

use clap::ArgMatches;
use config::{Config, ConfigError, Environment, File, Source, Value};
use serde::{Deserialize, Serialize};

use {{ artifact_id }}_persistence::settings::PersistenceSettings;
use {{ artifact_id }}_server::settings::ServerSettings;

const DEFAULT_CONFIG_FILE: &str = "etc/{{ prefix_name }}-service";
const DEFAULT_ENVIRONMENT_PREFIX: &str = "SERVICE";

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    server: ServerSettings,
    persistence: PersistenceSettings,
}

impl Settings {
    pub fn server(&self) -> &ServerSettings {
        &self.server
    }

    pub fn persistence(&self) -> &PersistenceSettings {
        &self.persistence
    }

    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            server: Default::default(),
            persistence: Default::default(),
        }
    }
}

impl Settings {
    pub fn new(args: &ArgMatches) -> Result<Self, ConfigError> {
        let config = Config::builder();

        // Defaults
        let config = config.add_source(File::from_str(
            Settings::default()
                .to_yaml()
                .map_err(|err| ConfigError::Foreign(Box::new(err)))?
                .as_str(),
            config::FileFormat::Yaml,
        ));

        let config = config.add_source(File::with_name(DEFAULT_CONFIG_FILE).required(false));

        let config = config.add_source(Environment::with_prefix(DEFAULT_ENVIRONMENT_PREFIX).separator("_"));

        // Merge in a config file specified on the command line
        let config = if let Some(config_file) = args.value_of("config-file") {
            if let Ok(config_file) = shellexpand::full(config_file) {
                let config = config.add_source(File::with_name(config_file.as_ref()).required(true));
                config
            } else {
                config
            }
        } else {
            config
        };

        // Merge in command line overrides
        let mut mappings = HashMap::new();
        mappings.insert("service-port".into(), "server.service.port".into());
        // mappings.insert("management-port".into(), "server.management.port".into());
        mappings.insert("host".into(), "server.host".into());
        mappings.insert("temp-db".into(), "persistence.temporary".into());
        // mappings.insert("database-url".into(), "persistence.database.url".into());
        let config = config.add_source(Clap::new(args.clone(), mappings));

        let conf = config.build()?;

        conf.try_deserialize()
    }
}

#[derive(Clone, Debug)]
struct Clap {
    keys: HashMap<String, String>,
    matches: ArgMatches,
}

impl Clap {
    pub fn new(matches: ArgMatches, keys: HashMap<String, String>) -> Clap {
        Clap {
            keys,
            matches: matches.clone(),
        }
    }
}

impl Source for Clap {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>, ConfigError> {
        let mut results = HashMap::new();
        for (key, mapped) in &self.keys {
            if let Some(value) = self.matches.value_of(key) {
                results.insert(mapped.into(), value.into());
            }
        }
        Ok(results)
    }
}