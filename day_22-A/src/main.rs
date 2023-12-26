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

    // Lower the Z coordinates of this brick to be at "z"
    fn move_at_z(&mut self, z: i32) {
	if self.corner.2 < z {
	    eprintln!("Brick {:?} is already under z {z}", self.corner);
	    return;
	}
	self.corner = (self.corner.0, self.corner.1, z);
    }


    // Returns the first Z on top of this brick
    fn get_z_above(&self) -> i32 {
	if self.direction.2 != 0 {
	    return self.corner.2 + self.len;
	} else {
	    return self.corner.2 + 1;
	}
    }

    // Returns the set of x,y coords of the "shadow"
    // Should be efficient enough as the input data seems
    // to never have bricks with absurd high length, it's always < 5
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
	// quicker to code but stupid and slow to compute.
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
// returns a tuple with:
// - the new list of moved bricks (roughly for bottom to top order)
// - a vec, in same order, of a list of the indices of bricks directly suupported above this one
// - a vec (same order) of the numer of supporting bricks below
fn settle_bricks(mut b: VecDeque<Brick>) -> (Vec<Brick>, Vec<Vec<usize>>, Vec<i32>) {
    let mut settled = Vec::<Brick>::new();
    let mut supports = Vec::<Vec::<usize>>::new();  // list of indices
    let mut supported_by = Vec::<i32>::new(); // count


    // order the falling bricks from bottom to top
    b.make_contiguous().sort_by(|n, m| n.cmp(m));
    // push down by starting from the bottom bricks, in
    // rough order (similar Z value will always be on different
    // x/y positions so no ambiguity or intersection)

    while let Some(mut brick) = b.pop_front() {
	eprintln!("settling brick {brick:?}");
	let mut min_z = 1; // ground
	// There is probably a better struct to iter
	// with an earlier exit (settled orderded by descending z+height)
	for s in &settled {
	    // small optimization, wins 30% of time
	    if s.get_z_above() < min_z {
	    	continue;
	    }

	    // not optimal as brick.() will recompute the same thing
	    // over. Should memoize ?
	    if brick.intersects_xy_brick(s) {
		min_z = cmp::max(min_z, s.get_z_above());
	    }
	}
	brick.move_at_z(min_z);
	settled.push(brick);
	supported_by.push(0);
	supports.push(vec![]);

    }


    // iterate again to check ALL supporting bricks at this final Z.
    for (idx, s) in settled.iter().enumerate() {
	// due to initial Z order, supporting -> brick -> supported
	// will always be ordered strictly in the final settled array.
	for (ib, below) in settled.iter().enumerate() {
	    if ib >= idx {
		break;
	    }
	    if below.get_z_above() == s.corner.2 && s.intersects_xy_brick(below) {
		supported_by[idx] += 1;
		supports[ib].push(idx);
	    }
	}
    }
    
    
    (settled, supports, supported_by)
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
	let (bricks, supports, supported_by) = settle_bricks(bricks);
	//eprintln!("Settled bricks = {:?}", bricks);
	eprintln!("Settled bricks = supporting indices{:?}", supports);
	eprintln!("Settled bricks = supported_by count{:?}", supported_by);

	// can be disintegrated if supporting no bricks or each of
	// those brick are supported by at least another one
	for k in 0..bricks.len() {
	    let mut disintegrable = true;
	    for &n in &supports[k] {
		if supported_by[n] == 1 {
		    disintegrable = false;
		    break;
		}
	    }
	    if disintegrable {
		eprintln!("settled brick index {k} can be disintegrated");
		self.total += 1;
	    }
	}
	
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
