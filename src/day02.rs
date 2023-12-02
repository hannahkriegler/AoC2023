use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    cmp
};
use crate::day01;

#[derive(Debug)]
struct Game{
    id: i32,
    draws: Vec<Draw>, 
    min_red: i32,
    min_blue: i32,
    min_green: i32
}

#[derive(Debug)]
struct Draw {
    id: i32,
    red: i32,
    blue: i32,
    green: i32
}

impl std::fmt::Display for Draw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Draw {}: ", self.id)?;

        let mut first = true;

        if self.red != 0 {
            write!(f, "red: {}", self.red)?;
            first = false;
        }
        if self.blue != 0 {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "blue: {}", self.blue)?;
            first = false;
        }
        if self.green != 0 {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "green: {}", self.green)?;
        }

        Ok(())
    }
}

fn read_games() -> Vec<Game>{
    let games_string = day01::lines_from_file("inputs/input02");
    let mut games : Vec<Game> = Vec::with_capacity(games_string.capacity());
    
    for (index, line) in games_string.iter().enumerate()
    {
        let (_game_string, draw_string) = line.split_once(":").unwrap();
        let draws_string = draw_string.split(";").collect::<Vec<_>>();
        let mut draws : Vec<Draw> = Vec::with_capacity(draw_string.len());
        for (index, &draw) in draws_string.iter().enumerate()
        {
            draws.push(parse_draw(draw, index as i32));
        }

        let mut game = Game{
            id: (index + 1) as i32,
            draws: draws, 
            min_blue: 0,
            min_green: 0,
            min_red: 0
        };

        for draw in &game.draws {
            game.min_blue = cmp::max(game.min_blue, draw.blue);
            game.min_green = cmp::max(game.min_green, draw.green);
            game.min_red = cmp::max(game.min_red, draw.red);
        }

        games.push(game);
    }
    
    return games;
}

fn parse_draw(a_draw: &str, index: i32) -> Draw {
    
    let mut draw = Draw{
        id: index,
        red: 0,
        blue: 0,
        green: 0
    };
    
    let colors:Vec<&str> = a_draw.split(",").collect();
    for color in colors
    {
        if color.contains("red"){
            draw.red = color.split_whitespace().next().unwrap().to_string().parse::<i32>().unwrap();
        }
        if color.contains("blue") {
            draw.blue = color.split_whitespace().next().unwrap().to_string().parse::<i32>().unwrap();
        }
        if color.contains("green") {
            draw.green = color.split_whitespace().next().unwrap().to_string().parse::<i32>().unwrap();
        }
    }

    return draw;
}

pub fn run(part_two: bool) -> () {
    let mut games = read_games();
    
    if !part_two {
        let part_1 = check_part_1(games);
        println!("{:?}", part_1);
    }
    else {
        let part_2 = check_part_2(games);
        println!("{:?}", part_2);
    }

}

fn check_part_2(games: Vec<Game>) -> i32 {
    let mut result = 0;

    for game in games {
        result += game.min_blue * game.min_green * game.min_red;
    }

    return result;
}
   

fn check_part_1(games: Vec<Game>) -> i32 {
    let mut result = 0;
    for game in games {
        let mut valid = true;
        for draw in game.draws {
            
            if draw.red > 12 || draw.green > 13 || draw.blue > 14 {
                    valid = false;
                }
        }
        if valid {
            result += game.id;
        }
    }
    return result;
}
