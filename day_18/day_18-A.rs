/*
https://adventofcode.com/2023/day/18
--- Day 18: Lavaduct Lagoon ---
 */


use std::io;
use std::str::FromStr;
use std::boxed::Box;
use std::cmp;

// A custom 2D array more friendly than a Vec<Vec<T>>
struct Grid<T> {
    width: usize,
    height: usize,
    s : Box<[T]>,
}

impl<T: std::clone::Clone> Grid<T> {
    // Allocate the low-level array for this grid
    fn new(width: usize, height: usize, t0: T) -> Self {
        Self {
            width: width,
            height: height,
            s: vec![t0; width * height].into_boxed_slice(),
        }
    }

    // consume and convert a double-vector
    fn from_vec(mut v: Vec<Vec<T>>) -> Self {
        let t0 = v[0][0].clone();
        let mut s = Self::new(v[0].len(), v.len(), t0);
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

    fn get(&self, x:usize, y:usize) -> &T {
        if x >= self.width || y >= self.height {
            panic!("array access {},{} out of bounds", x, y)
        } else {
            &self.s[x + y * self.width]
        }
    }

    fn get_mut(&mut self, x:usize, y:usize) -> &mut T {
        if x >= self.width || y >= self.height {
            panic!("array access {},{} out of bounds", x, y)
        } else {
            &mut self.s[x + y * self.width]
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
                eprint!("{}", &self.get(x,y));
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
                eprint!("{}", if  *self.get(x,y) { '*' } else { '.' });
            }
            eprintln!("]");
        }
    }
}






// Solver for this particular problem

struct Solver {
    total: i32,
}

#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
enum Direction {
    Ground, // no direction.
    Left,
    Right,
    Up,
    Down,
}
use Direction::*;





impl Solver {
    fn new() -> Self {
        Self{total : 0,
        }
    }

    // process input
    fn process_all(&mut self) {

        let mut path = Vec::<(Direction, usize)>::new();
        let mut xmin:i32 = 0;
        let mut ymin:i32 = 0;
        let mut xmax:i32 = 0;
        let mut ymax:i32 = 0;

        let mut current_x:i32 = 0;  // start path at 0 to compute boundaries
        let mut current_y:i32 = 0; 

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
                    let mut iter = input_clean.split(' ');
                    let Some(ds) = iter.next() else { panic!("Input line not splittable as expected (direction)") };

                    let l:usize = match iter.next() {
                        None => panic!("Input line not splittable as expected (length)"),
                        Some(v) => usize::from_str(&v).unwrap(),
                    };

                    let d:Direction = match ds {
                        "R" => { current_x += l as i32; Right},
                        "L" => { current_x -= l as i32; Left},
                        "U" => { current_y -= l as i32; Up},
                        "D" => { current_y += l as i32; Down},
                        _  => panic!("Input line not splittable as expected (direction)"),

                    };
                    path.push((d,l));
                    xmin = cmp::min(xmin, current_x);
                    xmax = cmp::max(xmax, current_x);
                    ymin = cmp::min(ymin, current_y);
                    ymax = cmp::max(ymax, current_y);
                }
            }
            // must clear for next loop
            input = String::from("");
        }

        eprintln!("Path = {:?}", path);
        eprintln!("Bounding box : {xmin},{ymin} -- {xmax},{ymax}");

        let map_width:usize = (xmax - xmin + 1) as usize;
        let map_height:usize = (ymax - ymin + 1) as usize;

        let mut grid_path = Grid::<Direction>::new(map_width as usize, map_height as usize, Ground);
        let mut grid_dbg = Grid::<char>::new(map_width as usize, map_height as usize, '.');

        // trace the path starting from a point which will not overflow
        let start_x:i32 = 0 - xmin;
        let start_y:i32 = 0 - ymin;
        let mut current_x = start_x;
        let mut current_y = start_y;

        let mut total_path:usize = 0; // count the area of the border trench
        for (d,l) in path {
            total_path += l;
            let incr:(i32,i32) = match d {
                Left => (-1, 0),
                Right => (1, 0),
                Up => (0, -1),
                Down => (0, 1),
                Ground => panic!("empty direction impossible from path"),
            };
            for _ in 0..l {
                // when going up/down from an horizontal section, note the new direction
                let p = grid_path.get_mut(current_x as usize, current_y as usize);
                if (*p == Left) || (*p == Right) {
                    *p = d;
                }
                
                // debug print only
                let c = grid_dbg.get_mut(current_x as usize, current_y as usize);
                *c = match *c {
                    '.' => if incr.0 == 0 { '|' } else { '-' },
                    '-' => if incr.0 == 0 { '+' } else { '-' },
                    '|' => if incr.0 == 0 { '|' } else { '+' },
                    _ => '+',
                };

                current_x += incr.0;
                current_y += incr.1;
                grid_path.set(current_x as usize, current_y as usize, d);

                // debug only
                let c = grid_dbg.get_mut(current_x as usize, current_y as usize);
                *c = match *c {
                    '.' => if incr.0 == 0 { '|' } else { '-' },
                    '-' => if incr.0 == 0 { '+' } else { '-' },
                    '|' => if incr.0 == 0 { '|' } else { '+' },
                    _ => '+',
                };
            }
        }
        let c = grid_dbg.get_mut(start_x as usize, start_y as usize);
        *c = 'S';

        grid_dbg.pretty_print();


        // perform similar interior counting algo as day 10 part 2
        let mut total_area = 0;
        for y in 0..map_height {
            let mut is_interior = false;
            let mut last_updown = Right; // nothing yet, arbitrary
            for x in 0..map_width {
                let d = grid_path.get(x,y);
                match  d {
                    Ground => if is_interior {
                        total_area += 1;
                        if last_updown  == Up {
                            grid_dbg.set(x, y, 'u');
                        } else {
                            grid_dbg.set(x, y, 'd');
                        }
                    },
                    Left =>grid_dbg.set(x, y, 'L'),
                    Right =>grid_dbg.set(x, y, 'R'),
                    Up | Down => {
                        if is_interior {
                            if *d != last_updown {
                                is_interior = false;
                                grid_dbg.set(x, y, '<'); // exit
                            }
                        } else if *d != last_updown {
                            is_interior = true;
                            grid_dbg.set(x, y, '>'); // enter
                        }
                        last_updown = *d;
                    }
                }
            }
        }

        eprintln!("Found {total_area} interior blocks in addition to {total_path} border");
        grid_dbg.pretty_print();

        self.total = total_path as i32 + total_area as i32;
        //grid_path.pretty_print();

    }


    fn postprocess(&mut self) {
        //self.total = 0;
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
