use regex::Regex;

use crate::day01;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
    beat_num: usize
}

fn create_races_part_1() -> Vec<Race> {
    let re = Regex::new(r"\b\d+\b").unwrap();
    let mut lines = day01::lines_from_file("inputs/input06");
    let times:Vec<usize> = re.find_iter(&lines[0])
        .map(|m| m.as_str().parse::<usize>().unwrap()).collect();

    let distances:Vec<usize> = re.find_iter(&lines[1])
        .map(|m| m.as_str().parse::<usize>().unwrap()).collect();
    
    let mut races: Vec<Race> = Vec::new();
    for i in 0..times.len() {
        races.push(Race { time: times[i], distance: distances[i], beat_num: 0 });
    }

    return races;
}

// fn create_races_part_2() -> Vec<Race> {
//     let re = Regex::new(r"\b\d+\b").unwrap();
//     let mut lines = day01::lines_from_file("inputs/testinput06");

//     let time = re.find_iter(&lines)
//         .flat_map(|m| m.as_str().chars())
//         .filter(|c| c.is_digit(10))
//         .collect::<String>()
//         .parse::<usize>()
//         .unwrap_or(0);

//     let times:Vec<usize> = [time].to_vec();

//     let distances:Vec<usize> = re.find_iter(&lines[1])
//         .map(|m| m.as_str().parse::<usize>().unwrap()).collect();
    
//     let mut races: Vec<Race> = Vec::new();
//     for i in 0..times.len() {
//         races.push(Race { time: times[i], distance: distances[i], beat_num: 0 });
//     }

//     return races;
// }

pub fn run() -> () {
    
    
    let races = create_races_part_1();

    let mut res_part_1 = 1;
    for mut race in races {
        
        for i in 0..race.time + 1 {
            let time_left = race.time - i;
            let speed = i;

            let distance = speed * time_left;
            
            if distance > race.distance {
                race.beat_num += 1;
            }
        }
        println!("{:?}", race);
        res_part_1 *= race.beat_num;
    }

    println!("{:?}", res_part_1);
    



    // for (race) in times {
    //     let seconds_left
    // }
}