/*
https://adventofcode.com/2023/day/15
--- Day 15: Lens Library ---
(hash & hashmap)
 */


use std::io;
use std::str::FromStr;
use std::collections::HashMap;

// Solver for this particular problem


type LensBox = Vec<String>;

struct Solver {
    total: u32,
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,
        }
    }

    fn hash(l: &str) -> u8 {
        (l.chars().fold(0, |acc, c|  (((acc as u32) + (c as u32)) * 17 ) & 0xff) & 0xff) as u8
    }

    // process the input
    fn process(&mut self, l: &str) {
        let mut lenses:HashMap<String, u32> = HashMap::new(); // all the lenses ("label" string) with their focal length
        let mut boxes = vec![ LensBox::new() ; 256]; // The boxes containing lenses of identical hash

        for step in l.split(',') {
            if step.contains('-') { // lab-
                if let Some((label,_)) = step.split_once('-') {
                    let lnum = Self::hash(&label);
                    if let Some(pos) = &boxes[lnum as usize].iter().position(|s| s == label) {
                        boxes[lnum as usize].remove(*pos);
                    }
                    // Lenses that are not in a box don't count for final focusing power
                    lenses.remove(&String::from(label));
                }
            } else { // "lab=x"
                if let Some((label,focal_length)) = step.split_once('=') {
                    let lnum = Self::hash(&label);
                    if boxes[lnum as usize].iter().position(|s| s == label) == None {
                        boxes[lnum as usize].push(String::from(label));
                    }
                    lenses.insert(String::from(label), u32::from_str(focal_length).unwrap());
                }
            }
        }

        // debug
        eprintln!("Boxes = ");
        for b in &boxes {
            eprintln!("B[] = {:?}", b);
        }
        eprintln!("Lenses = {:?}", lenses);

        // Compute focusing power
        // arbitrary order is ok
        self.total = lenses.iter()
            .fold(0, | acc, (k,v) | {
                let boxnum = Self::hash(&k);
                if let Some(slotposition) = &boxes[boxnum as usize].iter().position(|s| s == k) {
                    let focuspower = (boxnum as u32 + 1) * (*slotposition as u32 + 1) * v;
                    eprintln!("{k} has focusing power of [box {boxnum}+1] {focuspower}");
                    acc + focuspower
                } else {
                    panic!("Lense {} is not in a box, should have been remove'd", k);
                }
            });
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
