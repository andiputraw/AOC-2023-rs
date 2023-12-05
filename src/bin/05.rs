use std::{
    collections::btree_map::Range,
    panic::Location,
    sync::{mpsc, Arc, Mutex},
    thread, vec,
};

use regex::Regex;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Map {
    source: u32,
    destination: u32,
    range: u32,
}

impl Map {
    fn new(src: u32, dest: u32, range: u32) -> Self {
        Self {
            source: src,
            destination: dest,
            range,
        }
    }
}

#[derive(Debug)]
struct MapPipe {
    map: Vec<Map>,
}

impl MapPipe {
    fn new() -> Self {
        Self { map: Vec::new() }
    }

    fn convert(self: &Self, source: u32) -> u32 {
        let mut result = source;
        for map in self.map.iter() {
            if source < map.source || source > map.source + (map.range - 1) {
                continue;
            } else {
                result = map.destination + map.source.abs_diff(source)
            }
        }
        result
    }
}

struct PipeFactory {
    digit_re: Regex,
}

impl PipeFactory {
    fn new() -> Self {
        PipeFactory {
            digit_re: Regex::new(r"\d+").unwrap(),
        }
    }
    fn get_seed(self: &Self, re: Regex, input: &str) -> Vec<u32> {
        let result = re.find(input).unwrap().as_str();
        let mut map = Vec::new();
        for i in self.digit_re.find_iter(result).map(|c| c.as_str()) {
            map.push(i.parse::<u32>().unwrap());
        }

        map
    }

    fn get_seed_range(self: &Self, re: Regex, input: &str) -> Vec<(u32, u32)> {
        let result = re.find(input).unwrap().as_str();
        let mut map = Vec::new();
        for i in self.digit_re.find_iter(result).map(|c| c.as_str()) {
            map.push(i.parse::<u32>().unwrap());
        }

        let mut seed_range: Vec<(u32, u32)> = Vec::new();
        let mut i = 0;
        while i < map.len() {
            seed_range.push((map.get(i).unwrap().clone(), map.get(i + 1).unwrap().clone()));
            i += 2;
        }

        seed_range
    }
    fn create_pipe(self: &Self, re: Regex, input: &str) -> MapPipe {
        println!("{}", re);
        let result = re.find(input).unwrap().as_str();
        let mut map = Vec::new();
        for i in self.digit_re.find_iter(result).map(|c| c.as_str()) {
            map.push(i.parse::<u32>().unwrap());
        }

        let mut map_pipe = MapPipe::new();
        let mut i = 0;
        while i < map.len() {
            let map = Map::new(
                *map.get(i + 1).unwrap(),
                *map.get(i).unwrap(),
                *map.get(i + 2).unwrap(),
            );
            map_pipe.map.push(map);
            i += 3;
        }

        map_pipe
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let seed = Regex::new(r"seeds:\s*([\d\s]+)").unwrap();
    let seed_to_soil = Regex::new(r"seed-to-soil map:\s*([\d\s]+)").unwrap();
    let soil_to_fetlizer = Regex::new(r"soil-to-fertilizer map:\s*([\d\s]+)").unwrap();
    let fertilizer_to_water = Regex::new(r"fertilizer-to-water map:\s*([\d\s]+)").unwrap();
    let water_to_light = Regex::new(r"water-to-light map:\s*([\d\s]+)").unwrap();
    let light_to_temperature = Regex::new(r"light-to-temperature map:\s*([\d\s]+)").unwrap();
    let temperature_to_humidity = Regex::new(r"temperature-to-humidity map:\s*([\d\s]+)").unwrap();
    let humidity_to_location = Regex::new(r"humidity-to-location map:\s*([\d\s]+)").unwrap();

    let mut pipes_regex = Vec::new();
    let factory = PipeFactory::new();
    let seeds = factory.get_seed(seed, input);

    pipes_regex.push(seed_to_soil);
    pipes_regex.push(soil_to_fetlizer);
    pipes_regex.push(fertilizer_to_water);
    pipes_regex.push(water_to_light);
    pipes_regex.push(light_to_temperature);
    pipes_regex.push(temperature_to_humidity);
    pipes_regex.push(humidity_to_location);

    let pipes = pipes_regex
        .into_iter()
        .map(|v| factory.create_pipe(v, input))
        .collect::<Vec<MapPipe>>();

    let mut location = Vec::new();
    for seed in seeds.iter() {
        let mut result = seed.clone();
        for pipe in pipes.iter() {
            result = pipe.convert(result);
        }
        location.push(result);
    }

    println!("{}", location.iter().min().unwrap());

    None
}

struct WaitGroup {
    counter: Mutex<usize>,
    condvar: std::sync::Condvar,
}

impl WaitGroup {
    fn new() -> Self {
        WaitGroup {
            counter: Mutex::new(0),
            condvar: std::sync::Condvar::new(),
        }
    }

