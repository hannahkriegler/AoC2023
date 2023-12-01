use std::fs;

fn main() {
    let input = fs::read_to_string("src/input01").expect("Unable to read file");
    println!("{}", input);
}
