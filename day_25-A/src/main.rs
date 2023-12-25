/*
https://adventofcode.com/2023/day/25
--- Day 25: Snowverload ---
(Graphs)
 */


use std::io;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

/*
 * To find the edges to remove, the easiest method was to...
 * process the graph by GraphViz (convert input.txt into a .dot file)
 * and visually find the obvious edges and nodes linking the two
 * big blobs :(
 *
 * Counting the size of a connected graph was then easy.
 */



// Solver for this particular problem

// add the 2 vertices a->b and b->a
// (does not check that it was already inserted before !)
fn connect(connected: & mut HashMap::<String, Vec::<String>>,
	      na: &String, nb: &String) {
    for (a,b) in vec![(na,nb), (nb,na)] {
	if let Some(va) = connected.get_mut(a) {
	    va.push(b.clone());
	} else {
	    connected.insert(a.clone(), vec![b.clone()]);
	}
    }
}


// remove the connections between two nodes
fn disconnect(connected: & mut HashMap::<String, Vec::<String>>,
	      na: &String, nb: &String) {
    for (a,b) in vec![(na,nb), (nb,na)] {
	if let Some(va) = connected.get_mut(a) {
	    if let Some(index) = va.iter().position(|x| x == b) {
		va.remove(index);
	    } else {
		eprintln!("warning, node {b} not found in connect-set of {a}");
	    }
	}
    }
}

// Count the nb of nodes in the connect subgraph containing "start" node.
fn count_connex(connected: &HashMap::<String, Vec::<String>>, start: &String) -> usize {

    // FIFO of adjacent nodes to follow
    let mut border = VecDeque::<String>::new();
    // fixed nodes processed
    let mut nodes = HashSet::<String>::new();

    border.push_back(start.clone());

    while !border.is_empty() {
	let b = border.pop_front().unwrap();
	if nodes.contains(b.as_str()) {
	    // already process earlier by someone else
	    continue;
	}

	if let Some(neighbors) = connected.get(&b) {
	    for n in neighbors {
		if !nodes.contains(n.as_str()) {
		    border.push_back(n.clone());
		}
	    }
	}

	nodes.insert(b.clone());
    }

    return  nodes.len();
}

struct Solver {
    total: usize,
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,

        }
    }

    // process input
    fn process_all(&mut self) {

	// list all nodes connected, by name (both pairs)
        let mut connected = HashMap::<String, Vec::<String>>::new();
	
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
		    let (node,list) =  input_clean.split_once(": ").unwrap();
		    let dest:Vec<String> = list.split(' ').map(|s| s.to_string()).collect();

		    for d in &dest {
			connect(&mut connected, &node.to_string(), &d);
		    }

                }
            }
            // must clear for next loop
            input = String::from("");
        }

	eprintln!("Parsed {} graph nodes: {:?}",
		  connected.len(),
		  connected);

	/* SORRY
	The solutions to the edges to remove was hardcoded by visually
	inspecting the network graph output by GraphViz ;)
	*/
	
	disconnect(&mut connected, &"lmg".to_string(), &"krx".to_string());
	disconnect(&mut connected, &"vzb".to_string(), &"tnr".to_string());
	disconnect(&mut connected, &"tvf".to_string(), &"tqn".to_string());

	let group1 = count_connex(&connected, &"lmg".to_string());
	let group2 = count_connex(&connected, &"krx".to_string());

	eprintln!("size of connected subgraphs: {group1} and {group2}");
	self.total = group1 * group2;
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
    let mut s = Solver::new();
    // 10069 : answer too low
    // 20069 : answer is too high

    s.process_all();

    println!("{}", s.result());

}
