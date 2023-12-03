use core::num;

// check if digit
// true:
    // check if number is ongoing
    // add it to new str or ongoing string
    // check if number already has a digit neigbour
        // else check if digit has one
// false:
    // 
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

fn check_neighbors_for_symbol(lines: &Vec<String>, i: usize, j: usize, outNum: &mut Number) -> bool {
    let mut res = get_neigbours(lines, i, j, outNum);
    if res.len() > 0 {
        return true;
    }
    return false;
}

fn is_symbol(c: char) -> bool {
    if !c.is_digit(10) && c != '.' {
        return true;
    }
    return false;
}

fn match_js(out_neighbours: & mut Vec<char>, line: &String, i: usize, j: usize, outNum: &mut Number) {
    // left
    if j != 0 {
        match line.chars().nth(j-1) {
            Some(x) => 
            {
                if is_symbol(x) {
                    out_neighbours.push(x);
                    outNum.symbol = x;
                    outNum.symbol_pos = (i as i32, (j-1) as i32);
                };
            },
            None => (),
        };
    }
    // middle
    match line.chars().nth(j) {
        Some(x) => 
            {
                if is_symbol(x) {
                    out_neighbours.push(x);
                    outNum.symbol = x;
                    outNum.symbol_pos = (i as i32, (j) as i32);
                };
            },
            None => (),
    };
    // right
    match line.chars().nth(j+1) {
        Some(x) => 
            {
                if is_symbol(x) {
                    out_neighbours.push(x);
                    outNum.symbol = x;
                    outNum.symbol_pos = (i as i32, (j+1) as i32);
                };
            },
            None => (),
    };
}

fn get_neigbours(lines: &Vec<String>, i: usize, j: usize, outNum: &mut Number) -> Vec<char> {
    let mut neighbours : Vec<char> = Vec::new();

    // above
    if i != 0 {
    match lines.get(i-1) {
        Some(line) =>
        {
            match_js(&mut neighbours, line, i-1, j, outNum);
        }
        None => (),
    }
}

    // current
    match lines.get(i) {
        Some(line) =>
        {
            match_js(&mut neighbours, line, i, j, outNum);
        }
        None => (),
    }

    // below
    match lines.get(i+1) {
        Some(line) =>
        {
            match_js(&mut neighbours, line, i+1, j, outNum);
        }
        None => (),
    }
    return neighbours;
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
            else { // . or symbol
                if !new_number {
                    new_number = true;
                    if (number.has_symbol) {
                        numbers.push(number.clone());
                    }
                    all_numbers.push(number);
                    number = Number::new();
                }   
            }
        }
        if (number.has_symbol) {
            numbers.push(number.clone());
            all_numbers.push(number)
        }
                   
    }

    let mut result = 0;
    for number in &mut numbers {
        let num = number.digits.iter().fold(0, |acc, elem| acc * 10 + elem);
        number.number = num;
        result += num;
    }

    for number in &mut all_numbers {
        let num = number.digits.iter().fold(0, |acc, elem| acc * 10 + elem);
        number.number = num;
    }

    check_part_2(all_numbers);

    //println!("{:?}", result);
}

fn check_part_2(all_numbers: Vec<Number>) {
    let mut res = 0;
    for number in &all_numbers {
        if number.symbol == '*' {
            let star_numbers :Vec<_> = all_numbers.iter().filter(|x| {
                number.symbol_pos == x.symbol_pos
            }).collect();
            //println!("{:?}", star_numbers);
            if star_numbers.len() == 2 {
                let amount = star_numbers.iter().fold(1, |acc, elem| acc * elem.number);
                res += amount;
            }
        }
    }
    println!("{:?}", res / 2);
}

pub fn run(part_two: bool) -> () {
   read_numbers();

}