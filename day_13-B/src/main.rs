/*
https://adventofcode.com/2023/day/13
--- Day 13: Point of Incidence ---
 */


use std::io;


/*
 * Part B: smudges
 * exactly 1 bit is flipped in each input.
 *
 * A brute-force approch of inverting each coordinate data
 * and re-running the reflection test could be done
 * without too much performance loss, as the pattern are quite small
 * (on the order of 16x16 so 256 more expansive to brute-check)
 *
 * However we are going to be a bit more clever and use the fact
 * that we already store our data row/cols in single u64 variable so
 * a single-bit difference during reflection-check is easy to do
 * (change "a == b" into a "(a Xor b == 0) else u74::count_ones() == 1"
 *
 * The single bitflip should be detected by a reflection check
 * where there is a single bit difference, then compute the coordinate
 * by the bit position.

 * In practice we don't event need to compute the coordinate of the smudge:
 * the get_single_bitflip_reflected() find the reflexion axis directly
 * by looking for a single difference and doesn't even note where the difference
 * is exactly.
 *
 * P.S. : after reading some comments on reddit of people tricked by
 * the possibility that the solution to part-1 may still be valid after
 * the bitflip is corrected (because it is in a line/column ignored by the
 * "not maching any row" rule) and it had to be explicitly excluded:
 * I note that I didn't even think about that and the current algo
 * automatically avoid this case by return success only when the mirror
 * test is "exact + 1 single difference" (and not "is exact after flipping one
 * bit somewhere, which may or may not be in the covered reflection")
 * which automatically excludes the solution from part-1
 */


// Solver for this particular problem

// Map 1 point "#" to 1 bit
type LineData = u64;

// The matrix of points is represented in duplicate
// representations: all its lines, and all its rows,
// for easier comparison.
#[derive(Debug)]
struct Pattern {
    rows: Vec<LineData>,
    cols: Vec<LineData>,
}

impl Pattern {
    fn new() -> Self {
        Self {
            rows: Vec::<LineData>::new(),
            cols: Vec::<LineData>::new(),
        }
    }
    // Vectors random access with automatic bounds resize
    fn get_from_vec(vec: &Vec<LineData>, index:usize) -> LineData {
        match vec.get(index) {
            Some(&v) => v,
            None => 0,
        }
    }
    
    fn set_at_vec(vec: &mut Vec<LineData>, index:usize, data: LineData) {
        if vec.len() <= index {
            vec.resize(index+1, 0);
        }
        vec[index] = data;
    }

    // Note that it only grows and set the pattern when setting
    // a rock #. lines or columns ending in '.' are not modified
    // so this could have some errors from the real patterns;
    // however in practice it seems to not have any consequences
    // because the puzzle input never have full '...' rows or columns.
    fn set_point_at(&mut self, col: usize, row:usize) {
        let row_val = Self::get_from_vec(&self.rows, row);
        Self::set_at_vec(&mut self.rows, row, row_val | 1 << col);

        let col_val = Self::get_from_vec(&self.cols, col);
        Self::set_at_vec(&mut self.cols, col, col_val | 1 << row);
    }

    /*
    // return the last index before the reflexion axis,
    // indexed by 0. Add 1 for the Puzzle indexing.
    fn get_reflected(vec: &Vec<LineData>) -> Option<usize> {

        // reflexion axis can be anywhere until the last index
        for limit in 0..vec.len()-1 {
            let mut reflected = true;
            //eprintln!("Checking reflexion axis {limit}");
            for k in 0..=limit {
                let reflected_k = limit+k+1; // +1 !
                if reflected_k >= vec.len() {
                    // not a "break": if limit > len/2,
                    // the first index are always out of bound
                    // and we need to reach the first foldable ones later.
                    //eprintln!(" axis {}/{reflected_k} OOB", limit-k);
                    continue;
                }
                if Self::get_from_vec(vec, limit-k) != Self::get_from_vec(vec, reflected_k) {
                    //eprintln!(" axis {}/{reflected_k} are different", limit-k);
                    reflected = false;
                    break;
                } else {
                    //eprintln!(" axis {}/{reflected_k} are same, continuing", limit-k);
                }
            }
            if reflected {
                return Some(limit);
            }
        }

        return None;
    }
     */

    // perform similar test as get_reflected() but instead of looking
    // for "all are equal", look for "all are equal except one with a single bit difference".
    fn get_single_bitflip_reflected(vec: &Vec<LineData>) -> Option<usize> {

        // reflexion axis can be anywhere until the last index
        for limit in 0..vec.len()-1 {
            let mut reflected = true;
            let mut single_bit = false;
            //eprintln!("Checking reflexion axis {limit}");
            for k in 0..=limit {
                let reflected_k = limit+k+1; // +1 !
                if reflected_k >= vec.len() {
                    // not a "break": if limit > len/2,
                    // the first index are always out of bound
                    // and we need to reach the first foldable ones later.
                    //eprintln!(" axis {}/{reflected_k} OOB", limit-k);
                    continue;
                }
                let v = Self::get_from_vec(vec, limit-k);
                let r = Self::get_from_vec(vec, reflected_k);
                if  v == r {
                    //eprintln!(" axis {}/{reflected_k} are same, continuing", limit-k);
                } else {
                    let xdiff = v ^ r; // bit difference
                    if u64::count_ones(xdiff) == 1 {
                        if single_bit {
                            // puzzle input avoid this, so it never happens
                            eprintln!("*** ERROR ? multiple single-bit difference found");
                        }
                        single_bit = true;
                    } else {
                        reflected = false;
                        break;
                    }
                }
            }
            if reflected && single_bit {
                return Some(limit);
            }
        }

        return None;
    }

}


struct Solver {
    total: usize,
}




impl Solver {
    fn new() -> Self {
        Self{total : 0,
        }
    }

    fn process_pattern(&mut self, pat: &Pattern) {
        eprintln!("Consummed pattern: {:?}", pat);

        if let Some(row_reflection) = Pattern::get_single_bitflip_reflected(&pat.rows) {
            eprintln!("Pattern has reflection over horizontal line {row_reflection}+1");
            self.total += 100 * (row_reflection + 1);
        } else if let Some(col_reflection) = Pattern::get_single_bitflip_reflected(&pat.cols) {
            eprintln!("Pattern has reflection over vertical column {col_reflection}+1");
            self.total += col_reflection + 1;
        } else {
            panic!("No reflexion found for pattern !!");
        }

    }
    
    // process all line of texts until empty-line or EOF.
    // return true if not EOF yet
    fn process_pattern_block(&mut self) -> bool {

        let mut pat = Pattern::new();

        let mut input = String::new();
        let mut row = 0;
        loop {
            match io::stdin().read_line(&mut input) {
                Err(_) => { println!("input error, exit"); }
                Ok(0) => {
                    eprintln!("Eof detected");
                    self.process_pattern(&pat);
                    return false;
                },
                Ok(_) => {
                    let input_clean = input.trim(); // remove the \n
                    if input_clean.len() == 0 {
                        self.process_pattern(&pat);
                        return true;
                    }
                    for (i,c) in input_clean.char_indices() {
                        if c == '#' {
                            pat.set_point_at(i, row);
                        }
                    }
                }
            }
            // must clear for next loop
            input = String::from("");
            row += 1; 
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
    while s.process_pattern_block() {
    }


    println!("{}", s.result());

}
