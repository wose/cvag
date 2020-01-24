use cvag_client::Client;

fn main() {
    let mut client = Client::new().unwrap();

    let station = 343;
    let stop_list = client.stops(station).unwrap();
    println!("Next Stops for station {}:", station);
    for stop in stop_list.stops {
        println!("{:?}", stop);
    }
}
