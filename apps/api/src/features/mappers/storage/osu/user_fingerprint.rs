use chrono::{DateTime, Utc};
use sea_orm::JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapperFingerprint {
    pub username: String,
    pub country: String,
    pub country_code: String,
    pub avatar_url: String,
    pub cover: JsonValue,
    pub primary_mode: String,
    pub mapping_followers: i32,
    pub kudosu: MapperKudosu,
    pub badges: JsonValue,
    pub groups: JsonValue,
    pub is_bng: bool,
    pub is_nat: bool,
    pub is_gmt: bool,
    pub is_probation_bn: bool,
    pub is_full_bn: bool,
    pub cached_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapperKudosu {
    pub total: i32,
    pub available: i32,
}

impl MapperFingerprint {
    pub fn from_raw(raw: &JsonValue) -> Option<Self> {
        serde_json::from_value(raw.clone()).ok()
    }
}
