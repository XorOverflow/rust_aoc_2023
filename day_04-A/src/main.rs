/*
https://adventofcode.com/2023/day/4

 */


use std::io;
use std::str::FromStr;
use std::collections::HashSet;

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
        let line_parts: Vec<&str> = l.split(&[':', '|'][..]).collect();
        if line_parts.len() != 3 {
            panic!("Invalid input {}", l);
        }
        // Ignore card number prefix in [0]
        // split_whitespace() is better than split(" ") in this case because
        // two consecutive spaces are treated as only one separator, instead of
        // outputing a "" in their middle which should be filtered to keep only
        // the numbers, or at string start/end.
        // (input example pads single-digit numbers with spaces for formatting)

        // keep cards number as "str" or parse them into i32 ? probably useless if the
        // string hash is tested as fast as a number
        let winning: HashSet<&str> = line_parts.get(1).unwrap().split_whitespace().collect();
        let yours: HashSet<&str> = line_parts.get(2).unwrap().split_whitespace().collect();

        eprintln!("numbers: {:?} and {:?}", winning, yours);

        // Get size of Intersection
        let sames = winning.intersection(&yours).count();
        // For scoring, if > 0 we use powers of 2.
        if sames > 0 {
            eprintln!("{} winning numbers", sames);
            self.total += 1 << (sames - 1);
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
