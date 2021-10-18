use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(rename = "lps")]
    pub lps: Vec<LpConfig>,

    #[serde(rename = "sb_url")]
    pub sb_url: String,

    #[serde(rename = "no_sql_url")]
    pub no_sql_url: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LpConfig{
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "hostport")]
    pub hostport: String

}