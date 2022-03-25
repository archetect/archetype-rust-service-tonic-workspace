use std::collections::HashMap;

use clap::ArgMatches;
use config::{Config, ConfigError, Environment, File, Source, Value};
use serde::{Deserialize, Serialize};

use crate::traces::TraceFormat;
use {{ artifact_id }}_persistence::settings::PersistenceSettings;
use {{ artifact_id }}_server::settings::ServerSettings;

const DEFAULT_CONFIG_FILE: &str = "{{ artifact-id }}";
const DEFAULT_ENVIRONMENT_PREFIX: &str = "APPLICATION";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    server: ServerSettings,
    persistence: PersistenceSettings,
    tracing: TraceSettings,
}

impl Settings {
    pub fn server(&self) -> &ServerSettings {
        &self.server
    }

    pub fn persistence(&self) -> &PersistenceSettings {
        &self.persistence
    }

    pub fn persistence_mut(&mut self) -> &mut PersistenceSettings {
        &mut self.persistence
    }

    pub fn tracing(&self) -> &TraceSettings {
        &self.tracing
    }

    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}

impl Settings {
    pub fn new(args: &ArgMatches) -> Result<Self, ConfigError> {
        let config = Config::builder();

        // Load Defaults
        let config = config.add_source(File::from_str(
            Settings::default()
                .to_yaml()
                .map_err(|err| ConfigError::Foreign(Box::new(err)))?
                .as_str(),
            config::FileFormat::Yaml,
        ));

        // Merge Config File from Default Location
        let config = config.add_source(File::with_name(DEFAULT_CONFIG_FILE).required(false));

        // Merge Config File specified from Command Line
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

        // Merge Environment Variable Overrides
        let config = config.add_source(Environment::with_prefix(DEFAULT_ENVIRONMENT_PREFIX).separator("_"));

        // Merge Command Line overrides
        let mut mappings = HashMap::new();
        mappings.insert("service-port".into(), "server.service.port".into());
        // mappings.insert("management-port".into(), "server.management.port".into());
        mappings.insert("host".into(), "server.host".into());
        mappings.insert("temp-db".into(), "persistence.temporary".into());
        mappings.insert("migrate".into(), "persistence.migrate".into());
        mappings.insert("tracing-format".into(), "tracing.format".into());
        mappings.insert("tracing-filter".into(), "tracing.filter".into());
        mappings.insert("database-url".into(), "persistence.database.url".into());
        let config = config.add_source(Clap::new(args.clone(), mappings));

        let conf = config.build()?;

        conf.try_deserialize()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraceSettings {
    format: TraceFormat,
    filter: String,
}

impl TraceSettings {
    pub fn format(&self) -> &TraceFormat {
        &self.format
    }

    pub fn filter(&self) -> &str {
        self.filter.as_str()
    }
}

impl Default for TraceSettings {
    fn default() -> Self {
        TraceSettings {
            format: Default::default(),
            filter: "info".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
struct Clap {
    keys: HashMap<String, String>,
    matches: ArgMatches,
}

impl Clap {
    pub fn new(matches: ArgMatches, keys: HashMap<String, String>) -> Clap {
        Clap { keys, matches }
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
            } else if self.matches.is_present(key) {
                results.insert(mapped.into(), "true".into());
            }
        }
        Ok(results)
    }
}
