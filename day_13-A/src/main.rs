/*
https://adventofcode.com/2023/day/13
--- Day 13: Point of Incidence ---
 */


use std::io;


/*
 * By looking at the input:
 * 1) the reflexion center
 * is not just at Middle +/-1 : it could be also
 * juste at line 1.5, reflecting together lines
 * 1 and 2 and leaving all lines 3-10 with nowhere
 * to reflect outside the limit.
 * So an exhaustive search would do all [1..M/2]
 * and not just the pair (Middle-1, Middle+1)

 * 2) the input patterns are small, smaller than 20
 * in any direction. So it's possible to just map them
 * into bits in standard integers to speed-up equality
 * comparisons.

 * Impl: lots of small off-by-one errors when setting
 * the loop limits and the out-of-pattern tests and the
 * reflection out-of-bounds checks and...
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

        if let Some(row_reflection) = Pattern::get_reflected(&pat.rows) {
            eprintln!("Pattern has reflection over horizontal line {row_reflection}+1");
            self.total += 100 * (row_reflection + 1);
        } else if let Some(col_reflection) = Pattern::get_reflected(&pat.cols) {
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
