/*
https://adventofcode.com/2023/day/2
--- Day 2: Cube Conundrum ---
 */


use std::io;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Default)]
struct Solver {
    total_ids: i32,
}

impl Solver {
    // process one text line of input
    fn process(&mut self, l: &str) {
        let parts_id: Vec<&str> = l.split(": ").collect();
        if parts_id.len() != 2 {
            eprintln!("basic \"id:list\" format not found");
            return;
        }
        let id_str = parts_id[0].split("Game ").collect::<Vec<&str>>()[1];
        let id_num = i32::from_str(id_str).unwrap();

        let rounds_it = parts_id[1].split("; ");
        //eprintln!("ID = {} => {:?} ", id_num, subsets_str);
        let mut round_possible = true;
        for k in rounds_it {
            // parse each round "1 green, 2 blue" into hashmap ("blue"=>2, ..)
            // exit as soon as one is found impossible with the problem constraints.

            let mut hm: HashMap<&str,i32> = HashMap::new();
            let colors_it = k.split(", ");
            for c in colors_it {
                let pair: Vec<&str> = c.split(" ").collect(); 
                //eprintln!("color = {}, split {:?}", c, pair);
                
                let color_count = i32::from_str(pair[0]).unwrap();
                let color_name = pair[1];

                hm.insert(color_name, color_count);
            }
            // Now check the min/max value.
            let max_vals = vec![("red",12), ("green", 13), ("blue",14)];
            for max in max_vals {
                match hm.get(max.0) {
                    None => { /* this color was not drawn in this set, equivalent to 0 */ },
                    Some(n) => if *n > max.1 {
                        //eprintln!("In round id {}, color {} was too big at {}",  id_num, max.0, n);
                        round_possible = false;
                        break;
                    },
                }
            }
            if !round_possible  {
                // no need to parse the rest of the rounds in this game_id
                break;
            }
        }
        // all rounds in this game_id have been parsed, or was interrupted by impossibility
        if round_possible {
            //eprintln!("Adding {} as possible round", id_num);
            self.total_ids += id_num;
        } else {
            //eprintln!("Eliminating {} as impossible", id_num);
        }

    }

    // Returns the final string of expected output
    fn result(&self) -> String {
        self.total_ids.to_string()
    }
}

fn main() {

    let mut s = Solver::default();

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
