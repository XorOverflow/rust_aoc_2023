/*
https://adventofcode.com/2023/day/10
--- Day 10: Pipe Maze ---
 */


use std::io;
use std::fmt;

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
#[derive(Clone)]
enum LoopHint {
    None,
    Segment(bool, bool), // reaches (up, down) (else corner or horizontal)
    Interior, // Only for debug print
}

impl LoopHint {
    fn from_connection(c: &Connection) -> Self {
        // No point in our case to return "None" when connection is ground
        let up = c.d[0].1 == -1 || c.d[1].1 == -1; // any connection up (with y < 0) ?
        let down = c.d[0].1 == 1 || c.d[1].1 == 1; // any down ?
        LoopHint::Segment(up,down)
    }
    fn to_char(&self) -> char {
        match self {
            LoopHint::None => '.',
            LoopHint::Interior => 'O',
            LoopHint::Segment(false,false) => '-',
            LoopHint::Segment(true,false) => '^',
            LoopHint::Segment(false,true) => 'v',
            LoopHint::Segment(true,true) => '|',
        }
    }
}

impl fmt::Debug for LoopHint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoopHint::None =>   write!(f, "."),
            LoopHint::Interior => write!(f, "O"),
            LoopHint::Segment(false,false) => write!(f, "-"),
            LoopHint::Segment(true,false) => write!(f, "^"),
            LoopHint::Segment(false,true) => write!(f, "v"),
            LoopHint::Segment(true,true) => write!(f, "|"),
        }
    }
}


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
        let mut loop_from:(isize,isize) = (0, 0);
        let mut loop_found = false;
        let mut s_up = false; // needed only for loop interior
        let mut s_down = false;

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
                    //break; We don't break because we need to get all connections
                    // of Start for the loop interior algo
                    if dy > 0 {
                        s_down = true;
                    }
                    if dy < 0 {
                        s_up = true;
                    }
                },
            }
        }
        if !loop_found {
            panic!("No starting loop found");
        }

        // To count the interior of the loop, perform the classical
        // "odd/even number of intersections" used to fill a polygon.
        // We redraw the loop alone on a "blank page" and then for each
        // pixel count the number of "loop pixels" on its left:
        // odd = inside, even = outside.
        // The difficult part is when the pixel is aligned to
        // an horizontal part 'F-----J" of the loop, the length is not
        // directly the value to count for odd/even (and there are
        // more complexe cases such as "F-J" is different than "L-J"
        // For this we need to detect when this loop section crosses
        // over the X axis: Each horizontal span of connected tiles,
        // should indicate if its vertical exit point are crossing
        // above--below (=> counts for 1) or stays on the same side
        // (counts for 0). The exit are always on the first and last
        // tile of the span.

        let map_height = self.map.len();
        let map_width = self.map[0].len();
        let mut loop_map:  Vec<Vec<LoopHint>> = vec![vec![LoopHint::None; map_width]; map_height];

        // no need to know the exact value of S, only if it's connecte to top and/or bottom.
        loop_map[self.s_coordinate.1][self.s_coordinate.0] = LoopHint::Segment(s_up, s_down);

        // Follow the loop until it reaches back to S
        while loop_follow != self.s_coordinate {
            let tile = &self.map[loop_follow.1][loop_follow.0]; // XXX index vecs by [y,x] and not [x,y]
            loop_map[loop_follow.1][loop_follow.0] = LoopHint::from_connection(tile);
            match tile.next_coordinate_coming_from(loop_from) {
                None => panic!("Loop was broken"),
                Some((nx, ny)) => {
                    loop_follow = Self::add_delta_to_position((nx,ny), loop_follow);
                    loop_from = (-nx, -ny);
                },
            }
        }
        

        // Now iterate on the map and count the loop vertical intersections
        // to map the "interior"
        
        for k in loop_map { // one slice of map
            let mut left_count = 0;
            let mut in_segment = false;
            let mut prev_up = false;
            let mut prev_down = false;
            // debug
            let mut s = String::new();

            for i in k {
                let mut process = i.clone();
                match i {
                    LoopHint::None => if left_count % 2 == 1 {
                        self.total += 1;
                        // debug display
                        process = LoopHint::Interior;
                    },
                    LoopHint::Segment(false,false) => { // ---
                        // Nothing to do, count not modified
                        // (impossible to reach this state without first seeing
                        // a corner)
                    },
                    LoopHint::Segment(true,true) => { // | 
                        left_count += 1;
                    },
                    LoopHint::Segment(up,down) => {  // any corner
                        if in_segment {  // ending corner
                            if (up && prev_down) || (down && prev_up) {
                                // crossing boundary
                                left_count += 1;
                            }
                            in_segment = false;
                        } else { // starting corner
                            in_segment = true;
                            prev_up = up;
                            prev_down = down;
                        }
                    },
                    LoopHint::Interior => {}, // will never happen
                }
                // debug
                s.push(process.to_char());
            }

            // debug
            eprintln!("{}", s);
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
