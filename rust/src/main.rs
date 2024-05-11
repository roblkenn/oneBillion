use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::thread::{self, JoinHandle};
use std::time::Instant;

const ONE_BILLION: usize = 1_000_000_000;

struct Station {
    n: u32,
    sum: f32,
    min: f32,
    max: f32,
}
impl Station {
    fn new(measurement: f32) -> Self {
        Station {
            n: 1,
            sum: measurement,
            min: measurement,
            max: measurement,
        }
    }

    fn merge(&mut self, other: &Station) {
        self.n += other.n;
        self.sum += other.sum;
        if other.min < self.min {
            self.min = other.min;
        }
        if other.max > self.max {
            self.max = other.max;
        }
    }
}

fn main() {
    for number_of_threads in 1..16 {
        let start_time = Instant::now();
        one_billion(number_of_threads);
        let end_time = Instant::now();
        println!();
        println!();
        println!(
            "{} Threads with Elapsed time: {:?}",
            number_of_threads,
            end_time - start_time
        );
        println!();
    }
}

fn start_threads(number_of_threads: usize) -> Vec<JoinHandle<HashMap<String, Station>>> {
    let partition_size = ONE_BILLION / number_of_threads;
    let mut thread_handles = Vec::with_capacity(number_of_threads);
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
            stations
        });
        thread_handles.push(handle);
    }
    thread_handles
}

fn merge_results(threads: Vec<JoinHandle<HashMap<String, Station>>>) -> HashMap<String, Station> {
    let mut final_stations: HashMap<String, Station> = HashMap::new();
    for thread in threads {
        let stations = thread.join().unwrap();
        for (station_name, station) in stations.into_iter() {
            match final_stations.get_mut(&station_name) {
                Some(final_station) => {
                    final_station.merge(&station);
                }
                None => {
                    final_stations.insert(station_name, station);
                }
            }
        }
    }
    final_stations
}

fn print_results(stations: &HashMap<String, Station>) {
    print!("{{");
    let mut station_names = stations.keys().collect::<Vec<&String>>();
    station_names.sort();
    for (station_name, should_add_comma) in station_names
        .iter()
        .enumerate()
        .map(|(i, s)| (*s, i != station_names.len() - 1))
    {
        if let Some(station) = stations.get(station_name) {
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

fn one_billion(number_of_threads: usize) {
    let threads = start_threads(number_of_threads);
    let stations = merge_results(threads);
    print_results(&stations);
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
            stations.insert(station_name, Station::new(measurement));
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
