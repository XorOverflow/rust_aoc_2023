/*
https://adventofcode.com/2023/day/21
--- Day 21: Step Counter ---
 */


use std::io;
use std::boxed::Box;
use std::collections::HashSet;


// A custom 2D array more friendly than a Vec<Vec<T>>
struct Grid<T> {
    width: usize,
    height: usize,
    s : Box<[T]>,
    default_value: T, // for out-of-bounds
}

impl<T: std::clone::Clone> Grid<T> {
    // Allocate the low-level array for this grid
    fn new(width: usize, height: usize, t0: T, default:T) -> Self {
        Self {
            width: width,
            height: height,
            s: vec![t0; width * height].into_boxed_slice(),
	    default_value: default,
        }
    }

    // consume and convert a double-vector
    fn from_vec(mut v: Vec<Vec<T>>, default:T) -> Self {
        let t0 = v[0][0].clone();
        let mut s = Self::new(v[0].len(), v.len(), t0, default);
        // Could probably be done with something like:
        // v.drain(..).drain(..)
        
        // Pop from the end of the vector(s) to avoid
        // realloc (drain data)
        for y in (0..s.height).rev() {
            let mut row = v.pop().unwrap();
            for x in (0..s.width).rev() {
                s.set(x,y, row.pop().unwrap());
            }
        }
        s
    }

    fn get(&self, x:i32, y:i32) -> &T {
        if x < 0 || y < 0
	    || x >= self.width.try_into().unwrap()
	    || y >= self.height.try_into().unwrap() {
            &self.default_value
        } else {
	    let x = x as usize;
	    let y = y as usize;
            &self.s[x + y * self.width]
        }
    }

    // todo: provide a macro
    fn set(&mut self, x:usize, y:usize, t:T) {
        if x >= self.width || y >= self.height {
            panic!("array access {},{} out of bounds", x, y);
        } else {
            self.s[x + y * self.width] = t;
        }
    }
}

impl<T:  std::clone::Clone + std::fmt::Display> Grid<T> {
    fn pretty_print(&self) {
        eprintln!("[{},{}] = ", self.width, self.height);
        for y in 0..self.height {
            eprint!("[");
            for x in 0..self.width {
                eprint!("{} ", &self.get(x as i32,y as i32));
            }
            eprintln!("]");
        }
    }
}


impl Grid<bool> {
    fn pretty_print_bool(&self) {
        eprintln!("[{},{}] = ", self.width, self.height);
        for y in 0..self.height {
            eprint!("[");
            for x in 0..self.width {
                eprint!("{}", if  *self.get(x as i32,y as i32) { '*' } else { '.' });
            }
            eprintln!("]");
        }
    }
}






// Solver for this particular problem

struct Solver {
    total: usize,
    rock_map: Grid<bool>,
    start: (usize, usize),
}



impl Solver {
    fn new() -> Self {
        Self{total : 0,
	     rock_map: Grid::new(1,1, false, false),
	     start : (0,0),
        }
    }

    // process input
    fn process_all(&mut self) {

        let mut map = Vec::<Vec::<bool>>::new();
	
	let mut y = 0;
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
		    let mut x = 0;
                    let line:Vec<bool> = input_clean.chars()
			.map(|c| {
			    if c == 'S' {
				self.start = (x,y);
			    }
			    x += 1;
			    c == '#'
			})
			.collect();
		    map.push(line);
		    y += 1;
                }
            }
            // must clear for next loop
            input = String::from("");
        }

        self.rock_map = Grid::<bool>::from_vec(map, true);
	self.rock_map.pretty_print_bool();
    }


    fn postprocess(&mut self) {
	// last position reached after N steps.
	// hashset so that similar tiles reached by
	// different paths count only once.
	// could have been also implemented by marking
	// true on a Grid, but easier to set and iterate
	// (and doesn't depend on any dimension...)
	let mut tiles = HashSet::<(i32,i32)>::new();

	tiles.insert((self.start.0 as i32, self.start.1 as i32));
	for _steps in 0..64 {
	    let mut next_tiles = HashSet::<(i32,i32)>::new();
	    for (x,y) in tiles.iter() {
		for (dx,dy) in [(0,1), (0,-1), (1,0), (-1,0)] {
		    if !self.rock_map.get(x + dx, y + dy) {
			next_tiles.insert((x + dx, y + dy));
		    }
		}
	    }
	    tiles = next_tiles;
	}
	
        self.total = tiles.len();
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
