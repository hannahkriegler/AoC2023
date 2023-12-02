use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn run(part_two: bool) -> i32 {
    let lines = lines_from_file("inputs/input01");
    let mut result = 0;
    for mut line in lines{
        if part_two {
            line = convert_to_digit(&line);
        }
        result += ints_from_line(&line);
    }
    return result;
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String>
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn ints_from_line(line: &str) -> i32
{
    let mut numbers = [0,0];
    let first = line.chars()
        .find(|a| a.is_digit(10))
        .and_then(|a| a.to_digit(10))
        .unwrap();
        
    let last = line.chars()
        .rev()
        .find(|a| a.is_digit(10))
        .and_then(|a| a.to_digit(10))
        .unwrap();
       
    numbers[0] = first as i32;
    numbers[1] = last as i32;

    let test = numbers.iter().fold(0, |first, last| first * 10 + last);
    return test;
} 


//////// Part two ///////////
fn convert_to_digit(line:&str) -> String
{
    let mut result = str::replace(line, "one", "on1e");
    result = str::replace(&result, "two", "t2wo");
    result = str::replace(&result, "three", "t3hree");
    result = str::replace(&result, "four", "fo4ur");
    result = str::replace(&result, "five", "fi5ve");
    result = str::replace(&result, "six", "si6x");
    result = str::replace(&result, "seven", "se7ven");
    result = str::replace(&result, "eight", "ei8ght");
    result = str::replace(&result, "nine", "ni9ne");

    return result;
}