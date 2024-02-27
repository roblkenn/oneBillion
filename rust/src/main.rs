use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

struct Station {
    n: u32,
    sum: f32,
    min: f32,
    max: f32
}

fn main() {    
    let mut stations: HashMap<String, Station> = HashMap::new();

    if let Ok(lines) = read_lines("../measurements.data") {
        for line in lines.flatten() {
            let parts = line.split(";").collect::<Vec<&str>>();
            let station_name = parts[0].to_string();
            let measurement: f32 = parts[1].parse().unwrap();
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
    }
    print_stations(&stations);
}

fn print_stations(stations: &HashMap<String, Station>) {
    print!("{{");
    let mut station_names = stations.keys().collect::<Vec<&String>>();
    station_names.sort();
    for (station_name, should_add_comma) in station_names.iter().enumerate().map(|(i, s)| (s, i != station_names.len() - 1)) {
        match stations.get(*station_name) {
            Some(station) => {
                let mean = station.sum / station.n as f32;
                print!("{}={:.1?}/{:.1?}/{:.1?}", station_name, station.min, mean, station.max);
                if should_add_comma {
                    print!(", ")
                }
            },
            None => {}
        }
    }
    print!("}}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}