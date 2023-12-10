/*
https://adventofcode.com/2023/day/2
--- Day 2: Cube Conundrum ---
 */


use std::io;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Default)]
struct Solver {
    total_powers: i32,
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
        let mut max_vals: HashMap<&str,i32> = HashMap::new();
        max_vals.insert("red", 0);
        max_vals.insert("green", 0);
        max_vals.insert("blue", 0);

        for k in rounds_it {
            // parse each round "1 green, 2 blue" and
            // update the corresponding max value

            let colors_it = k.split(", ");
            for c in colors_it {
                let pair: Vec<&str> = c.split(" ").collect(); 
                //eprintln!("color = {}, split {:?}", c, pair);
                
                let color_count = i32::from_str(pair[0]).unwrap();
                let color_name = pair[1];

                match max_vals.get(color_name) {
                    None => { /* not red/green/blue ? */ },
                    Some(n) => if *n < color_count {
                        //update the max in the hashmap
                        max_vals.insert(color_name, color_count);
                    },
                }
                
            }
        }
        // all rounds in this game_id have been parsed, compute the "power" of this game
        let mut power = 1;
        for (color,count) in max_vals.iter() {
            power *= count;
        }
        eprintln!("game id {} power = {}", id_num, power);
        self.total_powers += power;
    }

    // Returns the final string of expected output
    fn result(&self) -> String {
        self.total_powers.to_string()
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
