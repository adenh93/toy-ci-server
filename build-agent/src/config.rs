use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LogLevel {
    Minimum,
    Normal,
    Verbose,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Normal
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Cache {
    pub name: Option<String>,
    pub path: PathBuf,
    pub key: String,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub logging: LogLevel,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Step {
    pub name: String,
    pub run: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StepFormat {
    Full(Step),
    Minimal(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub title: String,
    pub environment: Option<HashMap<String, String>>,
    pub caches: Option<Vec<Cache>>,
    #[serde(default)]
    pub settings: Settings,
    pub steps: Vec<StepFormat>,
}
