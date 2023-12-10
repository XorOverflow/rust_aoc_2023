/*
https://adventofcode.com/2023/day/8
--- Day 8: Haunted Wasteland ---
 */


use std::io;
use std::collections::HashMap;

type Node = (String, String);

// Solver for this particular problem

struct Solver {
    walk: String,
    total: u64,
    network: HashMap<String,Node>,
    starting_a: Vec<String>,
}

impl Solver {
    fn new(s: &str) -> Self {
        Self{walk:s.to_string(),
             total : 0,
             network: HashMap::<String,Node>::new(),
             starting_a: Vec::new(),
        }
    }

    // process one text line of input
    fn process(&mut self, l: &str) {
        if l.len() != 16 {
            panic!("Network node size invalid, no parsing");
        }
        let node_name = String::from(&l[0..3]);
        let node_L = String::from(&l[7..10]);
        let node_R = String::from(&l[12..15]);
        if &node_name[2..] == "A" {
            self.starting_a.push(node_name.clone());
        }

        self.network.insert(node_name, (node_L, node_R));

    }


    // Brute-force version, actually travel all nodes in parallel
    // (the starting set contains 6 ..A nodes, and 262 L/R instructions)
    // this seems to take a VERY LONG TIME with max 2 or barely 3
    // nodes ending with ..Z after a few minutes.
    // This will probably never find the correct path in human time.
    fn postprocess_brute_force(&mut self) {
        let mut step = 0;
        let mut direction = self.walk.chars();
        let mut parallel_current = self.starting_a.clone();


        eprintln!("Starting from {:?}", parallel_current);
        loop {
            let d = direction.next();
            if d == None {// End of L/R instruction: wrap around
                direction = self.walk.chars();
                continue; // try again
            };
            let mut total_z = 0;
            for l in parallel_current.iter_mut() {
                let node = self.network.get(l).unwrap();
                let dest: String;
                match d {
                    Some('L') => dest = node.0.clone(),
                    Some('R') => dest = node.1.clone(),
                    _ => panic!("Invalid left/right instruction"),
                }
                if &dest[2..] == "Z" {
                    total_z += 1;
                }
                // replace current (parallel) position with new destination
                // (mutable iterator)
                *l = dest;
            }
            //eprintln!("new step at {:?}, non-z detected = {}", parallel_current, any_not_z);
            if total_z >= 2 {
                eprintln!("Step {step}: Z = {total_z}");
            }

            step += 1;
            if total_z == parallel_current.len() { // all Z ?
                break;
            }
            
        }
        self.total = step;
    }

    // Intelligent version:

    // Final idea by using the actual behavior of input data:
    // Each starting ..A node reaches a Z node for the first time after an integral number
    // of cycling through all the walking steps (263 long). first node reaches after 79*263 steps,
    // seconds adter 73*263 steps, third in 47*263 steps... 263 is prime and the multiplier seems
    // to be always a prime number too.
    // If noted as M[i] * 263, then the first common number where all stars align would be, as lower common
    // multiple, Products (M[i]) * 263.

    // The initial, general (and dropped) idea where some spurious Z could be reached inside a larger cycle:
    // process each path from one starting node individually, and
    // note all steps reaching a Z. To know when to stop (as the L/R instruction may need
    // to be reused several times), as there is a finite number of nodes and walk instructions,
    // we can be sure that the path will end being cyclical and go back to the same node and
    // the same l/r instruction index.
    // The cycle could be limited only to an arbitrary subpart but we could make a bet that it will
    // return to the initial ..A starting node (if not, change the algo again to detect an arbitrary loop)
    // While doing this, note in a vector all the step numbers hitting a ..Z node.
    
    // Once the lists are done for all starting nodes, the "all Z nodes" should be found at step number N
    // such that for all path lists L of sizes P[i], "L[N % P[i]]  is a Z" (?)
    // If the actual input has a special behavior of hitting Z only once every 1 + 262 for the first starting node,
    // then 2 + 262 for the seconde node, 3 + 262 for the third... and they are primes relatively to
    // each other, then the common N for ALL path to reach all Z could be something like (262+1)*(262+2)*(262+3)...
    fn postprocess(&mut self) {
        let mut direction = self.walk.chars();
        let mut parallel_current = self.starting_a.clone();
        let mut parallel_z_index = Vec::<u32>::new();


        eprintln!("Starting from {:?}", parallel_current);
        for mut l in parallel_current {
            let mut step = 0;
            let initial_l = l.clone(); 
            loop {
                let d = direction.next();
                if d == None {// End of L/R instruction: wrap around
                    direction = self.walk.chars();
                    continue; // try again
                };
                step += 1;
                let node = self.network.get(&l).unwrap();
                let dest: String;
                match d {
                    Some('L') => dest = node.0.clone(),
                    Some('R') => dest = node.1.clone(),
                    _ => panic!("Invalid left/right instruction"),
                }
                if &dest[2..] == "Z" {
                    eprintln!("{initial_l} reaches first Z {} in {step}", dest);
                    parallel_z_index.push(step);
                    break;
                }
                l = dest;
            }

        }
        eprintln!("List of Z index: {:?}", parallel_z_index);
        // Compute lowest multiple (requires 64 bits)
        let wlen = self.walk.len() as u64;
        self.total = wlen * parallel_z_index.iter().map(|n| *n as u64 / wlen).product::<u64>() ;
    }

    
    // Returns the final string of expected output
    fn result(&mut self) -> String {
        self.postprocess();
        self.total.to_string()
    }
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("First line");
    let mut s = Solver::new(input.trim());

    io::stdin().read_line(&mut input).expect("Second empty line");

    input = String::from("");

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
