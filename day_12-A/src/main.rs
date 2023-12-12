/*
https://adventofcode.com/2023/day/12

 */


use std::io;
use std::str::FromStr;
use std::ops::Range;


// Solver for this particular problem

struct Solver {
    total: i64,
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,
        }
    }


    // Bruteforcing it on iterating on "N" bits = the count of "?".
    // There are some early cut-offs where we can detect that the current starting
    // branch can't possibly reach any solution, but it will still need to
    // iterate 1 by 1 all the valid arrangements.
    
    // Perform a recursive  search of possible mappings of "?" and return the totals.
    // Additional states are passed to avoid recomputing some trivial things over.
    fn argt_recursive_test(condition_state: String,
                           damaged_state: i64, operational_state: i64,
                           damaged_total:i64, operational_total:i64, crc: &Vec<i64>,) -> i64 {

        if let Some((left, right)) = condition_state.split_once('?') {
            if left.len() == 0 {
                // '?' at first char, we have done nothing yet, nothing to check
                // (would panic when indexing chars inside)
                //eprintln!("Trying {condition_state} for {:?}", crc);
            } else {
                // count the damaged spans we have so far before the first '?'.
                let damaged_left:Vec<&str> = left.split('.').collect();
                let mut damaged_left:Vec<i64> = damaged_left.iter().map(|s| s.len() as i64).filter(|len| *len != 0).collect();
                // if the last span was just before '?' then it can extend more in the next iteration.
                // else (there is a '.' explicitely cutting it) the last span is at its exact final value.
                let last_char = left.chars().last().unwrap(); // we could also force an indexing to len()-1 as it's ascii and not utf8
                
                let mut crc_begin:Vec<i64> = crc.clone(); // fixme: no better way to extract "view" ? chunks() gives a splice
                // with non-working pop() or comparison with damaged_left later.
                crc_begin.truncate(damaged_left.len()); // split at the first elements
                if last_char == '.' {
                    // all values must match
                    if crc_begin != damaged_left {
                        //eprintln!("partial test (1,==) at {condition_state} can not match {:?}", crc);
                        return 0; // Early return, impossible
                    }
                } else {
                    // last value can be >=, others must match
                    let last = crc_begin.len()-1;
                    if damaged_left[last] > crc_begin[last]  {
                        //eprintln!("partial test (2,>) at {condition_state} can not match {:?}", crc);
                        return 0;
                    }
                    // now compare exactly the rest of the elements
                    crc_begin.pop();
                    damaged_left.pop();
                    if crc_begin != damaged_left {
                        //eprintln!("partial test at {condition_state} can not match {:?}", crc);
                        return 0; // Early return, impossible
                    }
                }
            }
            //eprintln!("partial test at {condition_state} : proceeds to testing");
        } else {
            // terminal string with no '?'
            // we are leaf: check if we match crc.
            // split around contiguous '.' (remove empty splits)
            let damaged_parsed:Vec<&str> = condition_state.split('.').collect();
            let damaged_parsed:Vec<i64> = damaged_parsed.iter().map(|s| s.len() as i64).filter(|len| *len != 0).collect();
            if damaged_parsed == *crc {
                eprintln!("found {condition_state} is OK");
                return 1;
            } else {
                //eprintln!("final recursion to {condition_state} is not matching");
                return 0;
            }
        }

        let mut total:i64 = 0;

        // We don't want to actually enumerate on all "?", only the first one.
        // the recursive call will do the next ones after.
        // FIXME this was written first, before checking the range with the split_once() above;
        // we should instead reuse that split() directly instead of searching for ? again.
      
        for (i,c) in condition_state.chars().enumerate() {
            if c != '?' {
                continue;
            }
            // try # and . depending on the remaining budget
            let mut new_condition = condition_state.clone();
            if damaged_state < damaged_total {
                new_condition.replace_range( i..i+1, "#");
                total += Self::argt_recursive_test(new_condition,
                                                   damaged_state+1, operational_state,
                                                   damaged_total, operational_total, crc);
            }
            if operational_state < operational_total {
                // fixme why need to clone it again instead of calling replace_range() again
                // on the same ? borrow checker disallows
                new_condition = condition_state.clone();
                new_condition.replace_range( i..i+1, ".");
                total += Self::argt_recursive_test(new_condition,
                                                   damaged_state, operational_state+1,
                                                   damaged_total, operational_total, crc);
            }
            // We found our next '?', break now
            break;
        }
        return total;
    }
    
    // count the possible arrangements
    fn arrangements(condition: &str, crc: &Vec<i64>) -> i64 {
        let total = condition.len() as i64;
        let damaged = crc.iter().sum();
        let operational = total - damaged;
        let damaged_state = condition.chars().filter(|c| *c == '#').count() as i64;
        let operational_state = condition.chars().filter(|c| *c == '.').count() as i64;
        return Self::argt_recursive_test(String::from(condition), damaged_state, operational_state,
                                         damaged, operational, crc);
    }
    
    // process one text line of input
    fn process(&mut self, l: &str) {
        if let Some((condition,crc)) = l.split_once(" ") {
            let crc:Vec<i64> = crc.split(',').map(|x| i64::from_str(x).unwrap()).collect();
            let arg = Self::arrangements(&condition, &crc);
            eprintln!("{} : => argt {}", l, arg);
            self.total += arg;
        } else {
            panic!("format");
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
