/*
https://adventofcode.com/2023/day/3
--- Day 3: Gear Ratios ---
 */


use std::io;
use std::str::FromStr;
use regex::Regex;
use std::ops::Range;


// Solver for this particular problem

// part-number and its range in a line
// Range(0..1) is only 1 char wide
#[derive(Debug)]
pub struct NumberCoordinates {
    val: i32,
    r: Range<usize>,
}

struct Solver {
    total: i32,
    re: Regex,
    gears: Vec<Vec<usize>>,  // (potential gears) for each line, the list of X coordinates
    parts: Vec<Vec<NumberCoordinates>>,  // for each line, the list of parts value and coordinates
    
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,
             // parts-numbers (numbers) and symbols.
             // for part B we only care about symbol '*'
             re : Regex::new("([0-9]+)|([*])").unwrap(),
             gears : Vec::<Vec<usize>>::new(),
             parts : Vec::<Vec<NumberCoordinates>>::new(),
        }
    }

    // process one text line of input
    fn process(&mut self, l: &str) {
        // re.captures(l) would only return 1 match (the first number or first symbol)
        // Instead get all successive non-overlapping matched patterns with find_iter()
        // https://docs.rs/regex/latest/regex/struct.Regex.html#method.find_iter
        // Note the find_iter only returns strings and position, but loses the
        // matching group index from the regex. THis could be obtained by captures_iter()
        // instead but this comes with other complications.

        let mut line_gears = Vec::<usize>::new();
        let mut line_parts = Vec::<NumberCoordinates>::new();
        
        for m in self.re.find_iter(l) {
            if m.is_empty() {
                continue;
            }
            // Have to find again if it was matching a number or a symbol.
            let s = m.as_str();
            if s.chars().next().unwrap().is_ascii_digit() {
                // numeric part-number
                let nc = NumberCoordinates {
                    val: i32::from_str(s).unwrap(),
                    r: m.range(),  // returns byte offset on utf8 but ASCII input makes is identical to grapheme range
                };
                line_parts.push(nc);
            } else {
                // 1-char symbol (we actually don't care about its character)
                line_gears.push(m.start());
            }
        }
        //eprintln!("Parsed vecs: parts = {:?}, symbs = {:?}",
        //          line_parts, line_gears);

        self.gears.push(line_gears);
        self.parts.push(line_parts);
    }


    fn postprocess(&mut self) {
        // Reverse the part/symbols order test from A.
        let empty_parts = Vec::<NumberCoordinates>::new();

        for line in 0..self.gears.len() {
            let prev_parts: &Vec<NumberCoordinates>;
            let next_parts: &Vec<NumberCoordinates>;
            if line == 0  {
                prev_parts = &empty_parts;
            } else {
                prev_parts = self.parts.get(line-1).unwrap();
            }
            
            if line+1 >= self.parts.len() {
                next_parts = &empty_parts;
            } else {
                next_parts = self.parts.get(line+1).unwrap();
            }

            let this_parts = self.parts.get(line).unwrap();
            let all_parts = vec![&prev_parts, &this_parts, &next_parts];

            // gears candidates. Have now to check exactly 2 adjacent parts
            let gears = self.gears.get(line).unwrap();
            for x in gears {
                // iterate over all "parts number" on the surrounding lines.
                // count those on "adjacent" coordinate
                
                let mut adjacents = 0;
                let mut gear_ratio = 1;
                'allparts: for pl in &all_parts {
                    for p in **pl {
                        // extend the range to catch the symbols on corners
                        let extr: Range<i32> = std::ops::Range {
                            start: (p.r.start as i32)-1,
                            end: (p.r.end as i32)+1
                        };
                        let extx = *x as i32;
                        if extr.contains(&extx) {
                            adjacents += 1;
                            if adjacents > 2 {
                                break 'allparts;
                            } else {
                                gear_ratio *= p.val;
                            }
                            
                        }
                    }
                } // all parts
                if adjacents == 2 {
                    self.total += gear_ratio;
                }
            }
        }
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
