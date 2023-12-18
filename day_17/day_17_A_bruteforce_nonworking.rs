/*
https://adventofcode.com/2023/day/17
--- Day 17: Clumsy Crucible ---
(weighted graph traversal with constraints)
 */


use std::io;
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
    heat_loss: Grid<u8>,
}

#[derive(Clone,Copy,Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use Direction::*;

impl Direction {
    // Get possible next direction from a starting point: 
    // 2 perpendicular branching,
    // and if from_len is not yet 3 : continue direction
    fn get_possible_next(&self, from_len:u8) -> Vec<(Direction,u8)> {
        let (a,b): (Direction,Direction) = 
            match self {
                Left | Right => (Up, Down),
                Up | Down => (Left, Right),
            };
        if from_len < 3 {
            // favor continue straight first
            vec![(*self, from_len+1), (a,1), (b,1)]
        } else {
            vec![(a,1), (b,1)]
        }
    }
}

impl<T> Grid<T>{
    // Return Some(newx,newy) after moving by direction, else None if out-of-bounds
    fn get_next_coordinates(&self, x:usize, y:usize, d: Direction) -> Option<(usize,usize)> {
        match d {
            Left =>  if x == 0             { None } else { Some((x-1, y)) },
            Right => if x+1 >= self.width  { None } else { Some((x+1, y)) },
            Up =>    if y == 0             { None } else { Some((x, y-1)) },
            Down =>  if y+1 >= self.height { None } else { Some((x, y+1)) },
        }
    }
}


// DFS (depth-first search) brute-force
impl Solver {

    // XXX well brute-forcing doesn't work, qui aurait pû le prédire ?
    // sample takes 4 minutes
    // input doesn't even get a first candidate path below the diagonal cost

    // "Obvious" solution is Dijkstra, with the twist of the limit of 3 tiles straight.
    // Maybe it's possible to remap the grid into a graph with several nodes
    // representing 1/2/3 consecutive tiles and connected only to other nodes
    // representing grids at 90° path change.
    
    
    // Recurse one step on the grid.
    // returns:
    // after unwinding from reaching destination tile => Some(lowest cost)
    // when aborting recursion (no path leads to dest, or too long) = None

    // For micro_optimisation: once a path has been found, its cost is noted.
    // All other path traversal tests are capped as this cost to return early
    // (there is no negative weight cost in the graph)

    // self-intersection are not visited; it seems that even with the x3 limit and
    // only 90° turning, there is no better path that just revisit a previous tile
    // to continue on a 4th/5th straight line. There is always an even shorter
    // path that just don't need cycle.
    
    fn dfs_inner(&self, visited: &mut Grid<bool>,
                 xstart:usize, ystart:usize, from: Direction, from_len:u8,
                 current_cost:i32, mut max_total_cost:Option<i32>, recurs_depth:i32) -> Option<i32> {
        let mut reaches_destination = false;

        for (d,l) in from.get_possible_next(from_len) {
            //eprintln!("next direction {:?}", d);
            if let Some((x,y)) = visited.get_next_coordinates(xstart, ystart, d) {
                if *visited.get(x,y) {   // path self-intersection, cancel
                    continue;
                }

                // early detection of path already too expensive
                let new_cost = current_cost + *self.heat_loss.get(x,y) as i32;
                if let Some(max) = max_total_cost {
                    if new_cost >= max {

                        //eprintln!("COST OVERFLOW");
                        //visited.pretty_print_bool();

                        continue;
                    }
                }

                if  x+1 == self.heat_loss.width && y+1 == self.heat_loss.height {
                    eprintln!("REACHED DESTINATION with current cost {new_cost} at depth {recurs_depth}");
                    visited.pretty_print_bool();
                    // return Some(new_cost); Terminal condition ? No ! we need to check also 1 or 2 other possible paths
                    reaches_destination = true;
                    max_total_cost = Some(new_cost);
                    continue;
                }

                visited.set(x,y,true);
                if let Some(path_cost) = self.dfs_inner(visited,
                                                         x, y, d, l,
                                                        new_cost, max_total_cost,
                                                        recurs_depth+1) {
                    reaches_destination = true;
                    max_total_cost = Some(path_cost); // always get smaller by early-return of inner call.
                } else {
                    // this new direction lead to dead-end or too big value not followed.
                    // max_total_cost must not be changed (and must NOT be set to None !)
                }
                visited.set(x,y, false); // unmark for next path to test
            }
        }

        if reaches_destination {
            // return the smallest cost of the tests path, which has also been
            // assigned to the new max_total_cost
            max_total_cost
        } else {
            None
        }
    }

    // Compute the cost of a valid but arbitrary path that we know reaches the end
    // (diagonal staircase)
    // to cap the max_cost (and thus max recursion) of the path searching.

    // In my Input data, this lead to a diagonal cost of 1632 (compared to 75000 in the first
    // DFS results, which should cut early a good part of iterations)
    fn dfs_diagonal_seed(&self) -> i32 {
        // Only works when we know that width ~= height
        let mut diag = Grid::<bool>::new(self.heat_loss.width, self.heat_loss.height, false);
        let mut cost = 0;
        let diag_3_count = self.heat_loss.width / 2;
        // we count the 1st tile by mistake (compared to the instructions text) but since it's
        // just added to the max, it's still valid for our purpose of arbitrary capping.
        for k in 0..diag_3_count {
            let x = 2 * k;
            let y = 2 * k;
            
            cost += *self.heat_loss.get(x,y) as i32 + *self.heat_loss.get(x+1,y) as i32 + *self.heat_loss.get(x+2,y) as i32
                + *self.heat_loss.get(x+2,y+1) as i32;
            diag.set(x,y, true);
            diag.set(x+1,y, true);
            diag.set(x+2,y, true);
            diag.set(x+2,y+1, true);
        }
        cost += *self.heat_loss.get(self.heat_loss.width-1 ,self.heat_loss.height-1) as i32;
        diag.set(self.heat_loss.width-1 ,self.heat_loss.height-1, true);
        // debug
        diag.pretty_print_bool();
        eprintln!("Diagonal cost = {cost}");

        cost
    }

    fn dfs(&self) -> i32 {
        let mut visited = Grid::<bool>::new(self.heat_loss.width, self.heat_loss.height, false);

        visited.set(0,0, true);

        let max = Some(self.dfs_diagonal_seed());
        // for entry point we indicate an arbitrary "from" direction Right
        // with a fake length of 0 (and not 1) to allow to add 3 more Right block directions
        // instead of 2.
        // Cost of initial tile is ignored (from the instruction text)
        if let Some(minimal_cost) = self.dfs_inner(&mut visited,
                                                   0, 0, Right, 0,
                                                   0, max,
                                                   1) {
            eprintln!("Found a Path with minimal cost {minimal_cost}");
            minimal_cost
        } else {
            panic!("No terminal path found at all ??")
        }
    }
}


impl Solver {
    fn new() -> Self {
        Self{total : 0,
             heat_loss: Grid::<u8>::new(1,1,0), // Arbitrary size before replacing it after parsing
        }
    }

    // process input
    fn process_all(&mut self) {

        let mut map = Vec::<Vec::<u8>>::new();
        
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
                    let line:Vec<u8> = input_clean.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
                    map.push(line);
                }
            }
            // must clear for next loop
            input = String::from("");
        }

        self.heat_loss = Grid::<u8>::from_vec(map);
        self.heat_loss.pretty_print();
    }


    fn postprocess(&mut self) {
        self.total = self.dfs();
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
