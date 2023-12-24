/*
https://adventofcode.com/2023/day/24
--- Day 24: Never Tell Me The Odds ---
(Hailstorm)
 */


use std::io;
use std::str::FromStr;

#[derive(Clone,Copy,Debug)]
struct Hailstone {
    pos:(f64, f64, f64),
    vec:(f64, f64, f64),
}

const EPSILON:f64 = 0.000001;

impl Hailstone {
    fn from_str(s:&str) -> Self {
	let (p,v) = s.split_once(" @ ").expect("pos/vec formatting should have a @");
	// sample.txt manually cleaned from the "space aligned"  "-1"/"  1"
	let pos: Vec<f64> = p.split(", ").map(|f| f64::from_str(f).unwrap() ).collect();
	let vec: Vec<f64> = v.split(", ").map(|f| f64::from_str(f).unwrap() ).collect();
	if pos.len() != 3 || vec.len() != 3 {
	    panic!("pos/vec don't have 3 coords");
	}

	Self {
	    pos: (pos[0], pos[1], pos[2]),
	    vec: (vec[0], vec[1], vec[2]),
	}
    }


    // Returns the intersection coordinate and its time coordinates (in both
    // of hailstones vectors basis),
    // or None if they are parallel.
    fn get_intersect_xy(&self, other: &Hailstone) -> Option<((f64,f64), (f64, f64))> {
	let determinant = (self.vec.0 * other.vec.1) - (self.vec.1 * other.vec.0);
	if determinant.abs() < EPSILON {
	    // parallel
	    None
	} else {
	    let ab = (other.pos.0 - self.pos.0, other.pos.1 - self.pos.1);
	    let t:f64 = ((ab.0 * other.vec.1) - (ab.1 * other.vec.0)) / determinant;
	    let p = (self.pos.0 + t * self.vec.0, self.pos.1 + t * self.vec.1);

	    // eprintln!("itx: AB = {ab:?}; determinant = {determinant}; +dt {t} -> {p:?}");

	    // need to compute the other's t for the puzzle, but we can just
	    // check "past" or "future" and return -1/+1
	    let delta_other = p.0 - other.pos.0;
	    let t2 = if delta_other < 0.0 && other.vec.0 < 0.0 {
		1.0
	    } else if delta_other > 0.0 && other.vec.0 > 0.0 {
		1.0
	    } else {
		-1.0
	    };
	    
	    
	    
	    Some((p, (t, t2)))
	}
    }
}

// Solver for this particular problem

struct Solver {
    total: i32,
    min: f64,
    max: f64,
}

impl Solver {
    fn new(min: f64, max: f64) -> Self {
        Self{total : 0,
	     min: min,
	     max: max,
        }
    }

    // process input
    fn process_all(&mut self) {

        let mut hail = Vec::<Hailstone>::new();
	
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
		   hail.push(Hailstone::from_str(input_clean));
                }
            }
            // must clear for next loop
            input = String::from("");
        }

	eprintln!("Parsed {} hailtsones: {:?}",
		  hail.len(),
		  hail);

	let n = hail.len();
	for i in 0..n {
	    for j in i+1..n {
		let hi = &hail[i];
		let hj = &hail[j];
		match hi.get_intersect_xy(hj) {
		    None => eprintln!("{i}/{j} parallel"),
		    Some((p, t)) => {
			if t.0 < 0.0 {
			    eprintln!("{i}/{j} crossed before A: {t:?}");
			} else if t.1 < 0.0 {
			    eprintln!("{i}/{j} crossed before B: {t:?}");
			} else if p.0 < self.min || p.1 < self.min {
			    eprintln!("{i}/{j} outside low : {p:?}");
			} else if p.0 > self.max || p.1 > self.max {
			    eprintln!("{i}/{j} outside high: {p:?}");
			} else {
			    eprintln!("{i}/{j} intersect at valid coordinates {p:?} and times {t:?}");
			    self.total += 1;
			}
		    },
		}
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

    // zone for sample
    // let mut s = Solver::new(7.0, 27.0);

    // zone for actual puzzle
    let mut s = Solver::new(200000000000000.0, 400000000000000.0);
    // 10069 : answer too low
    // 20069 : answer is too high

    s.process_all();

    println!("{}", s.result());

}
