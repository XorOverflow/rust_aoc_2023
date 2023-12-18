

// annoying case, which doesn't give the correct result
// when not handled.
// When several paths lead to the same (or close) total weight/distance,
// they *all* need to be conserved and iterated: if not,
// when revisiting this node, maybe the first continuation
// cannot continue optimally (due to the 3 straight moves limit) while
// another continuation could progress in the good direction
// and have a final weight lower than the first one + detour.



/*

For example:  for the node marked X,
Path A has cost N, while cost coming
from B as cost N + 8

But at X, from A it's forbidden to continue
right to Dest, and instead has to take at
least a 90° turn to a node of cost 9.
From B, we can just make a last turn
and make up for our deficit of 8.

                                      .  
       A-v         > v B              .
         v           v                .
         v           v  9             .
         1 > 1 > 1 > 1X  Dest         .
                     9  9             .
                                      .
                                      .
 */



        fn dijkstra(&self) -> i32 {
        let null_node = DijkstraNode {
            visited: false,
            tentative_distance: None,
            tentative_distance_continuation: (Right, 0), // arbitrary for unvisited nodes
        };
        let mut nodes = Grid::<DijkstraNode>::new(self.heat_loss.width, self.heat_loss.height, null_node);

        // keep the "frontier" of unvisited nodes in a set/hash for easier iteration/search
        // than in the Grid node. They must be kept in sync.
        let mut unvisited_tentative = HashSet::<(usize,usize)>::new();
        
        // Set our starting point (distance 0, ignore heat_loss of starting)
        nodes.set(0,0, DijkstraNode { visited: true,
                                      tentative_distance: Some(0),
                                      tentative_distance_continuation: (Right, 0),});
        unvisited_tentative.insert((0,0));


        let mut debug_modulo = 0;
        // Follow dijkstra algo
        while !unvisited_tentative.is_empty() {
            // Get the unvisited node with the smallest tentative distance.

            debug_modulo += 1;
            if debug_modulo % 1000 == 0 {
                nodes.pretty_print_dijkstra();
            }
            
            // (Nodes in the unvisited_tentative set should always have Some() distance.
            // It would be an error to have None.)
            let current_coord = unvisited_tentative.iter()
                .min_by(|a,b| { let node_a = nodes.get(a.0, a.1);
                                let node_b = nodes.get(b.0, b.1);
                                node_a.tentative_distance.unwrap().cmp(&node_b.tentative_distance.unwrap()) }  )
                .unwrap().clone();
            let current_node = nodes.get(current_coord.0, current_coord.1);
            let Some(current_distance) = current_node.tentative_distance else { panic!("Current node has no distance") };

        
            if     (current_coord.0 + 1  == self.heat_loss.width)
                && (current_coord.1 + 1  == self.heat_loss.height) {
                    // We found the Destination node as the lowest tentative distance.
                    // This is the final path length.
                    eprintln!("Found destination node at tentative distance = {}", current_distance);
                    //return current_distance;
                }

            let (cd,cl) = current_node.tentative_distance_continuation;

            // check all unvisited neighbours
            for dir_len in cd.get_possible_next(cl).into_iter() {
                if let Some(neighbor_coord) = nodes.get_next_coordinates(current_coord.0, current_coord.1, dir_len.0) {
                    let neighbor = nodes.get_mut(neighbor_coord.0, neighbor_coord.1);
                    if neighbor.visited {
                        continue;
                    }
                    let tentative_dist = current_distance + *self.heat_loss.get(neighbor_coord.0, neighbor_coord.1) as i32;
                    // Update neighbor best distance (with its associated path origin)
                    match neighbor.tentative_distance {
                        None => {
                            neighbor.tentative_distance = Some(tentative_dist);
                            neighbor.tentative_distance_continuation = dir_len;
                        },
                        Some(d) => {

                            if d > tentative_dist {
                                neighbor.tentative_distance = Some(tentative_dist);
                                neighbor.tentative_distance_continuation = dir_len;
                            }
                            //else if d == tentative_dist {
                            //    // try a minor kludge: favour paths that don't end with
                            //    // a series of straight moves. This is not general and
                            //    // doesn't eliminate enough to reach the correct solution.
                            //    if dir_len.1 < neighbor.tentative_distance_continuation.1 {
                            //        neighbor.tentative_distance = Some(tentative_dist);
                            //        neighbor.tentative_distance_continuation = dir_len;
                            //    }
                            //}

                        }
                    }
                    // add new node in explorable list, if not already present
                    unvisited_tentative.insert(neighbor_coord);
                }
            }
            // Mark current node as "visited".
            // Have to re-borrow mutable now. Could not do it before due to neighbour nodes also mutable
            let current_node = nodes.get_mut(current_coord.0, current_coord.1);
            current_node.visited = true;
            unvisited_tentative.remove(&current_coord);
        }

        nodes.pretty_print_dijkstra();

        eprintln!("Dijkstra converged in {debug_modulo} iterations");
        let final_node = nodes.get(self.heat_loss.width - 1, self.heat_loss.height - 1);
        match final_node.tentative_distance {
            None =>     panic!("Should have found Destination tile during while()..."),
            Some(v) => return v,
        }
    }
