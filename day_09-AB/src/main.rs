/*
https://adventofcode.com/2023/day/9
--- Day 9: Mirage Maintenance ---
 */


/*
 * This program solves part 1 and part 2 simultaneously.
 * For once, the "part 2 plot twist" could fit perfectly
 * in the algo of part 1 with just a few more lines.
 */


use std::io;
use std::str::FromStr;



struct ValueSequence {
    v: Vec<i32> ,
}

impl ValueSequence {
    fn from_vec(v: Vec<i32>) -> Self {
        Self {
            v: v,
        }
    }

    // Return a new valueSequence whose content
    // is the discrete derivation of self (adjacent difference)
    fn derive(&self) -> Self {
        // Use slice windows() to operate on a sliding
        // window of 2 elements to get their difference.
        let slice = &self.v[..];
        let deriv = slice.windows(2).map(|w| w[1]-w[0]).collect();
        ValueSequence::from_vec(deriv)
    }

    // Add a new elements to self using the last or first element of d
    fn grow_by_integration(&mut self, d: &ValueSequence) {
        self.v.insert(0, self.v[0] - d.v[0]);
        self.v.push(self.v.last().unwrap() + d.v.last().unwrap());
    }

    fn is_all_zero(&self) -> bool {
        for k in &self.v {
            if *k != 0 {
                return false;
            }
        }
        true
    }
}


// Solver for this particular problem

struct Solver {
    total_next: i32,
    total_previous: i32,
}

impl Solver {
    fn new() -> Self {
        Self{
            total_next : 0,
            total_previous : 0,
        }
    }

    // process one text line of input
    fn process(&mut self, l: &str) {
        let mut pyramid = Vec::<ValueSequence>::new();

        // The history of this line's value
        let v = ValueSequence::from_vec(l.split_whitespace()
                                        .map(|s| i32::from_str(s).unwrap())
                                        .collect()
        );
        pyramid.push(v);
        // compute the N derivation until reaching all zeros
        while !pyramid.last().unwrap().is_all_zero() {
            let deriv = pyramid.last().unwrap().derive();
            pyramid.push(deriv);
        }

        let derive_len = pyramid.len(); // for debug
        // Now do the reverse by growing each sequence from last to first.
        let mut deriv = pyramid.pop().unwrap();
        // Technically we should add a final "0" but all the other existing 
        // zeroes will work the same for this one.

        let mut last_value:i32 = 0;
        let mut first_value:i32 = 0;
        while let Some(mut current) = pyramid.pop() {
            current.grow_by_integration(&deriv);
            last_value = *current.v.last().unwrap();
            first_value = current.v[0];
            deriv = current;
        }

        eprintln!("Sequence {l}: ∂/∂t ^ {derive_len} => {first_value}..{last_value}");

        self.total_next += last_value; 
        self.total_previous += first_value; 
   }


    fn postprocess(&mut self) {
    }
    
    // Returns the final string of expected output
    fn result(&mut self) {
        self.postprocess();
        println!("Part A : {}", self.total_next.to_string());
        println!("Part B : {}", self.total_previous.to_string());
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

    s.result();

}
