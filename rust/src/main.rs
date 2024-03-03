use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::thread;
use std::time::Instant;

const ONE_BILLION: usize = 1_000_000_000;

struct Station {
    n: u32,
    sum: f32,
    min: f32,
    max: f32,
}

fn main() {
    for number_of_threads in 1..16 {
        let start_time = Instant::now();
        one_billion(number_of_threads);
        let end_time = Instant::now();
        println!("");
        println!("");
        println!(
            "{} Threads with Elapsed time: {:?}",
            number_of_threads,
            end_time - start_time
        );
        println!("");
    }
}

fn one_billion(number_of_threads: usize) {
    let partition_size = ONE_BILLION / number_of_threads;
    let mut thread_handles = Vec::new();
    for partition_index in 0..number_of_threads {
        let handle = thread::spawn(move || {
            let lines = read_lines("../measurements.data")
                .unwrap()
                .map_while(Result::ok)
                .skip(partition_index * partition_size)
                .take(partition_size);

            let mut stations: HashMap<String, Station> = HashMap::new();

            for line in lines {
                let mut line_parts = line.split(';');
                let station_name = line_parts.next().unwrap().to_string();
                let measurement: f32 = line_parts.next().unwrap().parse().unwrap();
                insert_into_hashmap(&mut stations, station_name, measurement);
            }

            return stations;
        });
        thread_handles.push(handle);
    }

    let mut final_stations: HashMap<String, Station> = HashMap::new();
    for thread in thread_handles {
        let stations = thread.join().unwrap();
        for (station_name, station) in stations.into_iter() {
            match final_stations.get_mut(&station_name) {
                Some(final_station) => {
                    final_station.n += station.n;
                    final_station.sum += station.sum;
                    if station.min < final_station.min {
                        final_station.min = station.min;
                    }
                    if station.max > final_station.max {
                        final_station.max = station.max;
                    }
                }
                None => {
                    final_stations.insert(
                        station_name,
                        Station {
                            n: station.n,
                            sum: station.sum,
                            min: station.min,
                            max: station.max,
                        },
                    );
                }
            }
        }
    }

    print!("{{");
    let mut station_names = final_stations.keys().collect::<Vec<&String>>();
    station_names.sort();
    for (station_name, should_add_comma) in station_names
        .iter()
        .enumerate()
        .map(|(i, s)| (*s, i != station_names.len() - 1))
    {
        if let Some(station) = final_stations.get(station_name) {
            print!(
                "{}={:.1?}/{:.1?}/{:.1?}",
                station_name,
                station.min,
                station.sum / station.n as f32,
                station.max
            );
            if should_add_comma {
                print!(", ")
            }
        }
    }
    print!("}}");
}

fn insert_into_hashmap(
    stations: &mut HashMap<String, Station>,
    station_name: String,
    measurement: f32,
) {
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
        }
        None => {
            stations.insert(
                station_name,
                Station {
                    n: 1,
                    sum: measurement,
                    min: measurement,
                    max: measurement,
                },
            );
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
