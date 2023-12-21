/*
https://adventofcode.com/2023/day/20
--- Day 20: Pulse Propagation ---
 */


/*
Iterating until RX receives the correct input doesn't work.
RX always receives 0 LOW and 4..10 HIGH for MANY iterations
It receive input from several & nodes, which themselves receives from &
so there is a lot of stages that need to align.

With all those & and ~ it looks a bit like a SAT problem, with added 
feedback-loop to make it more fun. 
But I don't think it can really be answered with a SAT solver.

Instead, do a bit like Day 14 part 2:
find the cycles for each of the individual & inputs,
and compute the lowest common denominator

*/

/*
This solution is based on the particular individual input data
I got, clearly not generalizable to any arbitrary turing-complete state-machine.

In particular the final modules path I have are of the form (in reverse order)

RX  <-- &bq <--- { &vg            <-- &lx  <-- { many lx sources}
                      {many more} <-/  
                 { &kp            <-- &db
                 { &gc            <-- &qz
                 { &tx            <-- &sd
                
So RX receives LOW only when bq receives HIGH from the 4 others;
the 4 others (vg,kp,gc,tx) have a single input so they send HIGH only when their
input is LOW (from lx, db, qz, sd)

Algo will search from RX the first set of 4 conjunction modules,
then run_button() in loop until a cycle is found for each of those
modules state/output.



The next comments are based on the LAST pulse sent before end of cycle.
THis gives the wrong state because all final conjunction nodes reset to a different pulse after sending their "true" pulse once, so the last state
does not give the pulse received by rx predecessors.
Additionnaly, checking if the conj.node sent a Low pulse "anytime" before
the end also gives useless data because they always send something: the
problem is that it's not at the same time as others.

So the current algo is hopeless as is. Only method seems to analyze by
hand the circuit and get the adder/carry/12bits counter implemented
and fake it.


lx/db/qz/sd never sent any LOW pulse during the first 100K iterations.
So the cycle detection instead must be pushed one level deeper:
* LX sends LOW only when all the { many lx sources } send HIGH
  => find cycles in { many lx source } = { lg, gf, bm, cp, xm, kh, lh, dl, zx, gb }
* The same for { many db sources }  = { vv, sp, bh, kr, xz, qf, mq, zs }
* same for gc sources, tx sources

Until we finaly get LCM { LCM{lx sources}, LCM{DB sources}, LCM{GC sources}, LCM{TX sources}}

 */

use std::io;
use std::collections::HashMap;
use std::collections::VecDeque;

// We'll map short pulse to false, long pulse to true
// (Could have been an enum, initial idea was not to have any custom symbol)
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
    sent_high_once: bool, // sent a HIGH pulse once this cycle (may not be the last sent)
}

impl Module {
    fn new_broadcaster(dest: Vec<String> ) -> Self {
	Self {
	    mtype: Broadcaster,
	    destinations: dest,
	    sent_high_once: false,
	}
    }

    fn new_flipflop(dest: Vec<String> ) -> Self {
	Self {
	    mtype: FlipFlop(false),
	    destinations: dest,
	    sent_high_once: false,
	}
    }

    fn new_conjunction(dest: Vec<String> ) -> Self {
	Self {
	    mtype: Conjunction(HashMap::<String,Pulse>::new()),
	    destinations: dest,
	    sent_high_once: false,
	}
    }

    fn insert_input(&mut self, input_name:&String) {
	match self.mtype {
	    Conjunction(ref mut inputs) => { inputs.insert(input_name.clone(), LOW); },
	    _ => {},
	}
    }

