/*
https://adventofcode.com/2023/day/22
--- Day 22: Sand Slabs ---
(Tetris)
 */


use std::io;
use std::str::FromStr;
//use std::ops::Range;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::cmp;
use std::cmp::Ordering;

#[derive(Copy,Clone,Debug)]
struct Brick {
    corner: (i32,i32,i32), // x,y,z coords of the minimal coord extremity
    len: i32,
    direction: (i32,i32,i32), // one of (1,0,0), (0,1,0) or (0,0,1)
}


impl Brick {
    // Parse a "0,3,187~2,3,187" string
    fn from_str(s:&str) -> Self {
	let (a,b) = s.split_once('~').unwrap();
	let v: Vec<i32> = a.split(',').map(|i| i32::from_str(i).unwrap()).collect();
	let ca = (v[0], v[1], v[2]);
	let v: Vec<i32> = b.split(',').map(|i| i32::from_str(i).unwrap()).collect();
	let cb = (v[0], v[1], v[2]);
	// input data seems to always be ordered in the "smallest~largest" coordinate
	let delta = (cb.0 - ca.0,  cb.1 - ca.1, cb.2 - ca.2);
	let len = 1 + cmp::max(cmp::max(delta.0, delta.1), delta.2);
	if len <= 0 {
	    panic!("Block spec {s} invalid size or order");
	}
	// 1 block bricks are arbitrarily classified as "vertical"
	let direction = ((delta.0 !=0) as i32,
			 (delta.1 !=0) as i32,
			 ((len == 1) || (delta.2 !=0)) as i32);
	Self {
	    corner: ca,
	    len: len,
	    direction: direction,
	}
    }

    // Lower the Z coordinates of this brick to be at (z+1)
    fn move_at_z(&mut self, z: i32) {
	if self.corner.2 <= z {
	    panic!("Brick {:?} is already under z {z}", self.corner);
	}
	self.corner = (self.corner.0, self.corner.1, z+1);
    }

    // Get the explicit list of blocks coordinates directly
    // above this bricks. Will be self.len blocks if the brick
    // is horizontal, or 1 block if vertical.
    fn get_extensions_up(&self) -> Vec::<(i32,i32,i32)> {
	if self.direction.2 != 0 {
	    vec![(self.corner.0, self.corner.1, self.corner.2 + self.len)]
	} else {
	    (0..self.len).map(|n| (self.corner.0 + n * self.direction.0,
				   self.corner.1 + n * self.direction.1,
				   self.corner.2 + n * self.direction.2))
		.collect()
	}
    }

    // Returns the first Z on top of this brick
    fn get_z_above(&self) -> i32 {
	if self.direction.2 != 0 {
	    return self.corner.2 + self.len;
	} else {
	    return self.corner.2 + 1;
	}
    }

    // returns the set of x,y coords of the "shadow"
    fn get_xy(&self) -> HashSet<(i32,i32)> {
	if self.direction.2 != 0 {
	    let mut h = HashSet::<(i32,i32)>::new();
	    h.insert((self.corner.0, self.corner.1));
	    h
	} else {
	    let h:HashSet<(i32,i32)> = 
	    (0..self.len).map(|n| (self.corner.0 + n * self.direction.0,
				   self.corner.1 + n * self.direction.1))
		.collect();
	    h
	}
    }

    // Do 2 bricks share a common x/y block coordinate (independant of z) ?
    fn intersects_xy_brick(&self, other:&Brick) -> bool {
	let sxy = self.get_xy();
	let oxy = other.get_xy();
	!sxy.is_disjoint(&oxy)
    }
}

// Order only by Z. Ignore x/y and z-height when vertical,
// as no intersection will be present.
impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.corner.2.cmp(&other.corner.2)
    }
}
impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Brick {
    fn eq(&self, other: &Self) -> bool {
	self.corner.2 == other.corner.2
    }
}
impl Eq for Brick {}

// Move the bricks on the ground and on top of eachother.
fn settle_bricks(mut b: VecDeque<Brick>) -> Vec<Brick> {
    let mut settled = Vec::<Brick>::new();

    // order the falling bricks from bottom to top
    b.make_contiguous().sort_by(|a, b| b.cmp(a));
    // push down by starting from the bottom bricks, in
    // rough order (similar Z value will always be on different
    // x/y positions so no ambiguity or intersection)
    while !b.is_empty() {
	let mut brick = b.pop_front().expect("b should not be empty here");
    }

    settled
    
}

// Solver for this particular problem

struct Solver {
    total: i32,
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,
        }
    }

    // process input
    fn process_all(&mut self) {

        let mut bricks = VecDeque::<Brick>::new();
	
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
                    let input_clean = input.trim(); // remove the \n
		    bricks.push_back(Brick::from_str(input_clean));
                }
            }
            // must clear for next loop
            input = String::from("");
        }

	eprintln!("Parsed {} bricks: {:?}",
		  bricks.len(),
		  bricks);
	let bricks = settle_bricks(bricks);
	eprintln!("Settled bricks = {:?}", bricks);
	
    }

    fn postprocess(&mut self) {
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

    s.process_all();

    println!("{}", s.result());

}