    fn add(&self, count: usize) {
        let mut counter = self.counter.lock().unwrap();
        *counter += count;
    }

    fn done(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter -= 1;

        // Notify waiting threads when counter reaches zero
        if *counter == 0 {
            self.condvar.notify_all();
        }
    }

    fn wait(&self) {
        let mut counter = self.counter.lock().unwrap();
        while *counter > 0 {
            counter = self.condvar.wait(counter).unwrap();
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let seed = Regex::new(r"seeds:\s*([\d\s]+)").unwrap();
    let seed_to_soil = Regex::new(r"seed-to-soil map:\s*([\d\s]+)").unwrap();
    let soil_to_fetlizer = Regex::new(r"soil-to-fertilizer map:\s*([\d\s]+)").unwrap();
    let fertilizer_to_water = Regex::new(r"fertilizer-to-water map:\s*([\d\s]+)").unwrap();
    let water_to_light = Regex::new(r"water-to-light map:\s*([\d\s]+)").unwrap();
    let light_to_temperature = Regex::new(r"light-to-temperature map:\s*([\d\s]+)").unwrap();
    let temperature_to_humidity = Regex::new(r"temperature-to-humidity map:\s*([\d\s]+)").unwrap();
    let humidity_to_location = Regex::new(r"humidity-to-location map:\s*([\d\s]+)").unwrap();

    let mut pipes_regex = Vec::new();
    let factory = PipeFactory::new();
    let seeds = factory.get_seed_range(seed, input);

    pipes_regex.push(seed_to_soil);
    pipes_regex.push(soil_to_fetlizer);
    pipes_regex.push(fertilizer_to_water);
    pipes_regex.push(water_to_light);
    pipes_regex.push(light_to_temperature);
    pipes_regex.push(temperature_to_humidity);
    pipes_regex.push(humidity_to_location);

    let pipes = pipes_regex
        .into_iter()
        .map(|v| factory.create_pipe(v, input))
        .collect::<Vec<MapPipe>>();

    let pipes = Arc::new(pipes);

    let mut location = Arc::new(Mutex::new(Vec::new()));
    let (sender, reciever) = mpsc::channel();
    let wait_group = Arc::new(WaitGroup::new());
    let loop_iteration = seeds.len();
    for (start, range) in seeds.into_iter() {
        let wait_group_clone = Arc::clone(&wait_group);
        let pipes = pipes.clone();
        let sender = sender.clone();
        thread::spawn(move || {
            let mut counter = 0;
            for i in start..=(start + (range - 1)) {
                let mut result = i;
                for pipe in pipes.iter() {
                    result = pipe.convert(result);
                }
                if counter % 1_000_000 == 0 {
                    println!("{} of {} loops", counter, start + (range - 1))
                }
                counter += 1;
                sender.send(result).unwrap();
            }
            wait_group_clone.done();
        });
    }

    let location_target = location.clone();

    thread::spawn(move || {
        for recieved in reciever {
            let mut data = location_target.lock().unwrap();
            data.push(recieved);
        }
    });

    wait_group.add(loop_iteration);

    wait_group.wait();
    println!("{}", location.lock().unwrap().iter().min().unwrap());

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
