use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use restson::{Error, RestPath};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct StopList {
    pub stops: Vec<Stop>,
    #[serde(with = "ts_milliseconds")]
    pub now: DateTime<Utc>,
}

impl RestPath<u64> for StopList {
    fn get_path(id: u64) -> Result<String, Error> {
        Ok(format!("/eza/mis/stops/station/CAG-{}", id))
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    destination: String,
    service_type: String,
    has_actual_departure: bool,
    #[serde(with = "ts_milliseconds")]
    actual_departure: DateTime<Utc>,
    line: String,
    platform: Option<String>,
}
