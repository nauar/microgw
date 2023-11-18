use regex::Regex;
use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer};
use serde::ser::{Serializer};
use std::{fs::File, io::Read};
use toml;

fn deserialize_regex<'de, D>(deserializer: D) -> Result<Option<Regex>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Regex::new(&s).map_err(de::Error::custom).map(Some)
}

fn serialize_regex<S>(regex: &Option<Regex>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match regex {
        None => serializer.serialize_none(),
        Some(regex) => serializer.serialize_str(regex.as_str()),
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServiceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_regex", serialize_with = "serialize_regex")]
    pub host: Option<Regex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_regex", serialize_with = "serialize_regex")]
    pub path: Option<Regex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Header>,
    pub target_service: String,
    pub target_port: String,
    pub authentication_required: Option<bool>,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GatewayConfig {
    pub authorization_api_url: String,
    pub services: Vec<ServiceConfig>,
}

pub fn load_config(path: &str) -> GatewayConfig {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    toml::from_str(&contents).unwrap()
}
