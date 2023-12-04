use core::num;

use crate::day01;
#[derive(Clone)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
    points: i32,
    matches :i32
}

impl Card {
    pub fn new(id: i32) -> Self {
        Card {
            id: id,
            winning_numbers: Vec::new(),
            numbers: Vec::new(),
            points: 0,
            matches: 0
        }
    }
}

fn get_id_from_line(line: &str) -> i32 { // input: "Card XX"
    let mut iter = line.split_whitespace();
    return iter.nth(1).unwrap().parse().unwrap();
}

fn get_numbers_from_line(line: &str) -> (Vec<i32>, Vec<i32>) { // input: ": 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
    let (winning_num_s, numbers_s) = line.split_at(line.find('|').unwrap());
    
    let mut winning_numbers :Vec<i32> = Vec::new();
    let mut iter = winning_num_s.split_whitespace();
    let _ = iter.filter(|x| !x.contains(':')).map(|i| winning_numbers.push(i.parse().unwrap())).collect::<Vec<_>>();
    
    let mut numbers :Vec<i32> = Vec::new();
    iter = numbers_s.split_whitespace();
    let _ = iter.filter(|x| !x.contains('|')).map(|i| numbers.push(i.parse().unwrap())).collect::<Vec<_>>();

    return (winning_numbers, numbers);
}

fn read_cards() -> Vec<Card> {
    let lines = day01::lines_from_file("inputs/input04");
    let mut cards = Vec::new();
    for line in lines {
        let (card_id, card_num) = line.split_at(line.find(':').unwrap());
        let id = get_id_from_line(card_id);
        
        let mut card = Card::new(id);
        let (winning_numbers, numbers) = get_numbers_from_line(card_num);
        card.winning_numbers = winning_numbers;
        card.numbers = numbers;
        cards.push(card);
    }
    return cards
}

fn calculate_points(card: &mut Card) -> i32 {
    let mut points = 0;
    let mut matches = 0;
    let _ = card.numbers.iter().map(|x| {
        if card.winning_numbers.contains(x) {
            if points > 0 {
                points *=2;
            } else {
                points +=1;
            }
            matches +=1;
            
        }
    }).collect::<Vec<_>>();
    
    card.matches = matches;
    return points;
}

fn calculate_part_2(cards: Vec<Card>) -> i32 {
    
    let mut copies = vec![0;cards.len()];

    for (i, card) in cards.clone().iter().enumerate() {
        for c in 0..copies[i] +1 {
            for m in 1..(card.matches+1) {
                copies[i+m as usize] += 1;
            }
        }
        copies[i] +=1;
    }
    return copies.iter().sum();
    
}

pub fn run() -> () {
    let mut res_part_1 = 0;

    let mut cards = read_cards();

    for mut card in cards.iter_mut() {
        card.points = calculate_points(card);
        res_part_1 += card.points;
    }

    let res_part_2 = calculate_part_2(cards);

    println!("Part 1:{:?}", res_part_1);
    println!("Part 2: {}", res_part_2)
 }