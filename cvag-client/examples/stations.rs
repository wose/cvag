use cvag_client::Client;

fn main() {
    let mut client = Client::new().unwrap();

    println!("All stations:");
    let stations = client.stations(None).unwrap();
    for station in stations {
        println!("    {} - {}", station.number, station.display_name);
    }

    println!("Stations with name like \"Rudo\":");
    let stations = client.stations(Some("Rudo")).unwrap();
    for station in stations {
        println!("    {} - {}", station.number, station.display_name);
    }
}