    // update last_sent
    // returns the ordered list of the output pulses to destinations.
    fn send_destinations(&mut self, pulse: Pulse) -> Vec<(String, Pulse)> {
	if pulse == HIGH {
	    self.sent_high_once = true;
	}
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

    // Reset the "sent_high_once" for this new cycle
    fn reset_stats(&mut self) {
	self.sent_high_once = false;
    }
}



// Utility debug function. For compact representation of many bools,
// use numbers to display 4 bits at a time.
// Note this may lead to visual false negatives for cycles
// if the cycle length is not divisible by 4.
fn print_bool_vec(a: &Vec<bool>) {
    let charmap = &".123abcdABCD*%$#";
    eprint!("{}x[", a.len());
    for v in a.chunks_exact(4) {
	let bits:u8 =
	      (1 * v[0] as u8)
	    + (2 * v[1] as u8)
	    + (4 * v[2] as u8)
	    + (8 * v[3] as u8);
	eprint!("{}", &charmap[bits as usize..1+bits as usize]);
    }
    eprintln!("]");
}

// Find a cycle in the pattern.
// Begining of array may not have stabilized so returns the pair
// (start of cycle, cycle length), the caller allows to ignore
// at most the first maxstart elements.
fn find_smallest_cycle(a: &Vec<bool>, maxstart: usize) -> Option<(usize, usize)> {
    // custom algorithm, bruteforcy. Don't know if there is a
    // well-known optimization.
    // Could have used a regex like "(.+)(\1)*" to let the regexp matching
    // find the solution too.

    // Iterate over all sizes (starting from 1), get the last string
    // of size S, and compare if all previous [S] sized strings are
    // identical. If not, repeat for next size.
    
    let max = a.len();
    for size in 1..max/2 {
	let candidate = &a[max-size..max];
	let mut ok = true;
	let mut check_offset = size;
	// iterate over the other chunks.
	for check in a.rchunks_exact(size).skip(1) {
	    check_offset += size;
	    //eprintln!("comparing {:?} and {:?}", candidate, check);
	    if check != candidate {
		eprintln!("found difference on size {size}, at offset -{check_offset}");
		if max - check_offset > maxstart {
		    ok = false;
		} else {
		    // difference was at the beginning: allowed,
		    // but correct the exact cycle start
		    check_offset -= size;
		}

		break;
	    }
	}
	if ok {
	    let start_cycle = max - check_offset;
	    eprintln!("Candidate is OK for size {size} starting at {start_cycle}");
	    return Some((start_cycle, size));
	}
    }

    None
    
}


struct Network {
    modules: HashMap<String, Module>,
}


impl Network {
    // Send the initial low pulse from Button to Broadcaster.
    // Follow the network activity until no more pulse is emitted.
    fn run_button(&mut self) -> i32 {

	let mut queue = VecDeque::<(String, String, Pulse)>::new(); // source, dest, pulse value

	queue.push_back(("button".to_string(), "broadcaster".to_string(), LOW));

	let mut converge=0;
	while let Some((origin, dest, p)) = queue.pop_front() {
	    converge += 1;

	    if let Some(module) = self.modules.get_mut(&dest) {
		let next_dests = module.receive_pulse_from(p, &origin);
		for (next_name, next_pulse) in next_dests {
		    queue.push_back((dest.clone(), next_name, next_pulse));
		}
	    } else {
		//eprintln!("destination Module name {dest} not found in network map !");
		// we know, this is RX...
	    }
	}

	return converge;
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

    // Perform a backward search of the final modules influencing
    // the RX received signal.
    fn find_rx_senders(&self, dest:&String) -> Vec<String> {
	// Custom impl for the specific input structure.
	// Could have hardcoded the 4 strings names at this point..
	match dest.as_str() {
	    "TEST" => vec!["dl".to_string(),
			   "lh".to_string(),
			   "lx".to_string()],
	    "rx" => vec!["lx".to_string(),
			 "db".to_string(),
			 "qz".to_string(),
			 "sd".to_string()],
	    "lx" => vec!["lg".to_string(),
			 "gf".to_string(),
			 "bm".to_string(),
			 "cp".to_string(),
			 "xm".to_string(),
			 "kh".to_string(),
			 "lh".to_string(),
			 "dl".to_string(),
			 "zx".to_string(),
			 "gb".to_string()],
	    "db" => vec!["vv".to_string(),
			 "sp".to_string(),
			 "bh".to_string(),
			 "kr".to_string(),
			 "xz".to_string(),
			 "qf".to_string(),
			 "mq".to_string(),
			 "zs".to_string()],
	    _ => Vec::<String>::new(),
	}
			 
    }

    fn detect_rx_cycles(&mut self) {

	// lx sources: all are same cycle size 4027 starting at 3891
	//let monitor = self.find_rx_senders(&"lx".to_string());

	// db sources: all are cycle size 3929 starting at 354
	//let monitor = self.find_rx_senders(&"db".to_string());

	//let monitor = self.find_rx_senders(&"TEST".to_string());

	let monitor = self.find_rx_senders(&"rx".to_string());

	let mut monitor_history = Vec::<Vec::<Pulse>>::new();
	for _ in &monitor {
	    monitor_history.push(Vec::<Pulse>::new());
	}

	// "dl" and "lh" are constant false in the first 1000 iters;
	// need to reach higher to get first very large cycles
	for k in 1..11000 {
	    for (_,m) in self.modules.iter_mut() {
		m.reset_stats();
	    }

	    let converge = self.run_button();
	    if (k % 1000) == 0 {
		eprintln!("#{k} run converged in {converge} iterations");
	    }
	    let mut h_idx = 0;
	    for m in &monitor {
		let m = self.modules.get(m).expect("Named module should be found in the network");
		monitor_history[h_idx].push(m.sent_high_once);
		h_idx += 1;
	    }
	}


	for (idx, name) in monitor.iter().enumerate() {
	    let h = &monitor_history[idx];
	    eprintln!("state history of {name}:");
	    print_bool_vec(h);
	    find_smallest_cycle(h, 5000);
	}

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

	self.network.detect_rx_cycles();
	
	// Bruteforcing doesn't work, who would have guessed
	//for k in 1..200000 {
	//    let (_,_) = self.network.run_button();
	//    if self.network.rx_ok {
	//	self.total = k;
	//	eprintln!("RX correct input at iteration {k}");
	//	break;
	//    }
	//}

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
