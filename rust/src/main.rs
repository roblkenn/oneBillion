use std::fs::File;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;

const BUFFER_SIZE: usize = 256 * 1000 * 1000;

struct Station {
    n: u32,
    sum: f32,
    min: f32,
    max: f32
}

fn main() {    
    let mut stations: HashMap<String, Station> = HashMap::new();
    let lines = BufReader::with_capacity(BUFFER_SIZE, File::open("../measurements.data").unwrap()).lines().map_while(Result::ok);

    for line in lines {
        let mut line_split = line.split(';');
        let station_name = line_split.next().unwrap().to_string();
        let measurement: f32 = line_split.next().unwrap().parse().unwrap();
        match stations.get_mut(&station_name) {
            Some(station) => {
                station.n += 1;
                station.sum += measurement;
                if measurement < station.min {
                    station.min = measurement;
                }
                if measurement > station.max {
                    station.max = measurement;
                }
            },
            None => {
                stations.insert(station_name, Station {
                    n: 1,
                    sum: measurement,
                    min: measurement,
                    max: measurement
                });
            }
        }
    }

    print!("{{");
    let mut station_names = stations.keys().collect::<Vec<&String>>();
    station_names.sort();
    for (station_name, should_add_comma) in station_names.iter().enumerate().map(|(i, s)| (*s, i != station_names.len() - 1)) {
        if let Some(station) = stations.get(station_name) {
            print!("{}={:.1?}/{:.1?}/{:.1?}", station_name, station.min, station.sum / station.n as f32, station.max);
            if should_add_comma {
                print!(", ")
            }
        }
    }
    print!("}}");
}
