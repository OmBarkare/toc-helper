use std::{fs, io::Read};

struct FiniteAutomata {
    alphabets: Vec<String>,
    states: Vec<String>,
    transition: Vec<Vec<String>>,
    initial: String,
    accept: Vec<String>,
}
fn main() {
    let config = fs::read_to_string("config.txt").expect("failed to read config file");
    let lines = config.split("\n");
    for line in lines {
        print!("{line}\n");
    }
}
