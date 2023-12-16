/*
https://adventofcode.com/2023/day/15
--- Day 15: Lens Library ---
(hash & hashmap)
 */


use std::io;

// Solver for this particular problem

struct Solver {
    total: u32,
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,
        }
    }


    // could be rewritten in a one-liner .map().fold()
    fn hash(l: &str) -> u8 {
        let mut h:u32 = 0;
        for c in l.chars() {
            let ascii = c as u32;
            h += ascii;
            h *= 17;
            h = h & 0xff;
        }
        (h & 0xff) as u8
    }

    // process the input
    fn process(&mut self, l: &str) {
        for step in l.split(',') {
            self.total += Self::hash(step) as u32;
        }
    }


    fn postprocess(&mut self) {
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

    // Only 1 long line of input
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(_) => {
            panic!("input error, exit");
        }
        Ok(0) => {
            panic!("Eof detected, no input ??");
        },
        Ok(_) => {
            let input_clean = input.trim(); // remove the \n
            s.process(input_clean);
        }
    }

    println!("{}", s.result());

}
