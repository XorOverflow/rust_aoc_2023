/*
https://adventofcode.com/2023/day/12
--- Day 12: Hot Springs ---
 */


// Uses an "optimized brute-forcer".
// actually converges in human time (a few minutes instead of centuries)
// but clearly not immediate.
// To handle the 1000 lines input, simpler to parallelize between cores:
// for 32 core (33 lines each),
// # split -l 32 input_A.txt
// # for k i x*; do day_12-B < $k > output_$k & done
// # sum all the output_x* files.
// this will take a few minutes on a 32-cores threadripper (3 or 4 chunks take 4 minutes
// while the others take 1 minute, and one take 5 minutes)


use std::io;
use std::str::FromStr;
use std::collections::HashMap;


// to experiment in algo splitting of the "contiguous group of damaged springs".
// 3 or 2 seems optimal (may depend on each input line)
static SPLIT_UNIT:usize = 2;

// Solver for this particular problem

struct Solver {
    total: i64,
    memo_prefix: HashMap<(String, usize, Vec<i64>), HashMap<usize,i64>>, // god help me
    // The key is the type "condition_state string, max_offset in that string, and a small crc list of 1/2/3 elements.
    memo_hit: i64,  // for debug stats
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,
             memo_prefix: HashMap::<(String, usize, Vec<i64>), HashMap::<usize,i64>>::new(),
             memo_hit: 0,
        }
    }


    // Brute-forcing by counting only 1 by 1 is not possible.
    // Without finding a clever math formula, and keeping a "tree traversal",
    // need at some point to be able to multiply things instead of adding 1
    // to gain a factor of speed.
    // From the input data and manual tries, simply raising the part-1 result
    // to a 5th power doesn't work because concatenation very often allows
    // overlaps and new arrangements to be possible across the boundaries of
    // the part-1 solutions, so this will not do.
    
    // One guess is to split the input data into smaller size chunks, compute
    // the arrangements for each using the trivial method, and be able to
    // multiply them together.
    // Trying to be clever on the Condition Record based on its pattern seems difficult.
    // Anyway, puzzle input data and experiments show that the real combination
    // explosion happens for pathological records full of "?" where there is
    // no pattern anyway.

    // So the split will be done instead based on the "crc" (list of contiguous group)
    // such as (3,2,2,1,1)

    // We want to get sets of splits made of two parts A and B, such as A contains several
    // possible arrangements, and all B of a set start at the same point. This way
    // we can recurse on B, and multiply the results of B by the number of A arrangements.
    // Splitting the crc list only on the first element will never produce identical
    // B starting points because by definition, each possible unique span of #### will alway
    // be of different positions. So the minimal split of CRC is to extract the first two
    // elements.  For example:

    // ????????????...(size M)  3,2,2,1,1
    // Split into (3,2) + (2,1,1)
    // We know that the (2,1,1) must cover 2+1+1 "#" and at least 2 "." (to seperate them)
    // so the "B" part of the condition record is at least 6 chars long, so the A part
    // is at most M-6 chars.
    // Use the brute-force method on input and force it to always end with a "."
    // "???...."x(M-6)   (3,2)
    // This will output possible arrangements such as:
    // .....###.##.
    // ....###..##.  => C1 = 6 arrangement stopping at characters A1 = 12
    // ...###...##.
    // etc
    // ..###.##.
    // .###..##.     => Cn = 3 arrangements stopping at character An = 9
    // ###...##.
    // Collect all possible Stopping A position (12, 9, ..) (with their arrangement count)
    // Now for each one of them:
    // cut off the starting An chars of the initial condition record
    // [...An cut off]
    // recurse with remaining input data:
    //                [??#..??..???]  (2,1,1)
    // get the result number and multiply it by the Cn
    // some those totals together
    // return.
    // recursion stops when CRC is of size 2 or 1, use only the brute-force on the whole input string.


    // For a string containing #/./?, return the length of the minimal
    // prefix substring matching the crc without any "?" and ending with "#.".
    // If no match, returns None.
    // "..###..#...??" (3,1) => Some(9)
    //  123456789
    // "..###..#??" (3,1) => None
    // "..###..#" (3,1) => None

    fn get_exact_prefix(condition_state: &str, crc: &Vec<i64>) -> Option<usize> {
        let mut k:usize = 0;
        let mut segment_size:i64 = 0;
        let mut crc_size:i64 = 0;
        let mut in_segment = false;
        let mut crc_it = crc.iter();

        match crc_it.next() {
            None => panic!("get_exact_prefix must be called with non-empty crc vector"),
            Some(s) => crc_size = *s, // expect the first span of # soon
        }

        //eprintln!("  get exact {condition_state} for {:?}", crc);
        // walk the string and compare it in parallel to the expected
        // spans of crc. There may be some magical one-liner with split/zip/whatever
        // but I don't know. Anyway we have also to handle the final .
        for c in condition_state.chars() {
            match c {
                '?' => return None,
                '.' => {
                    k += 1;
                    if !in_segment {
                        continue;
                    }
                    if segment_size == crc_size {
                        in_segment = false;
                        match crc_it.next() {
                            None => {
                                //eprintln!("   String {condition_state} matches {:?} for the first {k} chars", crc);
                                return Some(k); // we matched all of the crc, we're done.
                            },
                            Some(s) => crc_size = *s, // expect a new span of # later
                        }
                        continue;
                    } else {
                        // The finished ## span was the wrong size.
                        //eprintln!("   wrong span {segment_size}/{crc_size} at {k}");
                        return None;
                    }
                },
                '#' => {
                    k += 1;
                    if in_segment {
                        segment_size += 1;
                        if segment_size > crc_size {
                            // ### is too large.
                            //eprintln!(   "overflow span {segment_size}/{crc_size} at {k}");
                            return None;
                        }
                    } else {
                        in_segment = true;
                        segment_size = 1;
                    }
                },
                _ => panic!("Malformed input"),
            }
        }
        // We finished parsing the string without consuming all the iterator
        // or reaching a final "."
        // the pattern matches only the beginning, so not a complete prefix.
        //eprintln!("   Finish, not matched");
        return None;
        
    }


    // brute_find_all_prefix + argt_recursive_test are still too "bruteforcy" and probably recomputes
    // the same similar things over and over (via different paths).
    // We need to memoize a few results to speed-up again.

    // "public" entry point
    fn find_all_prefix(&mut self, condition_state: &str, max_offset: usize, crc: &Vec<i64>, level: i32) -> HashMap<usize,i64> {
        let key = (String::from(condition_state), max_offset, crc.clone());
        if let Some(v) = self.memo_prefix.get(&key) {
            //eprintln!("memo find prefix HIT");
            self.memo_hit += 1;
            return v.clone();
        } else {
          let v = self.brute_find_all_prefix(condition_state, max_offset, crc, level);
          self.memo_prefix.insert(key, v.clone());
          return v;
        }
    }

    
    // Find all the arrangements of the crc list, starting from the beginning of the string
    // up to some index inside, and ending with "#."
    // The return value maps the valid "size" of prefix-strings with their number of arrangements.
    // those size include the final "."
    // This functions works only for reasonably small input strings and crc vectors.
    fn brute_find_all_prefix(&mut self, condition_state: &str, max_offset: usize, crc: &Vec<i64>, level: i32) -> HashMap<usize,i64> {

        if level == 0 {
            //eprintln!("Called top-level brute for {condition_state} [cap {max_offset}] {:?}", crc);
        }
        let mut hm = HashMap::<usize,i64>::new();

        if let Some((left, right)) = condition_state.split_once('?') {
            if left.len() == 0 {
                // '?' at first char, we have done nothing yet, nothing to check
                // (would panic when indexing chars inside)
                //eprintln!("Trying {condition_state} for {:?}", crc);
            } else {
                match Self::get_exact_prefix(condition_state, &crc) {
                    Some(n) => {
                        //eprintln!("recursed {condition_state} matches {:?} at length {n} ({left}) ", &crc);
                        hm.insert(n, 1);
                        return hm; // actual positive result
                    },
                    _ => {}, // continue
                }

                if left.find('#') == None && left.len() >= max_offset {
                    // We got too far, no need to iterate more
                    //eprintln!("early return for'{left}/?/{right}' over {max_offset}");
                    return hm; // early empty
                }
                //eprintln!("checking {condition_state} for {:?}", crc);
                // count the damaged spans we have so far before the first '?'.
                let damaged_left:Vec<&str> = left.split('.').collect();
                let mut damaged_left:Vec<i64> = damaged_left.iter().map(|s| s.len() as i64).filter(|len| *len != 0).collect();

                // as crc is a prefix, it will ignore all additional ".#" we may create when replacing "?".
                if damaged_left.len() > crc.len() {
                    // This means we have recursed into a wrong direction.
                    return hm; // empty
                }
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
                        return hm; // Early return, impossible
                    }
                } else {
                    // last value can be >=, others must match
                    let last = crc_begin.len()-1;
                    if damaged_left[last] > crc_begin[last]  {
                        //eprintln!("partial test (2,>) at {condition_state} can not match {:?}", crc);
                        return hm;
                    }
                    // now compare exactly the rest of the elements
                    crc_begin.pop();
                    damaged_left.pop();
                    if crc_begin != damaged_left {
                        //eprintln!("partial test at {condition_state} can not match {:?}:: d_left ={:?}, c_begin={:?}", crc, damaged_left, crc_begin);
                        //eprintln!("was {condition_state}, {:?}", crc);
                        return hm; // Early return, impossible
                    }
                }
            }

            // test both replacement of our '?' if we still have the budget  (micro-opti ?)
            if true {
                let new_condition = format!("{left}#{right}");
                //eprintln!("recursing # into {new_condition}");
                // at this point hm is still empty, we can replace it
                hm = self.find_all_prefix(&new_condition, max_offset, crc,level+1);
            }
            
            if true {
                let new_condition = format!("{left}.{right}");
                //eprintln!("recursing . into {new_condition}");
                if hm.len() == 0 {
                    hm = self.find_all_prefix(&new_condition, max_offset, crc, level+1);
                } else {
                    // add values of both # and . versions
                    let add_hm = self.find_all_prefix(&new_condition, max_offset, crc, level+1);
                    for (k,v) in add_hm {
                        if let Some(v0) = hm.get(&k) {
                            // add to the existing hashmap
                            hm.insert(k, v + v0);
                        } else {
                            hm.insert(k, v);
                        }
                    }
                }
            }

            return hm; // This is the merge/add of all the inner recursions results.


        } else {
            //eprintln!("Reached terminal {condition_state}");
            // terminal string with no '?'
            // we are leaf: check if we match crc.
            match Self::get_exact_prefix(condition_state, &crc) {
                None => {
                    //eprintln!("terminal {condition_state} doesn't match {:?}", &crc);
                } , // found 0 arrangement
                Some(n) => {
                    //eprintln!("terminal {condition_state} matches {:?} at length {n} ", &crc);

                    hm.insert(n, 1); },  // found 1 arrangement
            }
            return hm;
        }
    }

    // after we finish a complete brute_find_all_prefix / get_exact_prefix, the rest of the string
    // should not match any more ### segment.
    fn check_empty_postfix(condition_state: &str, after: usize) -> bool {
        if condition_state.len() == after + 1 {
            // nothing
            return  true;
        } else {
            // Not OK: any # would need at least another (1) in the crc
            // OK: nothing but . and maybe ? (that will match with the empty crc
            // by being all replaced by '.', so only 1 possible case
            // which will not change the previous number of arrangements)
            return ! condition_state[after..].contains('#');
        }
    }


    fn argt_recursive_test(&mut self, condition: &str, crc: &Vec<i64>, level: i32) -> i64 {
        // split the crc into two parts (if possible)
        let mut crc_head = crc.clone();
        let crc_split_index;
        if crc.len() <= SPLIT_UNIT {
            crc_split_index = crc.len();  // this will be our final recursion
        } else {
            crc_split_index = SPLIT_UNIT;
        }
        let crc_tail = crc_head.split_off(crc_split_index); // may be empty if final recursion
        //eprintln!("Split by {:?} and {:?}", crc_head, crc_tail);

        let total = condition.len();
        // compute the minimal number of characters occupied by all crc():
        let damaged:i64 = crc.iter().sum(); // all # characters
        let operational_min = crc.len() - 1; // all intervals between # must count at least one '.'
        // The first "#"  must be inside condition [..max_offset] as anything after will
        // not have enough room for the rest.
        //eprintln!("trying {total} - {damaged} - {operational_min}");
        let occupied_min = damaged as usize + operational_min;
        if occupied_min >= total {
            // previous "map" already went too far, no room left.
            return 0;
        }
        // else, may be possible
        let max_offset:usize = total - occupied_min;

        // variable max_offset messes with Memoization.
        // Since speedup my memo is greater than the smaller speedup by offset early exit alone,
        // we don't use it in final version.
        //let max_offset = total - occupied_min;
        
        

        let hm = self.find_all_prefix(condition,
                                             max_offset,
                                             &crc_head,
                                             if level<= 1 { 0 } else {level}
        );

        if level<= 1 {
            //eprintln!("At @[{level}] Prefixes map = {:?}", hm);
        }
        
        let mut arrangements = 0;
        for (k,v) in hm {

            if crc_tail.is_empty() {
                // nothing else to find
                if Self::check_empty_postfix(&condition, k) {
                    // actually nothing else wants to be found
                    arrangements += v ;
                }
                // else: dead-end search (0 valid new arrangements)
            } else {
                // factorize this prefix with the next recursion level on
                // the rest of the string and the rest of the crc
                let (_, condition_tail) = condition.split_at(k);
                arrangements += v * self.argt_recursive_test(condition_tail, &crc_tail, level+1);
            }
        }
        //eprintln!("Level: {arrangements}");
        return arrangements;
    }

    // count the possible arrangements
    fn arrangements(&mut self, condition: &str, crc: &Vec<i64>) -> i64 {
        // Add a terminal "." to ensure the invariant that any pattern can end
        // with "#." and not just "#", even at the end of original input with "#" or "?"
        // Any leading or trailing sequence of fixed '.' does not change the possible
        // permutations and the final result.
        let condition = format!("{condition}.");

        return self.argt_recursive_test(&condition, crc, 0);
    }
    
    // process one text line of input
    fn process(&mut self, l: &str) {
        eprintln!("Parsing {l}");
        if let Some((condition,crc)) = l.split_once(" ") {
            // try to multiply by 5 the input and check if the brute-force is still working...
            // It would have been nice if it was just giving "part_1 ^ 5" but the additionnal
            // "?" actually allows new combinations between copies.
            
            let condition = format!("{}?{}?{}?{}?{}", condition, condition, condition,
                                    condition, condition);
            let crc = format!("{},{},{},{},{}", crc, crc, crc, crc, crc);


            //let condition = format!("{}?{}?{}", condition, condition, condition);
            //let crc = format!("{},{},{}", crc, crc, crc);

            let crc:Vec<i64> = crc.split(',').map(|x| i64::from_str(x).unwrap()).collect();
            let arg = self.arrangements(&condition, &crc);
            eprintln!("{} : => argt {} ({} memo hits)", l, arg, self.memo_hit);
            // Tried to reuse the memo between samples lines. Ended up filling all my memory
            // after 200 lines.
            self.memo_prefix.clear(); //
            self.memo_hit = 0;
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
