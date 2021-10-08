use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(rename = "mapping")]
    pub lps: Vec<LpConfig>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LpConfig{
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "hostport")]
    pub hostport: String

}