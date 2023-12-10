/*
https://adventofcode.com/2023/day/<DAY>

 */


use std::io;
use std::str::FromStr;
use regex::Regex;
use std::ops::Range;


// Solver for this particular problem

struct Solver {
    total: i32,
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,
        }
    }

    // process one text line of input
    fn process(&mut self, l: &str) {
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
