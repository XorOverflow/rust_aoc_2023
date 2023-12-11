/*
https://adventofcode.com/2023/day/11
--- Day 11: Cosmic Expansion ---
 */


use std::io;


// Solver for this particular problem

struct Solver {
    total: i32,
    galaxies: Vec<(i32,i32)>,  // coordinates (x,y)
    current_y : i32,
    max_x: i32,
    max_y: i32,
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,
             galaxies : Vec::<(i32,i32)>::new(),
             current_y : 0,
             max_x : 0,
             max_y : 0,
        }
    }

    // process one text line of input
    fn process(&mut self, l: &str) {
        let mut x = 0;
        for c in l.chars() {
            if c == '#' {
                self.galaxies.push((x, self.current_y));
                if x > self.max_x {
                    self.max_x = x;
                }
                if self.current_y > self.max_y {
                    self.max_y = self.current_y;
                }
            }
            x += 1;
        }
        self.current_y += 1;
    }

    fn integral_distance(g1: &(i32,i32), g2: &(i32,i32)) -> i32 {
        // The "shortest distance using only up/down/left/right"
        // is simply the Manhattan distance (giving "Diamond" circles topology)
        return (g1.0 - g2.0).abs() + (g1.1 - g2.1).abs();
    }

    fn postprocess(&mut self) {
        // look for X and Y coordinates without any galaxy.
        let mut empty_x: Vec<bool> = vec![true; 1 + self.max_x as usize];
        let mut empty_y: Vec<bool> = vec![true; 1 + self.max_y as usize];

        for (gx,gy) in &self.galaxies {
            empty_x[*gx as usize] = false;
            empty_y[*gy as usize] = false;
        }

        // Add "1" to the X coordinate of all galaxies that are
        // on the right of an empty space column, for each of those columns.
        // Map this +1.. +2... +3 of each column in the following vectors
        // accumulatinf all expansions:

        let mut expansion_x = Vec::<i32>::new();
        let mut expansion_y = Vec::<i32>::new();
        expansion_x.push(0);
        expansion_y.push(0);

        for n in 0..empty_x.len() {
            let dx;
            if empty_x[n] {
                dx = 1;
            } else {
                dx = 0;
            }
            expansion_x.push(expansion_x.last().unwrap() + dx);
        }

        for n in 0..empty_y.len() {
            let dy;
            if empty_y[n] {
                dy = 1;
            } else {
                dy = 0;
            }
            expansion_y.push(expansion_y.last().unwrap() + dy);
        }

        // now for the galaxies
        for g in &mut self.galaxies.iter_mut() {
            let g0 = g.clone();
            *g = (g.0 + expansion_x[g.0 as usize],
                  g.1 + expansion_y[g.1 as usize]);
            eprintln!("expanding {:?} to {:?}", g0, g);
        }

        // Now compute shortest paths.
        let mut g_it =  self.galaxies.iter();
        while let Some(g1) = g_it.next() {
            // Now iterate on the other galaxies after this one,
            // by continuing from this same iterator position + 1
            let mut g_other = g_it.clone();
            while let Some(g2) = g_other.next() {
                eprintln!("Computing D({:?}--{:?})", g1, g2);
                self.total += Self::integral_distance(g1,g2);
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
