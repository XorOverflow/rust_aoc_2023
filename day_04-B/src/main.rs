/*
https://adventofcode.com/2023/day/4

 */


use std::io;
use std::str::FromStr;
use std::collections::HashSet;

// Solver for this particular problem

struct Solver {
    total: usize,
    current_line: usize,
    duplicates: Vec<usize>,  /* number of duplicates of card [n] (including the original one).
                              * card 1 starts at [1] , [0] is ignored
                              */
}


// vec[index] += value
// grows vec as needed to include [index] and initialize it by 0
fn add_value_to_index(vec: &mut Vec<usize>, value: usize, index: usize)  {
    if vec.len() <= index {
        vec.resize(index+1, 0);
    }
    vec[index] += value;
}


impl Solver {
    fn new() -> Self {
        Self{total : 0,
             current_line:0,
             duplicates: Vec::<usize>::new(),
        }
    }

    // process one text line of input
    fn process(&mut self, l: &str) {
        let line_parts: Vec<&str> = l.split(&[':', '|'][..]).collect();
        if line_parts.len() != 3 {
            panic!("Invalid input {}", l);
        }

        // We could parse it from line[0] but it's just simpler like that
        self.current_line += 1;
        // count this original scratch-card
        add_value_to_index(&mut self.duplicates, 1, self.current_line);
        
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

        // Get size of Intersection
        let sames = winning.intersection(&yours).count();
        // duplicate following cards by the count of matched numbers
        if sames > 0 {
            // We may have been duplicated by previous cards,
            // duplicate next cards accordingly
            let factor = self.duplicates[self.current_line];
            for k in (self.current_line+1)..(self.current_line+1+sames) {
                add_value_to_index(&mut self.duplicates, factor, k);
                // Note: by the puzzle description, it is guaranteed that the last
                // cards will not "win" and thus have no risk to duplicate
                // cards number above the real parsed cards, so will not
                // count "virtual" cards in the final total.
                // If not, we would have needed to shrink the vector at self.current_line.
            }
        }

    }


    fn postprocess(&mut self) {
        // count all the cards
        self.total = self.duplicates.iter().sum();
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
