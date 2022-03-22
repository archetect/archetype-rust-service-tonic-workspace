#![allow(non_camel_case_types)]

use clap::{ArgEnum, PossibleValue};
use serde::{Deserialize, Serialize};

use crate::settings::TraceSettings;

#[derive(Copy, Clone, Debug, ArgEnum, Serialize, Deserialize)]
pub enum TraceFormat {
    standard,
    json,
    pretty,
}

impl TraceFormat {
    pub fn possible_values() -> impl Iterator<Item = PossibleValue<'static>> {
        TraceFormat::value_variants()
            .iter()
            .filter_map(ArgEnum::to_possible_value)
    }
}

impl Default for TraceFormat {
    fn default() -> Self {
        TraceFormat::standard
    }
}

pub fn init(settings: &TraceSettings) {
    match settings.format() {
        TraceFormat::standard => {
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::INFO)
                .init();
        }
        TraceFormat::json => {
            tracing_subscriber::fmt()
                .json()
                .with_max_level(tracing::Level::INFO)
                .init();
        }
        TraceFormat::pretty => {
            tracing_subscriber::fmt()
                .pretty()
                .with_max_level(tracing::Level::INFO)
                .init();
        }
    }
}