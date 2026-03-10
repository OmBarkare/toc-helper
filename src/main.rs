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

    fn minimise(&self, state_index: HashMap<String, u32>, alphabet_index: HashMap<String, u32>) {

        // setup ------------------------

        //current nmuber of equivalence classes
        let num_eclass: u32 = 2;
        let mut eclass: HashMap<String, u32> = HashMap::new();
        let mut equivalent = false;

        // to compare equivalance of two states
        let mut v1: Vec<u32> = Vec::new();
        let mut v2: Vec<u32> = Vec::new();

        // splitting final and non-final states
        for element in &self.states {
            if self.accept.contains(element) {
                eclass.insert(element.to_string(), 1);
            } else {
                eclass.insert(element.to_string(), 0);
            }
        }
        let mut temp_eclass: HashMap<String, u32> = HashMap::new();

        // main minimisation loop ---------------------------

        loop {

            // break if previous class was same
            if eclass == temp_eclass {
                break;
            }

            // check equivalance of all elements, and put elements in respective class.
            // this is like performing one step of making separate classes
            for element1 in &self.states {
                // building v1 to compare equivalance
                for alphabet in &self.alphabets {
                    // for each transition for current element,
                    // we push the equivalance-class-index of the state at which we arrived after this transition
                    if self.transition[state_index[element1] as usize][alphabet_index[alphabet] as usize] != "" {
                        v1.push(
                            eclass[&self.transition[state_index[element1] as usize][alphabet_index[alphabet] as usize]],
                        );
                    }
                }

                for element2 in &self.states {
                    for alphabet in &self.alphabets {
                        println!("eclasss -> {:#?}", eclass);
                        println!("trying to access for {}, {}", element2, alphabet);
                        if self.transition[state_index[element2] as usize][alphabet_index[alphabet] as usize] != "" {
                            v2.push(
                                eclass[&self.transition[state_index[element2] as usize][alphabet_index[alphabet] as usize]],
                            );
                        }
                    }
                    if v2 == v1 {
                        equivalent = false;
                        eclass.insert(element2.to_string(), eclass[element1]);
                    }
                    v2.clear();
                }

                // checking equivalance with all elements after the current element
                // reseting v1
                v1.clear();
            }
            temp_eclass = eclass.clone();
        }

        println!("eclasss -> {:#?}", eclass); //debug
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
    finiteAutomata.minimise(state_index, alphabet_index);
}
