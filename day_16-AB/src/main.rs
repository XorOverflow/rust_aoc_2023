/*
https://adventofcode.com/2023/day/16
--- Day 16: The Floor Will Be Lava ---
Energize !
 */


use std::io;
use std::cmp;

// Solver for this particular problem

#[derive(Clone,Copy)]
enum Tile {
    Empty,
    SplitterH,
    SplitterV,
    MirrorSlash,
    MirrorAnti,
}
use Tile::*;

#[derive(Clone,Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use Direction::*;

// For transfer, mirror or splitting:
// Indicate a single direction or a split directions pair.
#[derive(Clone,Copy)]
enum AnyDirections {
    OneDirection(Direction),
    TwoDirections(Direction,Direction),
}
use AnyDirections::*;

struct Solver {
    total: i32,
    map: Vec<Vec<Tile>>,
    //energy: Vec<Vec<i32>>,  // 
    directions: Vec<Vec<u8>>,  // for each tile, a bitmask indicating if a
    // beam has entered it with one of the four directions.
    // non-zero indicated "energized" status, exact value indicate end of recursion
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,
             map : Vec::new(),
             directions: Vec::new(),
        }
    }

    fn get_tile(&self, x:usize, y:usize) -> Tile {
        self.map[y][x]
    }

    // return true if the new bits are "new", 
    // false if they were already set.
    fn update_direction_bits(&mut self, x:usize, y:usize, b:u8) -> bool {
        let v = self.directions[y][x];
        if (v & b) == b {
            false
        } else {
            self.directions[y][x] |= b;
            true
        }
            
    }

    
    // process one text line of input
    fn process(&mut self, l: &str) {
        let line:Vec<Tile> = l.chars().map(|c|
                                           match c {
                                               '.' => Empty,
                                               '-' => SplitterH,
                                               '|' => SplitterV,
                                               '/' => MirrorSlash,
                                               '\\' => MirrorAnti,
                                               _ => Empty,
                                           }).collect();
        self.map.push(line);
    }

    // If splitting: return the two split directions;
    // if not splittiong: return the single new direction
    fn tile_to_directions(t: Tile, d: Direction) -> AnyDirections {
        match t {
            Empty => OneDirection(d),
            SplitterH => match d {
                Left | Right => OneDirection(d),
                Up | Down => TwoDirections(Left, Right),
            },
            SplitterV => match d {
                Left | Right => TwoDirections(Up, Down),
                Up | Down => OneDirection(d),
            },
            MirrorSlash => match d {
                Left => OneDirection(Down),
                Right => OneDirection(Up),
                Up => OneDirection(Right),
                Down => OneDirection(Left),
            },
            MirrorAnti => match d {
                Left => OneDirection(Up),
                Right => OneDirection(Down),
                Up => OneDirection(Left),
                Down => OneDirection(Right),
            },
        }
    }

    fn direction_to_bitmask(d: Direction) -> u8 {
        match d {
            Left => 1,
            Right => 2,
            Up => 4,
            Down => 8,
        }
    }
    
    // move_by() and out_of_bounds() could probably be merged into
    // a move_by() -> Option()  (none when OOB ?)
    fn move_by(x: i32, y:i32, d: Direction) -> (i32, i32) {
        match d {
            Left =>  (x-1, y),
            Right => (x+1, y),
            Up =>    (x, y-1),
            Down =>  (x, y+1),
        }
    }

    fn out_of_bounds(&self, x: i32, y:i32) -> bool {
        (x < 0)
            || (y < 0)
            || (y >= self.map.len() as i32)
            || (x >= self.map[0].len() as i32)
    }
    
    // Ray enter (x,y) tile with direction d.
    // Recursively follow when splitting and update path.
    fn follow_ray(&mut self, x: usize, y:usize, mut d: Direction) {

        // Convert to signed to detect underflow < 0
        let mut x:i32 = x as i32;
        let mut y:i32 = y as i32;
        loop {
            let d8 = Self::direction_to_bitmask(d);
            if !self.update_direction_bits(x as usize, y as usize, d8) {
                //eprintln!("Beam already reached {x},{y} via direction {d8}, break");
                break;
            }
            let new_direction = Self::tile_to_directions(self.get_tile(x as usize,y as usize), d);
            match new_direction { // follow single beam
                OneDirection(d1) => {
                    (x,y) = Self::move_by(x, y, d1);
                    if self.out_of_bounds(x,y) {
                        //eprintln!("Ray exits at {x},{y}");
                        break;
                    }
                    d = d1;
                }, 
                TwoDirections(d1,d2) => {  // follow split beam
                    let (x1,y1) = Self::move_by(x, y, d1);
                    if self.out_of_bounds(x1,y1) {
                        //eprintln!("split Ray 1 exits at {x1},{y1}");
                    } else {
                        self.follow_ray(x1 as usize, y1 as usize, d1);
                    }
                    let (x2,y2) = Self::move_by(x, y, d2);
                    if self.out_of_bounds(x2,y2) {
                        //eprintln!("split Ray 2 exits at {x2},{y2}");
                    } else {
                        self.follow_ray(x2 as usize, y2 as usize, d2);
                    }
                    break;
                },
            }
        }
    }
    
    fn postprocess_1(&mut self) {
        self.directions.resize(self.map.len(), Vec::new());
        let width = self.map[0].len();
        for row in self.directions.iter_mut() {
            row.resize(width, 0);
        }

        // Enter the top corner coming from top-left,
        // and follow recursively.
        self.follow_ray(0, 0, Right);

        for row in &self.directions {
            self.total += row.iter().fold(0, |acc, v| if *v == 0 { acc } else { acc +1 } );
            // debug
            eprintln!("[] = {:?}", row);
        }
    }


    fn reset_directions_map(&mut self) {
        let width = self.map[0].len();
        for row in self.directions.iter_mut() {
            row.clear();
            row.resize(width, 0);
        }
    }

    fn get_energized_count(&self) -> i32 {
        let mut t = 0;
        for row in &self.directions {
            t += row.iter().fold(0, |acc, v| if *v == 0 { acc } else { acc +1 } );
        }
        t
    }
        
    // part 2: try all edge tiles
    fn postprocess_2(&mut self) {
        let mut max_total:i32 = 0;

        self.directions.resize(self.map.len(), Vec::new());

        let right_col = self.map[0].len()-1;
        let bottom_row = self.map.len()-1;
        // test top and bottom rows
        for xstart in 0..=right_col {
            self.reset_directions_map();
            self.follow_ray(xstart, 0, Down);
            max_total = cmp::max(max_total, self.get_energized_count());

            self.reset_directions_map();
            self.follow_ray(xstart, bottom_row, Up);
            max_total = cmp::max(max_total, self.get_energized_count());
        }

        // test left and right columns
        for ystart in 0..=bottom_row {
            self.reset_directions_map();
            self.follow_ray(0, ystart, Right);
            max_total = cmp::max(max_total, self.get_energized_count());

            self.reset_directions_map();
            self.follow_ray(right_col, ystart, Left);
            max_total = cmp::max(max_total, self.get_energized_count());
        }

        self.total = max_total;


    }

    // Returns the final string of expected output
    fn result(&mut self) -> String {
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

    if let Some(_) = std::env::args().find(|s| s == "-2") {
        s.postprocess_2();
    } else {
        s.postprocess_1();
    }
    

    println!("{}", s.result());

}
