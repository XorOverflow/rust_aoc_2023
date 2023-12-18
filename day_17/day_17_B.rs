/*
https://adventofcode.com/2023/day/17
--- Day 17: Clumsy Crucible ---

 part 2 : ULTRA Crucible !
 */


use std::io;
use std::boxed::Box;
use std::cmp;
use std::collections::HashSet;


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
                eprint!("{} ", &self.get(x,y));
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

#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use Direction::*;

impl Direction {
    // Get possible next direction from a starting point and its current
    // moving stats.
    // For Ultra-Crucibles, the initial start or turning
    // requires 4 consecutive straight moves before doing anything else,
    // indicated by the final "bool" in the tuple: caller must iterate
    // on all new_linstead of doing 1 move only.
    fn get_possible_next(&self, from_len:u8) -> Vec<(Direction,u8,bool)> {
        // compute perpendicular turn
        let (a,b): (Direction,Direction) = 
            match self {
                Left | Right => (Up, Down),
                Up | Down => (Left, Right),
            };
        if from_len == 0 { // starting node only: test everything (ignore Self)
            vec![(Right, 4, true), (Left, 4, true), (Down, 4, true), (Up, 4, true)]
        } else if from_len < 4 {
            panic!("get_possible_next called with current len {}. Should be >= 4", from_len);
            // we don't return just [(*self, from_len+1)] because we can't just stop
            // at this node: technically the next node+1 is not "reachable" as a possible starting
            // (or stopping) point. Just count accumulate its cost.
        } else if from_len < 10 {
            vec![(a,4, true), (b,4, true), (*self, from_len+1, false)]
        } else {
            vec![(b,4, true), (a,4, true)]
        }
    }
}

impl<T> Grid<T>{
    // Return Some(newx,newy) after moving by direction (for 1 length only), else None if out-of-bounds
    fn get_next_coordinates(&self, x:usize, y:usize, d: Direction) -> Option<(usize,usize)> {
        match d {
            Left =>  if x == 0             { None } else { Some((x-1, y)) },
            Right => if x+1 >= self.width  { None } else { Some((x+1, y)) },
            Up =>    if y == 0             { None } else { Some((x, y-1)) },
            Down =>  if y+1 >= self.height { None } else { Some((x, y+1)) },
        }
    }
}

#[derive(Clone,Copy,Debug)]
struct DijkstraNode {
    visited: bool,
    tentative_distance: Option<i32>,
    // When progressing in the distance to neighbours, we must respect path constraints
    tentative_distance_continuation: (Direction, u8),
}


impl Grid<DijkstraNode> {
    fn pretty_print_dijkstra(&self) {
        eprintln!("[{},{}] = ", self.width, self.height);
        for y in 0..self.height {
            eprint!("[");
            for x in 0..self.width {
                let n = self.get(x,y);
                let v = match n.tentative_distance {
                    Some(d) => d,
                    _ => 0,
                };
                if n.visited {
                    eprint!("{:4}", v);
                    match n.tentative_distance_continuation.0 {
                        Left => eprint!("<"),
                        Right => eprint!(">"),
                        Up => eprint!("^"),
                        Down => eprint!("v"),
                    }
                } else {
                    eprint!("  .  ");
                }
            }
            eprintln!("]");
        }
    }
}

// Dijkstra
impl Solver {


    /* 
       Same as part1 but with a different dimension formula for the virtual nodes.

    Dijkstra converged in 510773 iterations
    Answer: 1367
    timing: 4 minutes...( 4m15,600s )

     */


    // return the extra dimension for the virtual grid mapping
    fn dijkstra_dimension(d: Direction, len: u8) -> usize {
        if len == 0 {
            0  // for starting node only
        } else {
            // len always 4..10; ignore 1/2/3
            (len as usize - 4) + match d {
                Left => 1,  // maps to [1..=7]
                Right => 8, // maps to [8..=14]
                Up => 15,
                Down => 21,
            }
        }
    }
    
