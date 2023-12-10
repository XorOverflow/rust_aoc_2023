/*
https://adventofcode.com/2023/day/5
--- Day 5: If You Give A Seed A Fertilizer ---
 */


use std::io;
use std::str::FromStr;

// A single source-dest map from the almanac.
// HashMaps are not possible for this problem (millions of individual k-v) so it uses
// vectors of "range" mapping and manual iteration.
// i64 required, i32 is too small for the puzzle data.
struct GardenMap {
    map: Vec<(i64, i64, i64)>, // destination start, source start, range length
}

impl GardenMap {
    fn new() -> Self {
        Self{ map: Vec::<(i64, i64, i64)>::new(), }
    }
    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn add_mapping_element(&mut self, destination: i64, source: i64, length: i64) {
        self.map.push((destination, source, length));
    }

    // Return the destination value from a source value.
    fn get_mapping_from(&self, source:i64) -> i64 {
        // The map list may be unordered, but not overlapping.
        // So just iterate all of them until found (or not).
        // No need to sort first.
        for r in &self.map {
            // xxx why does "for (d,s,l) in &self.map" half-works but breaks the next range check ?
            let d = r.0;
            let s = r.1;
            let l = r.2;
            if (s..s+l).contains(&source) {
                // apply same delta to the destination range
                return d + (source-s);
            }
        }
        // implicit mapping is identity
        return source;
    }
}



struct Almanac {
    all_maps: Vec<GardenMap>, // All successive maps from the almanac.
}


impl Almanac {
    fn new() -> Self {
        Self{ all_maps: Vec::<GardenMap>::new(), }
    }


    fn add_mapping(&mut self, m: GardenMap) {
        self.all_maps.push(m);
    }

    // start from the first map, and call get_mapping_from until
    // the last map. Return the final "destination" value.
    fn get_recursive_mapping_from(&self, source: i64) -> i64 {
        // No tail-recursion optimisation in rust, just loop.
        let mut dest = source;
        let mut src = source;
        for map in &self.all_maps {
            dest = map.get_mapping_from(src);
            src = dest;
        }
        return dest;
    }
}

// Solver for this particular problem

struct Solver {
    total: i64,
    seeds: Vec<i64>,
    is_parsing_maps: bool, // context for the line-by-line parser
    almanac : Almanac,
    current_map: GardenMap,
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,
             seeds: Vec::<i64>::new(),
             is_parsing_maps : false,
             almanac : Almanac::new(),
             current_map : GardenMap::new(),
        }
    }

    // process one text line of input
    fn process(&mut self, l: &str) {
        if l.is_empty() {
            return;
        }

        if self.is_parsing_maps {
            // Parsing the different maps.

            // Detect a new "map header" ?
            if l.chars().next().unwrap().is_ascii_alphabetic() {
                if !self.current_map.is_empty() {
                    // new header: store old one and change map for future work
                    // (atomically replace our local borrow by a new allocated one)
                    self.almanac.add_mapping(std::mem::replace(&mut self.current_map,
                                                               GardenMap::new())
                    );
                }
                // else: it was just the first header, we already have a map from our new()
                return; // header processed, mapping syntax will be found on next call of process()
            } // else: keep current map

            // should be a 3-number line.
            let mapping: Vec<i64> = l.split_whitespace().map(|s| i64::from_str(s).unwrap()).collect();
            if mapping.len() != 3 {
                panic!("Incorrect formatted mapping {}", l);
            }
            self.current_map.add_mapping_element(mapping[0], mapping[1], mapping[2]);

        } else {
            // parsing the initial seed list
            let seed_list: Vec<&str> = l.split(':').collect();
            self.seeds = seed_list[1].split_whitespace().map(|s| i64::from_str(s).unwrap()).collect();
            self.is_parsing_maps = true;
            return;
        }
    }


    fn postprocess(&mut self) {
        // Store/flush the last mapping being parsed now that we reached end-of-file.
        self.almanac.add_mapping(std::mem::replace(&mut self.current_map,
                                                   GardenMap::new()));
        // Now iterate on all the seeds and get their final "location" mapping, return the lowest.
        self.total = self.seeds.iter().map(|v| self.almanac.get_recursive_mapping_from(*v)).min().unwrap();
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
