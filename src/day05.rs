use std::process::Output;

use regex::Regex;

use crate::day01;
#[derive(Clone)]
struct Seed {
    id: usize,
    range: usize,
}

impl Seed {
    pub fn new() -> Self {
        Seed { id: 0, range: 0 }
    }
}

#[derive(Clone)]
struct Map {
    name: String,
    entries: usize,
    des_range_start: Vec<i64>,
    source_range_start: Vec<i64>,
    range_length: Vec<i64>
}

impl Map {
    pub fn new(name: String) ->Self {
        Map { name: name, entries: 0, des_range_start: Vec::new(), source_range_start: Vec::new(), range_length: Vec::new() }
    }
}

fn map_seed_to_location(an_input: usize, maps: &Vec<Map>) -> usize {
    
    let mut input = an_input as i64;
    let mut output = input;
    for m in maps {
        output = map(input, m);
        input = output;
    }
    return output as usize;
}


fn map(input: i64, map: &Map) -> i64 {
    let mut output = input as i64;

    for i in 0..map.range_length.len() {
        if input >= map.source_range_start[i] && input < (map.source_range_start[i] + map.range_length[i]) {
            let offset:i64 = map.des_range_start[i] as i64 - map.source_range_start[i] as i64;
            output = input + offset;
        }
    }

    return output;
}

pub fn run() -> () {
    
    let mut lines = day01::lines_from_file("inputs/input05");

    let seed_line = lines.iter().nth(0).unwrap();
    let seeds = parse_seeds(seed_line.to_string());
    let seeds_part_two = parse_seeds_part_two(seeds.clone());
    
    // delete seeds
    lines.remove(0);
    lines.remove(0);

    // parse maps
    let re_dig = Regex::new("[0-9]+").unwrap();
    let mut maps: Vec<Map> = Vec::new();
    let mut map_name: String = "".to_owned();
    let mut map_dest_start: Vec<i64> = Vec::new();
    let mut map_src_start: Vec<i64> = Vec::new();
    let mut map_range: Vec<i64> = Vec::new();
    for line in lines {
        if !re_dig.is_match(&line) && !line.is_empty() { // map name
            map_name = line;
        }
        else if !re_dig.is_match(&line) { // new line
            let mut map = Map::new(map_name.clone());
            map.des_range_start = map_dest_start.clone();
            map.source_range_start = map_src_start.clone();
            map.range_length = map_range.clone();
            map.entries = map.range_length.len();
            maps.push(map);
            map_dest_start.clear();
            map_src_start.clear();
            map_range.clear();
        }
        else {
            let line_numbers = re_dig.find_iter(&line).map(|se| se.as_str().parse::<usize>().unwrap()).collect::<Vec<_>>();
            map_dest_start.push(*line_numbers.iter().nth(0).unwrap() as i64);
            //println!("{:?}", line_numbers[0]);
            map_src_start.push(*line_numbers.iter().nth(1).unwrap() as i64);
            map_range.push(*line_numbers.iter().nth(2).unwrap() as i64);
        }
    }

    let mut locations:Vec<_> = Vec::new();

    for seed in seeds {
        //println!("###### Seed: {} #####", seed);

        let out = map_seed_to_location(seed, &maps);
        locations.push(out);
        //println!("###### Location: {} #####", out);
    }

    let result = locations.iter().min().unwrap();

    println!("###### MIN Location Part 1: {} #####", result);

    let mut min_seed_loc = map_seed_to_location(seeds_part_two[0].id, &maps);

    for seed in seeds_part_two {
        for i in 0..seed.range {
            let seed_to_look = seed.id + i;
            let out = map_seed_to_location(seed_to_look, &maps);
            if out < min_seed_loc {
                min_seed_loc = out;
            }
        }
    }

    println!("###### MIN Location Part 2: {} #####", min_seed_loc);
    
    
 }


 fn parse_seeds(line: String) -> Vec<usize> {
    let mut seeds = Vec::new();
    
    let re_dig = Regex::new("[0-9]+").unwrap();
    seeds = re_dig.find_iter(&line).map(|se| se.as_str().parse::<usize>().unwrap()).collect::<Vec<_>>();

    return seeds;
}

fn parse_seeds_part_two(seeds: Vec<usize>) -> Vec<Seed> {
    let mut seeds_range:Vec<Seed> = Vec::new(); 
    let _ = seeds.chunks(2).map(|chunk| {
        let seed = Seed { id: chunk[0], range: chunk[1] };
        seeds_range.push(seed);
    }).collect::<Vec<_>>();

    return seeds_range;
}