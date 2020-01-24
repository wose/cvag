use restson::{Error, RestPath};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct StationList {
    pub stations: Vec<Station>,
}

impl RestPath<()> for StationList {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("/eza/mis/stations"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub display_name: String,
    pub mandator: String,
    pub number: u64,
}
