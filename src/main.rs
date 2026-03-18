use std::{
    clone,
    collections::{BTreeMap, BTreeSet, HashMap, btree_map::Keys},
    fs,
    hash::Hash,
    io::BufRead,
    os::linux::raw::stat,
};

#[derive(Debug)]
struct FiniteAutomata {
    alphabets: Vec<String>,
    states: Vec<String>,
    transition: HashMap<String, HashMap<String, String>>,
    initial: String,
    accept: Vec<String>,
}

impl FiniteAutomata {
    fn init() -> FiniteAutomata {
        FiniteAutomata {
            alphabets: Vec::new(),
            states: Vec::new(),
            transition: HashMap::new(),
            initial: String::new(),
            accept: Vec::new(),
        }
    }

    // should return another FiniteAutomata struct
    fn minimise(&self) {
        // ----------------- SETUP -----------------

        // each index represents a equivalance set
        let mut e_classes: Vec<BTreeSet<String>> = Vec::new();
        let mut temp_eclasses: Vec<BTreeSet<String>> = Vec::new();

        e_classes.push(BTreeSet::new());
        e_classes.push(BTreeSet::new());
        let mut track_all: HashMap<String, u32> = HashMap::new();

        // removing unreachable states TODO

        // splitting final and non-final states and initializing track
        for element in &self.states {
            if self.accept.contains(element) {
                e_classes[1].insert(element.clone());
                track_all.insert(element.clone(), 1);
            } else {
                e_classes[0].insert(element.clone());
                track_all.insert(element.clone(), 0);
            }
        }

        // ---------------- MAIN LOOP ----------------

        loop {
            let mut buf_vector: Vec<BTreeSet<String>> = Vec::new();

            for class in &e_classes {
                if class.is_empty() {
                    continue;
                }

                let mut track_class: HashMap<String, isize> = HashMap::new();
                for s in class {
                    track_class.insert(s.clone(), -1);
                }

                //first element loop
                for (i, element) in class.iter().enumerate() {
                    if track_class[element] == -1 {
                        if let Some(ecidx) = track_class.get_mut(element) {
                            *ecidx = i as isize;
                        }
                    } else {
                        continue;
                    }
                    let mut v1: Vec<u32> = Vec::new(); // tuple of element1, initialized everytime
                    for alphabet in &self.alphabets {
                        v1.push(track_all[&self.transition[element][alphabet]]);
                    }

                    // second element loop
                    for (k, element2) in class.iter().enumerate() {
                        if track_class[element2] == -1 {
                            let mut v2: Vec<u32> = Vec::new(); // tuple of element2, initialized everytime
                            for alphabet in &self.alphabets {
                                v2.push(track_all[&self.transition[element2][alphabet]]);
                            }

                            if v1 == v2 {
                                if let Some(ecidx) = track_class.get_mut(element2) {
                                    *ecidx = i as isize;
                                }
                            }
                        }
                    }
                }
                // println!("eclass_idx -> {:?}", eclass_idx); //debug

                let mut new_classes: HashMap<isize, BTreeSet<String>> = HashMap::new();
                for (element, class_no) in track_class {
                    new_classes
                        .entry(class_no)
                        .or_insert_with(BTreeSet::new)
                        .insert(element);
                }
                for (_, temp_btree) in new_classes {
                    buf_vector.push(temp_btree);
                }
            }

            println!("bif_vector: {:?}", buf_vector);

            buf_vector.sort();
            temp_eclasses = e_classes.clone();
            e_classes = buf_vector;

            println!("t->{:?}\ne->{:?}", temp_eclasses, e_classes);
            if temp_eclasses == e_classes {
                break;
            }

            for (i, class) in e_classes.iter().enumerate() {
                for element in class {
                    if let Some(t) = track_all.get_mut(element) {
                        *t = i as u32;
                    }
                }
            }
        }

        // ---------------------------- BUILDING DFA OBJECT ------------------------------------------

        let mut minimisedAutomata = FiniteAutomata::init();
        let mut state = String::from("A");
        let mut class_to_state: HashMap<&BTreeSet<String>, String> = HashMap::new();

        // make mapping of equivalance classes to new "state"
        for class in &e_classes {
            class_to_state.insert(class, state.clone());

            // also building states set
            minimisedAutomata.states.push(state.clone());

            let char_as_u8 = state.chars().next().unwrap() as u8 + 1;
            state = (char_as_u8 as char).to_string();
        }

        // alphabets same for initial and minimised automata
        minimisedAutomata.alphabets = self.alphabets.clone();

        // final states
        for class in &e_classes {
            for state in &self.accept {
                if class.contains(state) {
                    minimisedAutomata.accept.push(class_to_state[class].clone());
                    break;
                }
            }
        }

        // transition
        for class in &e_classes {
            let curr_state = class_to_state[class].clone();
            let mut hmap: HashMap<String, String> = HashMap::new();

            for alphabet in &self.alphabets {            
                let first_state = class.iter().next().unwrap();
                let dest_state = &self.transition[first_state][alphabet];

                // finding which e_class contains the transition to first_state
                let dest_class = e_classes
                    .iter()
                    .find(|c| c.contains(dest_state))
                    .expect("Destination state not found in any class!");

                let new_state = class_to_state[dest_class].clone();

                hmap.insert(alphabet.clone(), new_state);
            }

            minimisedAutomata.transition.insert(curr_state, hmap);
        }
        // println!("{:#?}", minimisedAutomata.transition);

        // initial state
        let initial_class = e_classes
            .iter()
            .find(|class| class.contains(&self.initial))
            .unwrap()
            .clone();

        minimisedAutomata.initial = class_to_state[&initial_class].clone();

        println!("{:#?}", minimisedAutomata);
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
                loop {
                    let str = lines.next().unwrap().trim();
                    if str == "end_transition" {
                        break;
                    }
                    line = str.split(" ");
                    let state = line.next().unwrap().to_string();
                    println!("{}", state);
                    finiteAutomata
                        .transition
                        .insert(state.clone(), HashMap::new());
                    if let Some(hmap) = finiteAutomata.transition.get_mut(&state) {
                        hmap.insert(
                            line.next().unwrap().to_string(),
                            line.next().unwrap().to_string(),
                        );
                        hmap.insert(
                            line.next().unwrap().to_string(),
                            line.next().unwrap().to_string(),
                        );
                    }
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
                println!("syntax error ! ->{}", section);
            }
        }
    }
    println!("{:#?}", finiteAutomata);
    finiteAutomata.minimise();
}
