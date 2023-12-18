/*
https://adventofcode.com/2023/day/18
--- Day 18: Lavaduct Lagoon ---
 */


use std::io;
use std::str::FromStr;
use std::boxed::Box;
use std::cmp;


/*
 * Scanline filling can't work with the new dimensions of part 2.
 * Use something like Shoelace formula, or rather Trapezoid formula
 * optimized for axis-parallel edges.
 */




struct Solver {
    total: i64,
}

#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use Direction::*;

#[derive(Clone,Copy,Debug)]
enum TurnDirection {
    TurnLeft,
    TurnRight,
}
use TurnDirection::*;

impl Direction {
    // will panic when impossible combinations (Up + Down) for our use-case
    // to simplify caller match rather that returning Option()
    fn followed_by(&self, newd: Direction) -> TurnDirection {
        match (&self,newd) {
            (Up,Left) => TurnLeft,
            (Up,Right) => TurnRight,
            (Down,Left) => TurnRight,
            (Down,Right) => TurnLeft,
            // anti-symetric versions
            (Left,Up) => TurnRight,
            (Right,Up) => TurnLeft,
            (Left,Down) => TurnLeft,
            (Right,Down) => TurnRight,
            _ => panic!("Illegal turning combination {:?} -> {:?}", self, newd),
        }
    }
}



impl Solver {
    fn new() -> Self {
        Self{total : 0,
        }
    }


    // Compute the area of a closed path using trapezoid formula.
    // Since the path edges are only parallel to X or Y, the formula
    // is very simplified (rectangles and half of them are 0)
    fn get_area(path: &Vec<(Direction, i64)>) -> i64 {
        let mut surface: i64 = 0;
        let mut y:i64 = 0; // arbitrary starting point
        // Project on X.
        // Coordinates are computed with usual cartesian coordinates (Up is Y > 0)
        for (d,l) in path {
            match d {
                Up => y += l,
                Down => y -= l,
                Left => surface -= l * y, 
                Right => surface += l * y, 
            }
        }
        eprintln!("signed surface = {surface}");
        // return absolute value
        surface.abs()
    }


    
    // Construct a new path corresponding to one of the side edge (left or right)
    // of the trench:
    // every time the original path makes a left or right turn, create a
    // corresponding path edge more, or less, long depending on the interior or exterior
    // of the turn.
    // "Interior or Exterior" depends on the global path being in the Direct orientation.
    // If the path is Indirect, than the actual result will be of interior and exterior swapped.
    // XXX check the connectivity between the start and the end
    /*
        Edge growing/keeping/shrinking depends on the turning direction before and after.
        In the case of a Direct oriented path and we want "exterior" edge, there are 4 cases:
        When deciding to create the new edge for segment AB                                
          ----+                    ----+                                                   
         <--+/|        B+-->      <--/ |  ext       B +------>                             
           B| | ext     |\___       B| |___           |\ ____                              
           A| |        A| |          |/               | |  ext                             
         -->+ |     --->+ |          +<----           |/ ‾ ‾                                 
         __ _\|     _____\| ext      A              A +<-------                                      

        left+left    left+right     right+left      right+right
         size+1         size==         size==          size-1

        growing factor is inversed when we want interior rather that exterior
        growing factor is inversed again if the path is Indirect rather than Direct (implicitely)
     */
    fn make_path_edge_at_side(path: &Vec<(Direction, i64)>, exterior:bool) -> Vec<(Direction, i64)> {
        let mut edge_path = Vec::<(Direction, i64)>::new();

        let mut prev_d = path.last().unwrap().0; // used for wrapping around last/first edge
        let first_turn = prev_d.followed_by(path[0].0); // used for wrapping around last edge
        let mut prev_turn = first_turn;
        
        let mut p_it = path.iter();
        let exterior_mul = if exterior { 1 } else { -1 };

        while let Some((edge_d, edge_l)) = p_it.next() {
            // Maybe using windows(2) iterator would have been simpler ?
            // still need to handle the wrapping at start and end.
            let mut next_it = p_it.clone();
            let next_turn = match next_it.next() {
                None => first_turn,
                Some((next_d, _)) => edge_d.followed_by(*next_d),
            };

            let len_change:i64 = match (prev_turn, next_turn) {
                (TurnLeft, TurnLeft) => exterior_mul * 1,  // grow if exterior, shrink on interior
                (TurnRight, TurnRight) => exterior_mul * (-1),  // opposite
                _ => 0,  // keep same length
            };

            edge_path.push((*edge_d, edge_l + len_change));

            prev_d = *edge_d;
            prev_turn = next_turn;
        }

        edge_path
    }

    // process input
    fn process_all(&mut self) {

        let mut path = Vec::<(Direction, i64)>::new();

        let mut input = String::new();
        loop {
            match io::stdin().read_line(&mut input) {
                Err(_) => {
                    panic!("input error, exit");
                }
                Ok(0) => {
                    eprintln!("Eof detected");
                    break;
                },
                Ok(_) => {
                    // extract the final #hexnumber
                    let input_clean = input.trim(); // remove the \n
                    if let Some((_,hexa)) = input_clean.rsplit_once('#') {
                        let hexdistance = &hexa[0..5];
                        // Still not understanding string indexing. 
                        // Arbitrary slices are ok, but not direct byte access for a single char, so..
                        let hexdirection = &hexa[5..6];
                        let d:Direction = match hexdirection {
                            "0" => Right,
                            "1" => Down,
                            "2" => Left,
                            "3" => Up,
                            _ => panic!("Incorrec input string, last hexdigit is not a direction: {}", input_clean),
                        };
                        let l:i64 = i64::from_str_radix(hexdistance, 16).unwrap();

                        eprintln!("Parsed 2 : {:?} for {l}", d);

                        path.push((d,l));
                    } else {
                        eprintln!("Warning input string malformed: no # delimiter in {input_clean}");
                        continue;
                    }
                }
            }
            // must clear for next loop
            input = String::from("");
        }

        eprintln!("Path = {:?}", path);
        Self::get_area(&path);

        // We don't really know if our path is in Direct orientation or not.
        // yes there is probably a formula to get it but it's simpler to
        // test both case and keep the one with the largest of the final surface.
        eprintln!("Creating path edge 1:");
        let edge_exterior = Self::make_path_edge_at_side(&path, true);
        eprintln!("Path = {:?}", edge_exterior);

        eprintln!("Creating path edge 2:");
        let edge_interior = Self::make_path_edge_at_side(&path, false);
        eprintln!("Path = {:?}", edge_interior);

        self.total = cmp::max(  Self::get_area(&edge_exterior), 
                                Self::get_area(&edge_interior) );

    }


    // Returns the final string of expected output
    fn result(&mut self) -> String {
        self.total.to_string()
    }
}

/* common to all problems */
fn main() {

    let mut s = Solver::new();

    s.process_all();
    
    println!("{}", s.result());

}