    fn dijkstra(&self) -> i32 {
        let null_node = DijkstraNode {
            visited: false,
            tentative_distance: None,
            tentative_distance_continuation: (Right, 0), // arbitrary for unvisited nodes
        };
        let mut nodes: Vec<Grid::<DijkstraNode>> = Vec::new();
        let max_dim = Self::dijkstra_dimension(Down, 10); // XXX impl detail
        for _ in 0..=max_dim {
            nodes.push(Grid::<DijkstraNode>::new(self.heat_loss.width, self.heat_loss.height, null_node));
        }

        // keep the "frontier" of unvisited nodes in a set/hash for easier iteration/search
        // than in the Grid node. They must be kept in sync.
        let mut unvisited_tentative = HashSet::<(usize,usize, Direction, u8)>::new();
        
        // Set our starting point (distance 0, ignore heat_loss of starting move count)
        // [0] is valid only for len 0
        nodes[0].set(0,0, DijkstraNode { visited: true,
                                         tentative_distance: Some(0),
                                         tentative_distance_continuation: (Right, 0),});
        unvisited_tentative.insert((0,0, Right, 0));


        let mut debug_modulo = 0;
        // Follow dijkstra algo
        while !unvisited_tentative.is_empty() {
            // Get the unvisited node with the smallest tentative distance.

            debug_modulo += 1;
            if debug_modulo % 5000 == 0 {
                nodes[2].pretty_print_dijkstra(); // arbitrary one
            }
            
            // (Nodes in the unvisited_tentative set should always have Some() distance.
            // It would be an error to have None, meaning that unvisited_tentative() and nodes[]
            // were not maintained in sync.
            let current_coord = unvisited_tentative.iter()
                .min_by(|a,b| { let node_a = nodes[Self::dijkstra_dimension(a.2, a.3)].get(a.0, a.1);
                                let node_b = nodes[Self::dijkstra_dimension(b.2, b.3)].get(b.0, b.1);
                                node_a.tentative_distance.unwrap().cmp(&node_b.tentative_distance.unwrap()) }  )
                .unwrap().clone();
            let current_node = nodes[Self::dijkstra_dimension(current_coord.2, current_coord.3)].get(current_coord.0, current_coord.1);
            let Some(current_distance) = current_node.tentative_distance else { panic!("Current node has no distance") };

        
            if     (current_coord.0 + 1  == self.heat_loss.width)
                && (current_coord.1 + 1  == self.heat_loss.height) {
                    // We found the Destination node as the lowest tentative distance.
                    // This is the final path length.
                    // xxx this node is present in 3 versions, we don't know yet which one has
                    // shortest path.
                    eprintln!("Found destination node at tentative distance = {}", current_distance);
                    //return current_distance;
                }

            let (cd,cl) = current_node.tentative_distance_continuation;

            // check all unvisited neighbours
            'possible_next:
            for dir_len in cd.get_possible_next(cl).into_iter() {
                let accumulate_move = dir_len.2; // consume all length without stopping, only the final node will be the real neighbour
                let mut accumulate_dist = 0;
                let mut neighbor_coord:(usize,usize) = (0,0); // not supposed to be left unitialized but...
                if accumulate_move {  // Travel an initial continuous segment without stopping.
                    // We do a little too much here which could be skipped if (direction * 4) directly overflows
                    // or is marked as a visited node. But simpler to write quick & dirty this way, it's 7am and I'm
                    // already late for day 18
                    let mut x = current_coord.0;
                    let mut y = current_coord.1;
                    for _ in 0..dir_len.1 {
                        if let Some(n_coord) = nodes[0].get_next_coordinates(x, y, dir_len.0) {
                            neighbor_coord = n_coord;
                            x = neighbor_coord.0;
                            y = neighbor_coord.1;
                            accumulate_dist += *self.heat_loss.get(x, y) as i32;
                        } else {
                            // Out of Bound. Impossible to move by 4 straight without stopping at the border,
                            // this is an invalid move.
                            continue 'possible_next;
                        }
                    }
                } else {  // perform a single 1-tile move
                    // for next coord pick any "node_x" for identical bound checking.
                    if let Some(n_coord) = nodes[0].get_next_coordinates(current_coord.0, current_coord.1, dir_len.0) {
                        neighbor_coord = n_coord;
                        accumulate_dist = *self.heat_loss.get(n_coord.0, n_coord.1) as i32;
                    } else {
                        continue 'possible_next;
                        // XXX rustc believe neighbor_coord is "uninitialized" here, but we always continue instead of going on below...
                    }
                }
                
                let neigh_nodes = &mut nodes[Self::dijkstra_dimension(dir_len.0, dir_len.1)];
                let neighbor = neigh_nodes.get_mut(neighbor_coord.0, neighbor_coord.1);
                if neighbor.visited {
                    continue;
                }
                let tentative_dist = current_distance + accumulate_dist;
                // Update neighbor best distance (with its associated path origin)
                match neighbor.tentative_distance {
                    None => {
                        neighbor.tentative_distance = Some(tentative_dist);
                        neighbor.tentative_distance_continuation = (dir_len.0, dir_len.1);
                    },
                    Some(d) => {
                            if d > tentative_dist {
                                neighbor.tentative_distance = Some(tentative_dist);
                                neighbor.tentative_distance_continuation = (dir_len.0, dir_len.1);
                            }
                    }
                }
                // add new node in explorable list, if not already present
                unvisited_tentative.insert((neighbor_coord.0, neighbor_coord.1, dir_len.0, dir_len.1));
            }
            // Mark current node as "visited".
            // Have to re-borrow mutable now. Could not do it before due to neighbour nodes also mutable
            let current_node = nodes[Self::dijkstra_dimension(current_coord.2, current_coord.3)].get_mut(current_coord.0, current_coord.1);
            current_node.visited = true;
            unvisited_tentative.remove(&current_coord);
        }

        for k in 0..3 {
            nodes[k].pretty_print_dijkstra();
        }

        eprintln!("Dijkstra converged in {debug_modulo} iterations");
        let mut d:i32 = i32::MAX;
        for k in 0..=max_dim {
            let final_node = nodes[k].get(self.heat_loss.width - 1, self.heat_loss.height - 1);
            match final_node.tentative_distance {
                None =>     eprintln!("end-path dim{k} doesn't reach destination"),
                Some(v) =>  {eprintln!("end-path dim{k}: {v}"); if v<d { d = v}; },
            }
        }
        d
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
        self.total = self.dijkstra();
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
