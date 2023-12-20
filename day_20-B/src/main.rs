/*
https://adventofcode.com/2023/day/20
--- Day 20: Pulse Propagation ---
 */


use std::io;
use std::collections::HashMap;
use std::collections::VecDeque;

// We'll map short pulse to false, long pulse to true
type Pulse = bool;
const LOW:Pulse = false;
const HIGH:Pulse = true;

// XXX some ad-hoc inheritance/polymorphism in rust ?
#[derive(Clone,Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop(bool), // internal state on or off
    Conjunction(HashMap<String,Pulse>), // recent inputs
}
use ModuleType::*;

#[derive(Clone,Debug)]
struct Module {
    mtype: ModuleType,
    destinations: Vec<String>,
}

impl Module {
    fn new_broadcaster(dest: Vec<String> ) -> Self {
	Self {
	    mtype: Broadcaster,
	    destinations: dest,
	}
    }

    fn new_flipflop(dest: Vec<String> ) -> Self {
	Self {
	    mtype: FlipFlop(false),
	    destinations: dest,
	}
    }

    fn new_conjunction(dest: Vec<String> ) -> Self {
	Self {
	    mtype: Conjunction(HashMap::<String,Pulse>::new()),
	    destinations: dest,
	}
    }

    fn insert_input(&mut self, input_name:&String) {
	match self.mtype {
	    Conjunction(ref mut inputs) => { inputs.insert(input_name.clone(), LOW); },
	    _ => {},
	}
    }

    // returns the ordered list of the output pulses to destinations.
    fn send_destinations(&mut self, pulse: Pulse) -> Vec<(String, Pulse)> {
	self.destinations.iter().map(|n| (n.clone(), pulse)).collect()
    }
    // Process an input pulse.
    // Update its internal state and return the new pulses to send.
    fn receive_pulse_from(&mut self, pulse: Pulse, origin: &String) -> Vec<(String, Pulse)> {
	match self.mtype {
	    // re-send the same pulse to all dests
	    Broadcaster => self.send_destinations(pulse),
	    FlipFlop(ref mut internal_state) => if pulse == HIGH {
		Vec::new() // nothing happens
	    } else {
		let send_pulse = if *internal_state { LOW } else { HIGH };
		*internal_state = !*internal_state;
		self.send_destinations(send_pulse)
	    },
	    Conjunction(ref mut inputs) => {
		inputs.insert(origin.clone(), pulse);
		let all_high = inputs.iter().fold(true, |acc, (_,&v)| acc && v==HIGH);
		let send_pulse = if all_high { LOW } else { HIGH };
		self.send_destinations(send_pulse)
	    }
	}
    }
}

struct Network {
    modules: HashMap<String, Module>,
}


impl Network {
    // Send the initial low pulse from Button to Broadcaster.
    // Follow the network activity until no more pulse is emitted.
    // Return the pair of count of (Low, High) pulses sent
    // through the network
    fn run_button(&mut self) -> (i32, i32) {
	let mut low_count:i32 = 0;
	let mut high_count:i32 = 0;
	let mut queue = VecDeque::<(String, String, Pulse)>::new(); // source, dest, pulse value

	queue.push_back(("button".to_string(), "broadcaster".to_string(), LOW));

	let mut converge=0;
	while let Some((origin, dest, p)) = queue.pop_front() {
	    converge += 1;
	    if p == LOW {
		low_count += 1;
	    } else {
		high_count += 1;
	    }

	    if let Some(module) = self.modules.get_mut(&dest) {
		let next_dests = module.receive_pulse_from(p, &origin);
		for (next_name, next_pulse) in next_dests {
		    queue.push_back((dest.clone(), next_name, next_pulse));
		}
	    } else {
		eprintln!("destination Module name {dest} not found in network map !");
	    }
	}

	eprintln!("Network need {converge} iterations to send {low_count} LOW and {high_count} HIGH");
	(low_count, high_count)
    }

    // perform an initial pass to find all "origins" to each modules (from the
    // existing list of "destinations").
    // Required so Conjunction modules start with ALL their sources states correctly set.
    fn initialize_origins(&mut self) {

	// Rust borrow checker problems:
	// we need to do a double-loop on the modules hashmap,
	// to modify the element pointed in the inner loop.
	// this leads to intractable incompatibilities of mut/non mut.
	// Instead, copy the hashmap to do all the modifications
	// then copy back (in-place not possible)
	// Another solution could have been to copy only the set
	// of module names string, and do double lookups in the (mutable) hashmap ?

	let mut final_modules = self.modules.clone();

	for (k,m) in self.modules.iter() {
	    let origin = k;
	    for dest in m.destinations.iter() {
		if let Some(dest_m) = final_modules.get_mut(dest) {
		    dest_m.insert_input(&origin);
		} else {
		    eprintln!("A destination name {dest} is not found in the network (from {origin})");
		}
	    }
	}
	self.modules = final_modules;
    }

}

// Solver for this particular problem

struct Solver {
    total: i32,
    network: Network,
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,
	     network: Network {modules: HashMap::new(),},
        }
    }

    // process one text line of input
    fn process(&mut self, l: &str) {
	if let Some((mname, dests)) = l.split_once(" -> ") {
	    let dests:Vec<String> = dests.split(", ").map(|s| s.to_string()).collect();
	    let mut name = &mname[1..];
	    let module = match &mname[0..1] {
		"%" => Module::new_flipflop(dests),
		"&" => Module::new_conjunction(dests),
		_ => {name = mname; Module::new_broadcaster(dests)},
	    };
	    self.network.modules.insert(name.to_string(), module);
	} else {
	    eprintln!("warning malformed input, no ' -> '");
	}
    }


    fn postprocess(&mut self) {
	eprintln!("Network has {} modules",
		  self.network.modules.len());
	self.network.initialize_origins();
	let mut total_l = 0;
	let mut total_h = 0;
	for _ in 0..1000 {
	    let (l,h) = self.network.run_button();
	    total_l += l;
	    total_h += h;
	}
	self.total = (total_l) * (total_h);
    }
    
    // Returns the final string of expected output
    fn result(&mut self) -> String {
        self.postprocess();
        self.total.to_string()
    }
}

/* common to all problems */
fn main() {

    let mut s = Solver::new();

    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Err(_) => { println!("input error, exit"); break; }
            Ok(0) => {
                eprintln!("Eof detected");
                break;
            },
            Ok(_) => {
                let input_clean = input.trim(); // remove the \n
                s.process(input_clean);
            }
        }
        // must clear for next loop
        input = String::from("");
    }

    println!("{}", s.result());

}
