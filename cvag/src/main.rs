use chrono::Timelike;
use clap::{values_t, App, Arg};
use termion::{color, style};

use cvag_client::{Client, ServiceType};

fn main() {
    let matches = App::new("cvag")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("CVAG bus and tram schedule client.")
        .max_term_width(100)
        .arg(
            Arg::with_name("station")
                .short("s")
                .long("station")
                .help("CVAG station id")
                .value_name("ID")
                .required(true)
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    if let Ok(mut stations) = values_t!(matches.values_of("station"), u64) {
        stations.sort_unstable();
        stations.dedup();

        let mut client = Client::new().unwrap();
        let now = chrono::Local::now();
        let station_data = client.stations(None).unwrap();

        for station in stations {
            let name = &station_data
                .iter()
                .find(|s| s.number == station)
                .map_or("", |s| &s.display_name);
            println!("{} ({}):", name, station);
            let stop_list = client.stops(station).unwrap();
            for stop in stop_list.stops {
                let departure = stop.actual_departure.with_timezone(&chrono::Local);
                let diff = departure - now;

                match stop.service_type {
                    ServiceType::Bus => {
                        print!("  {}██{}", color::Fg(color::LightMagenta), style::Reset)
                    }
                    ServiceType::Tram => print!("  {}██{}", color::Fg(color::Red), style::Reset),
                    ServiceType::ChemnitzBahn => {
                        print!("  {}██{}", color::Fg(color::LightGreen), style::Reset)
                    }
                    ServiceType::SchienenErsatzVerkehr => {
                        print!("  {}██{}", color::Fg(color::Yellow), style::Reset)
                    }
                }

                print!(
                    " {:<4} {:>4} | {:02}:{:02} | in ",
                    stop.service_type,
                    stop.line,
                    departure.hour(),
                    departure.minute(),
                );

                if stop.has_actual_departure {
                    print!(
                        "{}{:>2}{}",
                        color::Fg(color::Green),
                        diff.num_minutes(),
                        style::Reset
                    );
                } else {
                    print!("{:>3}", diff.num_minutes());
                }

                println!(" min | {}", stop.destination);
            }
            println!();
        }
    }
}
