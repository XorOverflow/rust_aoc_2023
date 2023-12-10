/*
https://adventofcode.com/2023/day/10
--- Day 10: Pipe Maze ---
 */


use std::io;


// This puzzle tiles can be described by the two possible exit
// directions relative to our selves, as (x,y) tuples in [-1,0,+1]
struct Connection {
    d: [(isize,isize);2], // order of d[0] and d[1] is not important
}


impl Connection {
    fn from_char(c: char) -> Connection {
        match c {
            '.' => Connection{ d:[(0,0), (0,0)] }, // special case, no direction at all
            'S' => Connection{ d:[(0,0), (0,0)] }, // until we know better
            '-' => Connection{ d:[(-1,0), (1,0)] }, 
            '|' => Connection{ d:[(0,-1), (0,1)] }, 
            'L' => Connection{ d:[(0,-1), (1,0)] }, 
            'J' => Connection{ d:[(0,-1), (-1,0)] }, 
            '7' => Connection{ d:[(-1,0), (0,1)] }, 
            'F' => Connection{ d:[(1,0), (0,1)] }, 
            _ => panic!("{c} is not a valid tile"),
        }
    }

    // If a tile at relative coordinate (from) (-1/0/+1) is connected to us, where does the opposing connection
    // leads to (relative to us) ?
    // For example if coming the tile East of us (+1,0) and we are a "L" ,
    // this will return (0,-1) (up)
    //   ___|  next|__
    //   ___[ self ] from (+1, 0)
    //      |      |
    // Return None when this title is not possibly connected.
    fn next_coordinate_coming_from(&self, from: (isize, isize)) -> Option<(isize, isize)> {
        if from == (0, 0) {
            return None;
        }
        // find the matching entry and return the opposit exit
        for i in 0..=1 {
            if from == self.d[i] {
                return Some(self.d[1-i]);
            }
        }
        return None;
    }
}

// Solver for this particular problem

struct Solver {
    total: i32,
    s_coordinate: (usize,usize),
    map: Vec<Vec<Connection>>,
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,
             s_coordinate: (0,0),
             map: Vec::<Vec::<Connection>>::new(),
        }
    }

    // process one text line of input
    fn process(&mut self, l: &str) {
        let mut line = Vec::<Connection>::new();
        for c in l.chars() {
            if c == 'S' {
                self.s_coordinate = (line.len(), self.map.len());
                eprintln!("S is at {:?}", self.s_coordinate);
            }
            line.push(Connection::from_char(c));
        }
        self.map.push(line);
    }

    // handle the usize/isize convestions for addition.
    fn add_delta_to_position(delta:(isize, isize), position:(usize,usize)) -> (usize, usize) {
        (position.0.checked_add_signed(delta.0).unwrap(),
         position.1.checked_add_signed(delta.1).unwrap())
    }

    fn postprocess(&mut self) {
        let mut loop_follow:(usize,usize) = (0, 0);
        let mut loop_previous:(usize,usize) = self.s_coordinate;
        let mut loop_from:(isize,isize) = (0, 0);
        let mut loop_found = false;

        let d: [(isize,isize);4] = [(1,0), (0,1), (-1,0), (0,-1)];
        for (dx,dy) in d {
            // vectors are index by usize but we need to add a signed integers
            // for the delta +/-1 coordinates. "+" between different types
            // is not allowed in rust so we resort to dedicated functions
            let t_x:usize;
            match self.s_coordinate.0.checked_add_signed(dx) {
                Some(x) => t_x = x,
                None => continue,
            }
            let t_y:usize;
            match self.s_coordinate.1.checked_add_signed(dy) {
                Some(y) => t_y = y,
                None => continue,
            }
            // We checked above for "non negative" but if S is on the right/bottom
            // side it will panic. Good enough for the actual input and samples.
            let adj_tile = &self.map[t_y][t_x]; // XXX index vecs by [y,x] and not [x,y]
            // reverse relative coordinates
            match adj_tile.next_coordinate_coming_from((-dx, -dy)) {
                None => continue, // not connectable tile
                Some((nx, ny)) => {
                    loop_follow = Self::add_delta_to_position((dx,dy), self.s_coordinate);
                    eprintln!("Found the loop start at delta {nx} {ny}, or {:?}", loop_follow);
                    loop_from = (-dx, -dy);
                    loop_found = true;
                    break;
                },
            }
        }
        if !loop_found {
            panic!("No starting loop found");
        }
        // Follow the loop until it reaches back to S
        let mut loop_length = 2; // S and our starting connected tile
        while loop_follow != self.s_coordinate {
            let tile = &self.map[loop_follow.1][loop_follow.0]; // XXX index vecs by [y,x] and not [x,y]
            match tile.next_coordinate_coming_from(loop_from) {
                None => panic!("Loop was broken"),
                Some((nx, ny)) => {
                    loop_follow = Self::add_delta_to_position((nx,ny), loop_follow);
                    loop_from = (-nx, -ny);
                    eprintln!("walking to {},{}", loop_follow.0, loop_follow.1);
                    loop_length += 1;
                },
            }
        }
        loop_length -= 1; // We counted S twice when coming back to it
        eprintln!("Looped back with length {loop_length}");
        self.total = loop_length / 2;
        
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
