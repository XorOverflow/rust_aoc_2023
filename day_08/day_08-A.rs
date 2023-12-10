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
    total: i32,
    network: HashMap<String,Node>,
}

impl Solver {
    fn new(s: &str) -> Self {
        Self{walk:s.to_string(),
             total : 0,
             network: HashMap::<String,Node>::new(),
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
        self.network.insert(node_name, (node_L, node_R));
    }


    fn postprocess(&mut self) {
        let mut step = 0;
        let end = String::from("ZZZ");
        let start = String::from("AAA");
        let mut label = &start;
        let mut direction = self.walk.chars();
        loop {
            let mut node = self.network.get(label).unwrap();
            
            match direction.next() {
                Some('L') => label = &node.0,
                Some('R') => label = &node.1,
                Some(_) => panic!("Invalid left/right instruction"),
                None => { // End of L/R instruction: wrap around
                    direction = self.walk.chars();
                    continue;
                },
            }
            step += 1;
            if label == &end {
                break;
            }
            
        }
        self.total = step;
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
