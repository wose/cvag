use std::fmt;

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
#[serde(rename_all = "UPPERCASE")]
pub enum ServiceType {
    Bus,
    ChemnitzBahn,
    SchienenErsatzVerkehr,
    Tram,
}

impl fmt::Display for ServiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServiceType::Bus => write!(f, "BUS"),
            ServiceType::ChemnitzBahn => write!(f, "CB"),
            ServiceType::SchienenErsatzVerkehr => write!(f, "SEV"),
            ServiceType::Tram => write!(f, "TRAM"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    pub destination: String,
    pub service_type: ServiceType,
    pub has_actual_departure: bool,
    #[serde(with = "ts_milliseconds")]
    pub actual_departure: DateTime<Utc>,
    pub line: String,
    pub platform: Option<String>,
}
