use restson::{Error, RestClient};

use crate::{Station, StationList, StopList};

pub struct Client {
    client: RestClient,
}

impl Client {
    pub fn new() -> Result<Self, Error> {
        let client = RestClient::new("https://www.cvag.de/eza/mis/")?;
        Ok(Client { client })
    }

    pub fn stations(&mut self, filter: Option<&str>) -> Result<Vec<Station>, Error> {
        let params = if let Some(filter) = filter {
            vec![("like", filter)]
        } else {
            vec![
                ("minLat", "0"),
                ("maxLat", "100"),
                ("minLon", "0"),
                ("maxLon", "100"),
            ]
        };

        self.client
            .get_with((), &params)
            .and_then(|station_list: StationList| Ok(station_list.stations))
    }

    pub fn stops(&mut self, station: u64) -> Result<StopList, Error> {
        self.client.get(station)
    }
}
