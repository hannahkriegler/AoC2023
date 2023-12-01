use core::num;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let lines = lines_from_file("src/input01");
    let mut result = 0;
    for line in lines{
        result += ints_from_line(&line);
    }
    print!("{:?}", result);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String>
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn ints_from_line(line: &str) -> i32
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