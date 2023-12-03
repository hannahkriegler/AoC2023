use core::num;
use crate::day01;

#[derive(Debug, Clone)]
struct Number {
    digits: Vec<i32>,
    number: i32,
    has_symbol: bool,
    symbol: char,
    symbol_pos: (i32, i32)
}
impl Number {
    pub fn new() -> Self {
        Number { digits : Vec::new(), number: 0, has_symbol : false, symbol : '.', symbol_pos: (-1, -1) }
    }
}

fn check_neighbors_for_symbol(lines: &Vec<String>, i: usize, j: usize, out_num: &mut Number) -> bool {
    return !get_neighbours(lines, i, j, out_num).is_empty()
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn match_js(out_neighbours: &mut Vec<char>, line: &str, i: usize, j: usize, out_num: &mut Number) {
    for offset in [-1, 0, 1].iter() {
        let new_j = (j as isize + offset) as usize;
        if let Some(x) = line.chars().nth(new_j) {
            if is_symbol(x) {
                out_neighbours.push(x);
                out_num.symbol = x;
                out_num.symbol_pos = (i as i32, new_j as i32);
            }
        }
    }
}

fn get_neighbours(lines: &[String], i: usize, j: usize, out_num: &mut Number) -> Vec<char> {
    let mut neighbours: Vec<char> = Vec::new();

    for offset in [-1, 0, 1].iter() {
        let new_i = (i as isize + offset) as usize;
        if let Some(line) = lines.get(new_i) {
            match_js(&mut neighbours, line, new_i, j, out_num);
        }
    }

    return neighbours
}

fn read_numbers() -> () {
    let lines = day01::lines_from_file("inputs/input03");
    let mut numbers : Vec<Number> = Vec::new();
    let mut all_numbers : Vec<Number> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let mut new_number = true;
        let mut number = Number::new();

        for (j, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                new_number = false;
                number.digits.push(c.to_digit(10).unwrap() as i32);
                if !number.has_symbol {
                    number.has_symbol = check_neighbors_for_symbol(&lines, i, j, &mut number);
                }
                // edge case last element
                if j == line.len() {
                    all_numbers.push(number.clone());
                }
            }
            else if !new_number {
                new_number = true;
                if number.has_symbol {
                    numbers.push(number.clone());
                }
                all_numbers.push(number);
                number = Number::new();
                  
            }
        }
        if number.has_symbol {
            numbers.push(number.clone());
            all_numbers.push(number)
        }
                   
    }

    all_numbers.iter_mut().for_each(|x| {
        let num = x.digits.iter().fold(0, |acc, elem| acc * 10 + elem);
        x.number = num;
    });


    let result_part1: i32 = numbers
        .iter_mut()
        .map(|x| { 
            let num = x.digits.iter()
                                    .fold(0, |acc, elem| acc * 10 + elem);
            x.number = num;
            num
        }).sum();
    
    let mut result_part2: i32 = 0;
    for number in all_numbers.iter().filter(|&x| x.symbol == '*') {
        let star_numbers: Vec<_> = all_numbers.iter().filter(|&x| number.symbol_pos == x.symbol_pos).collect();

        if star_numbers.len() == 2 {
            let amount: i32 = star_numbers.iter().map(|elem| elem.number).product();
            result_part2 += amount;
        }
    }


    println!("Part1: {:?}", result_part1);
    println!("Part2: {:?}", result_part2 / 2);
}


pub fn run() -> () {
   read_numbers();
}