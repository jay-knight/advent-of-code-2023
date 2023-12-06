use std::fs::File;
use std::cmp;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct MapRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl MapRange {
    fn new(destination_start: u64, source_start: u64, length: u64) -> Self {
        Self {
            destination_start,
            source_start,
            length,
        }
    }

    fn translate(&self, value: u64) -> Option<u64> {
        if value >= self.source_start && value < self.source_start + self.length {
            Some(self.destination_start + (value - self.source_start))
        } else {
            None
        }
    }
}

type Ranges = Vec<MapRange>;
type Map = Vec<Ranges>;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut seeds: Vec<u64> = Vec::new();
        let mut n = 0;
        let mut maps: Map = Vec::new();
        let mut ranges: Ranges = Vec::new(); // the first list of maps
        for line in lines {
            if let Ok(line_str) = line {
                // Get seeds from first line
                if n == 0 {
                    seeds =  line_str
                        .split(" ")
                        .filter(|s| *s != "seeds:")
                        .map(|s| s.parse::<u64>().expect("Could not parse number"))
                        .collect();
                } else if line_str == "" {
                    if ranges.len() > 0 {
                        //println!("Line {}, adding new vec of maps {:?}", n, ranges);
                        maps.push(ranges);
                        ranges = Ranges::new();
                    }
                } else {
                    let parts: Vec<&str> = line_str.split(" ").collect();
                    if parts.len() == 3 { // this ignores the "headers"
                        let map_parts: Vec<u64> = parts
                            .iter()
                            .map(|s| s.parse::<u64>().expect("Could not parse map numer"))
                            .collect();
                        let map: MapRange = MapRange::new(
                            *map_parts.get(0).unwrap(),
                            *map_parts.get(1).unwrap(),
                            *map_parts.get(2).unwrap());
                        ranges.push(map);
                    }
                }
            }
            n += 1;
        }
        maps.push(ranges); // got to the end, add final set of maps


        // Part 1
        let mut min = u64::MAX;
        for seed in seeds.clone() {
            let value = translate_seed(&seed, &maps);
            min = cmp::min(min, value);
        }
        println!("Part 1 minimum: {}", min);

        //Part 2
        // This takes 1.5 hours to run.
        // I know there's a better way, and maybe I'll come back to it one day.
        let mut s = 0;
        let mut min2 = u64::MAX;
        while s < seeds.len() {
            println!("Doing pair starting at {}", s);
            let start = *seeds.get(s).expect("Couldn't get first seed number");
            s += 1;
            let end = *seeds.get(s).expect("Couldn't get second seed number");
            s += 1;
            for seed in start..start+end {
                let value = translate_seed(&seed, &maps);
                min2 = cmp::min(min2, value);
            }
        }
        println!("Part 2 minimum: {}", min2);
    }
}

fn translate_seed(seed: &u64, maps: &Map) -> u64 {
    let mut value = *seed;
    'map: for (_n, map) in maps.iter().enumerate() {
        for range in map {
            if let Some(new_value) = range.translate(value.clone()) {
                value = new_value;
                //println!("Map {} ({:?}) -> {}", n, range, value);
                continue 'map;
            }
        }
    }
    value
}

// The output is wrapped in a Result to allow matching on errors
// // Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let range = MapRange::new(10, 50, 30);
        assert_eq!(range.translate(60), Some(20));
        assert_eq!(range.translate(40), None);
        assert_eq!(range.translate(80), None);
        let range2 = MapRange::new(18, 25, 70);
        assert_eq!(range2.translate(49), Some(42));
    }
}
