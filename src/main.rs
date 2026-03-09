use std::{default, fs, io::Read};

#[derive(Debug)]
struct FiniteAutomata {
    alphabets: Vec<String>,
    states: Vec<String>,
    transition: Vec<Vec<String>>,
    initial: String,
    accept: Vec<String>,
}

impl FiniteAutomata {
    fn init() -> FiniteAutomata {
        FiniteAutomata {
            alphabets: Vec::new(),
            states: Vec::new(),
            transition: Vec::new(),
            initial: String::new(),
            accept: Vec::new(),
        }
    }
}
fn main() {
    let config = fs::read_to_string("config.txt").expect("failed to read config file");
    let mut lines = config.split("\n");

    let mut line;
    let mut section;
    let mut finiteAutomata = FiniteAutomata::init();
    loop {
        line = lines.next().unwrap().split(" ");
        section = line.next().unwrap();
        // if section == "end" {
        //     break;
        // }
        match section {
            "alphabets:" => {
                println!("{}", section);
                for element in line {
                    println!("{:?}", element);
                    finiteAutomata.alphabets.push(element.to_string());
                }
            }
            "states:" => {
                println!("{}", section);
                for element in line {
                    println!("{:?}", element);
                    finiteAutomata.states.push(element.to_string());
                }
            }
            "initial:" => {
                println!("{}", section);
                for element in line {
                    println!("{:?}", element);
                    finiteAutomata.initial = element.to_string();
                }
            }
            "transition:" => {
                println!("in transition");
            }
            "final:" => {
                println!("{}", section);
                for element in line {
                    println!("{:?}", element);
                    finiteAutomata.accept.push(element.to_string());
                }
            }
            "end" => {
                break;
            }
            default => {
                print!("this is the section ->{}", section);
            }
        }
    }
    println!("{:?}", finiteAutomata);
}
