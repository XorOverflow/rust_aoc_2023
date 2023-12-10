/*
https://adventofcode.com/2023/day/5
--- Day 5: If You Give A Seed A Fertilizer ---
 */


/*
 * Dev notes:
 * initial implem was based on the sample input and though it would be easier to work backward
 * from a list of location in ascending order until it mapped to a starting seed, and used explicit
 * 1-1 mappings in a HashMap.
 * Actual puzzle input rendered both assumptions invalid due to the size of the values.
 */


use std::io;
use std::str::FromStr;
//use regex::Regex;
use std::ops::Range;
use std::collections::{HashMap,HashSet};


struct GardenMap {
    all_maps: Vec<HashMap<i32,i32>>, // All successive maps from the almanac.
}


impl GardenMap {
    fn new() -> Self {
        Self{ all_maps: Vec::<HashMap<i32,i32>>::new(), }
    }

    fn get_new_map() -> HashMap<i32,i32> {
        HashMap::<i32,i32>::new()
    }

    fn add_mapping_element(m: &mut HashMap<i32,i32>, destination: i32, source: i32, length: i32) {
        // XXX nope: the actual dataset uses huge ranges so we can't just explicit
        // each individual mapping.
        for k in 0..length {
            // We map in by using "destination" as the key and not the value.
            // This will make the final reverse search easier.
            m.insert(destination+k, source+k);
        }
    }

    fn add_mapping(&mut self, m: HashMap<i32,i32>) {
        self.all_maps.push(m);
    }

    // Check in the all_maps[map_number] and returns the "source" value which maps
    // to "destination". If not explicitely associated, return the same numeric value.
    fn get_reverse_mapping(&self, map_number: usize, destination: i32) -> i32 {
        let map = &self.all_maps[map_number];
        match map.get(&destination) {
            Some(s) => return *s,
            None => return destination,
        };
    }

    // start from the last map, and get_reverse_mapping until the first map.
    fn get_recursive_reverse_mapping(&self, destination: i32) -> i32 {
        // No tail-recursion optimisation, just loop.
        let mut map_number = self.all_maps.len()-1;
        let mut dest = destination;
        let mut src: i32;
        loop {
            src = self.get_reverse_mapping(map_number, dest);
            if map_number > 0 {
                dest = src;
                map_number -= 1;
            } else {
                return src;
            }
        }
    }
}
// Solver for this particular problem

struct Solver {
    total: i32,
    seeds: HashSet<i32>,
    parsing_maps: bool,
    garden_map : GardenMap,
    current_map: HashMap<i32,i32>,
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,
             seeds: HashSet::<i32>::new(),
             parsing_maps : false,
             garden_map : GardenMap::new(),
             current_map : GardenMap::get_new_map(),
        }
    }

    // process one text line of input
    fn process(&mut self, l: &str) {
        if l.is_empty() {
            return;
        }

        if self.parsing_maps {
            // Parsing the different maps.

            // Detect a new "map header" ?
            if l.chars().next().unwrap().is_ascii_alphabetic() {
                if self.current_map.len() != 0 {
                    // new header: store old one and change map for future work
                    // (atomically replace our local borrow by a new allocated one)
                    self.garden_map.add_mapping(std::mem::replace(&mut self.current_map,
                                                                  GardenMap::get_new_map())
                    );
                }
                // else: it was just the first header, we already have a map
            } // else: keep current map

            // should be a 3-number line.
            let mapping: Vec<i32> = l.split_whitespace().map(|s| i32::from_str(s).unwrap()).collect();
            if mapping.len() != 3 {
                panic!("Incorrect formatted mapping {}", l);
            }
            GardenMap::add_mapping_element(&mut self.current_map, mapping[0], mapping[1], mapping[2]);

        } else {
            // parsing the initial seed list
            let seed_list: Vec<&str> = l.split(':').collect();
            self.seeds = seed_list[1].split_whitespace().map(|s| i32::from_str(s).unwrap()).collect();
            self.parsing_maps = true;
            return;
        }
    }


    fn postprocess(&mut self) {
        // Store/flush the last mapping being parsed now that we reached end-of-file.
        self.garden_map.add_mapping(std::mem::replace(&mut self.current_map,
                                                      GardenMap::get_new_map())

        // Now we travel backwards from the locations to the seeds.
        // XXX we don't have an explicit list of valid locations; the last map
        // could just consider all humidity-to-location from 1 to 10e99 as implicit so we don't
        // know the exact range to test....
        
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
