use std::{collections::HashMap, fs};

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

    // iterator over lines of the txt file
    let mut lines = config.split("\n");

    // to iterate over elements in one line
    let mut line;

    // when a line contains a section heading, we will call next and store the section heading here
    // this way we know which section we are in
    let mut section;
    
    // this is where we will parse and store the finite automata
    let mut finiteAutomata = FiniteAutomata::init();

    // hashmaps to map all states and alphabets to a index
    let mut state_index: HashMap<String, u32> = HashMap::new();
    let mut alphabet_index: HashMap<String, u32> = HashMap::new();

    loop {
        // get a line and separate elements in a line at whitespaces
        line = lines.next().unwrap().trim().split(" ");

        // first element in each line is the section
        section = line.next().unwrap();

        // build finiteAutomata according to sections
        match section {
            "alphabets:" => {
                println!("{}", section); // debug
                
                let mut i: u32 = 0; // to index alphabets
                for element in line {
                    println!("{:?}", element);
                    finiteAutomata.alphabets.push(element.to_string());
                    alphabet_index.insert(element.to_string(), i);
                    i += 1;
                }
            }
            "states:" => {
                println!("{}", section); // debug

                let mut i: u32 = 0; // to index states
                for element in line {
                    println!("{:?}", element);
                    finiteAutomata.states.push(element.to_string());
                    state_index.insert(element.to_string(), i);
                    i += 1;
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
                finiteAutomata
                    .transition
                    .resize(state_index.len(), vec![String::new(); alphabet_index.len()]);
                loop {
                    let str = lines.next().unwrap().trim();
                    if str == "end_transition" {
                        break;
                    }
                    line = str.split(" ");
                    let curr_state_idx: usize = state_index[line.next().unwrap()] as usize;
                    let curr_alphabet_index: usize = alphabet_index[line.next().unwrap()] as usize;
                    finiteAutomata.transition[curr_state_idx][curr_alphabet_index] =
                        line.next().unwrap().to_string();
                    println!("{:#?}", line);
                }
            }
            "final:" => {
                println!("{}", section);
                for element in line {
                    println!("{:#?}", element);
                    finiteAutomata.accept.push(element.to_string());
                }
            }
            "end" => {
                break;
            }
            _default => {
                print!("this is the section ->{}", section);
            }
        }
    }
    println!("{:#?}", finiteAutomata);
}