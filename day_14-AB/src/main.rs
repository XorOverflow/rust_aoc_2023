/*
https://adventofcode.com/2023/day/14
--- Day 14: Parabolic Reflector Dish ---
 */

use std::io;
use std::ops::Range;
use std::env::Args;
/*

A naive Algo to roll boulders until they block would
be easy to do with an exhaustive double iteration on each
column (find each blocking point then find each "O" to accumulate
on the point). It would probably be something like O(M*N)
with M,N the map dimensions, not too crazy. It would just be
a bit tedious to write.

Reading part 1, I guess that part 2 will ask
to do the same thing again in S/W/E directions, probably
repeatedly until a minimal total load is found.
Better to have a reasonably fast algo for part 1 from the start.
 */


/*
Instead of low-level iteration on each boulder, it is possible to
pre-process the map into a set of free ranges between square rocks:
For example for 1 line (on the E-W axis)
      ..##.....#...#
      ^^  ^---^ ^-^
=> [1-2]  [5-9] [11-13]

when computing the rolling, count the number of O inside each range
(ignore their position). After rolling, on the final map
each range will be replaced by their consecutive numbers of O at
the start or end (depending on the direction).

 An optimized version of this can be done with bit representation
(like in the day 13) and various bit-shifting operations instead of
settings a range of N elements in a bool slice[].
Actual puzzle input is of size 100x100 and rust supports u128 integer type,
so this will fit.

For expected Part2, after rolling, and when we will want to perform different rolling axis direction,
there will need to use 2 matrix representations of the map (transpose of each other)
for easier processing.

*/


/* 
 Actual Part 2 : Do N/W/S/E titling 1 BILLION TIMES
 - I would have imagined that it would converge at some point
   to a stable configuration, but no. Instead it enters a ...
   cycle of loads value after around 85 tilt cycles, with
   a cycle length of 11 (in my input sample) or 21.
The sample input in the puzzle has a cycle of length 8

Once the cycle is detected it's only a matter of doing 
(1 BILLION - cycle start) modulo cycle_length
to pick the correct value.

Cycle detection consists of finding our last computed state
somewhere in the previous cycles. As each state derive
mechanically from the previous one, if we have 
State(n) == State(p), then for any i
State(n+i) == State(p+i).

We can't just compare the numerical "load" value because
different boulder map configuration may lead to the same
total value, so state must include the exact map data
(or a robust hash) (or just a simple xor of the map, this seems
to work...)


However it looks like my algo found a cycle of length 22 instead of visually
11, but the load values are the same and duplicated, so no actual difference
even if the binary map state seems different ?
*/


// Used only for initial parsing.
#[derive(PartialEq,Clone,Copy)]
enum Tile {
    Empty,
    Rock,
    Boulder,
}

use Tile::*;


// The maps will exist in two redundant representation, one as
// a vec of rows, and as a vec of columns.

// The explicit map of all boulders, 1 bit = 1 boulder.
type BoulderMap = Vec<u128>;
// A representation of all the free spaces between # rocks as sets of [..] range coordinates.
// Vec<Range> could be also be HashSet as we don't care about the order.
type RangeMap = Vec<Vec<Range<u32>>>;


struct Solver {
    total: i64,
    hbmap: BoulderMap,
    hrmap: RangeMap,
    vbmap: BoulderMap,
    vrmap: RangeMap,
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,
             hbmap: Vec::new(),
             hrmap: Vec::new(),
             vbmap: Vec::new(),
             vrmap: Vec::new(),
        }
    }

    // This could have been parsed directly from the textual string instead
    // of the intermediate representation, but with too much code nesting
    // in process()...
    
    // parse the matrix of tiles into compact representations.
    // return the "rows" version.
    fn tile_map_into_bouldermap(map: &Vec<Vec<Tile>>) -> BoulderMap {
        let mut bmap: BoulderMap = Vec::new();

        for line in map {

            // first boulder gets bit 0 (1<<0), second bit 1 (1<<1) etc... 
            let boulders:u128 = line.iter()
                .enumerate()
                .filter(|(_,v)| **v == Boulder)
                .fold(0u128, |acc, (idx,_)| acc | 1 << idx); // the third | is for bit "or"
            bmap.push(boulders);
        }
        //eprintln!("Mapped O into {:?}", bmap);
        
        bmap
    }

    fn tile_map_into_rangemap(map: &Vec<Vec<Tile>>) -> RangeMap {
        let mut rmap: RangeMap = Vec::new();

        for line in map {

            // no immediate functional equivalent to the boulders to get
            // the list of ranges. Vec (or strings, or slice) .split() never seem
            // to provide the absolute starting indice from the original container (unless
            // we resort to pointer arithmetics), so make it manually:
            let mut ranges:Vec<Range<u32>> = Vec::new();
            let mut r_current:u32 = 0; // 
            for s in line.split(|t| *t == Rock) {
                // This subsplice is a normal one
                let l = s.len() as u32;
                if l > 0 {
                    ranges.push(r_current..r_current+l);
                }
                // else empty slice between "##".
                // Next line will work in both cases.
                r_current += l + 1; // +1 to also skip this #
            }
            rmap.push(ranges);
        }
        //eprintln!("Mapped #..# into {:?}", rmap);
        
        rmap
    }

    // transpose the row/columns in the bitfield representation
    // of a boulder map.
    // Note: since we don't know the horizontal size of the original map,
    // assume it's square and ==  vertical size by vec.len()
    // There may exist some magical/optimized method to transpose binary
    // matrixes like this (found some hints that it requires some special
    // internal representation)
    fn transpose_bouldermap(bmap: &BoulderMap) -> BoulderMap {
        let dim = bmap.len();
        let mut transposed:BoulderMap = Vec::new();
        for i in 0..dim {
            let mut ti:u128 = 0;
            for j in 0..dim {
                // extract all "i-th" bit of bmap in order.
                ti |= ((bmap[j] & (1<<i)) >> i) << j;
            }
            transposed.push(ti);
        }
        transposed
    }

    // process all text input
    fn process(&mut self) {

        let mut map: Vec<Vec<Tile>> = Vec::new();
        
        let mut input = String::new();
        loop {
            match io::stdin().read_line(&mut input) {
                Err(_) => {
                    panic!("input error, exit");
                },
                Ok(0) => {
                    eprintln!("Eof detected");
                    break;
                },
                Ok(_) => {
                    let input_clean = input.trim(); // remove the \n
                    let line: Vec<Tile> = input_clean.chars()
                        .map(|c| match c { '#' => Rock, 'O' => Boulder, _ => Empty })
                        .collect();
                    map.push(line);
                }
            }
            // must clear for next loop
            input = String::from("");
        }

        let bmap_h = Self::tile_map_into_bouldermap(&map);
        let rmap_h = Self::tile_map_into_rangemap(&map);

        let bmap_v = Self::transpose_bouldermap(&bmap_h);
        // Transpose map to get the vertical rangemap
        let mut tmap = vec![Vec::with_capacity(map.len()); map[0].len()];
        for l in map {
            for i in 0..l.len() {
                tmap[i].push(l[i]);
            }
        }
        let rmap_v = Self::tile_map_into_rangemap(&tmap);

        self.hbmap = bmap_h;
        self.hrmap = rmap_h;
        self.vbmap = bmap_v;
        self.vrmap = rmap_v;
    }

    // all bits from rstart..rend are set to 1.
    fn bitmask_from_range(r: &Range<u32>) -> u128 {
        ((1u128 << r.end) - 1) // all bits < r.end are set to 1
            ^ ((1u128 << r.start) - 1) // all bits < r.start are set to 0
    }

    // Apply the boulder/range computing towards beginning or
    // end of axis and return a new bouldermap with items stacked.
    fn tilt_bouldermap_to_direction(bmap: &BoulderMap,
                                    rmap: &RangeMap,
                                    to_0: bool) -> BoulderMap {
        // "lines" are relatives to the orientation of this.
        // bmap/rmap pair. It can correspond to the input rows,
        // or to its transposed columns when tilting North/South.

        let mut tilted: BoulderMap = Vec::new();
        // use rmap for size reference as bmap may be bumped to 128
        // depending on the implementation details
        for line in 0..rmap.len() {
            let bline:u128 = bmap[line];
            let mut moved_boulders_line:u128 = 0;
            for r in &rmap[line] {
                let bitmask:u128 = Self::bitmask_from_range(r);
                // This should use some 1-instruction assembly such as popcount
                let count = (bline & bitmask).count_ones();
                let boulder_moved:u128;
                if to_0 {
                    // all counts should be starting at range start
                    boulder_moved = Self::bitmask_from_range(&(r.start..(r.start+count)));
                } else {
                    // all counts should be ending at range end
                    boulder_moved = Self::bitmask_from_range(&((r.end-count)..r.end));
                }
                moved_boulders_line |= boulder_moved;
            }
            tilted.push(moved_boulders_line);
        }

        tilted
    }

    // Perform only tilt to north once.
    fn postprocess_part_1(&mut self) {
        // Perform Tilting.
        // Part 1 : Tilting to north means using the transposed (vertical) map
        // and tilt to 0.
        let tilted_north = Self::tilt_bouldermap_to_direction(&self.vbmap, &self.vrmap, true);
        //eprintln!("North tilt map: (columns) {:?}", tilted_north);

        // Compute score. Re-transpose the map to get vertical again.
        let tmap = Self::transpose_bouldermap(&tilted_north);
        //eprintln!("North tilt map: (rows) {:?}", tmap);

        // count with the original horizontal map dimension.
        for line in 0..self.hrmap.len() {
            let coefficient = (self.hrmap.len() - line) as i64; // 1-indexing and not 0-indexin
            let boulder_row_count = tmap[line].count_ones() as i64;
            //eprintln!("There are {boulder_row_count} at row {line} (x{coefficient})");
            self.total += coefficient * boulder_row_count;
        }
    }


    // Do the 4 tilts N -> W -> S -> E.
    fn cycle_four_directions(&mut self) {
        let tilted_north = Self::tilt_bouldermap_to_direction(&self.vbmap, &self.vrmap, true);
        self.vbmap = tilted_north;
        self.hbmap = Self::transpose_bouldermap(&self.vbmap);

        let tilted_west = Self::tilt_bouldermap_to_direction(&self.hbmap, &self.hrmap, true);
        self.hbmap = tilted_west;
        self.vbmap = Self::transpose_bouldermap(&self.hbmap);

        let tilted_south = Self::tilt_bouldermap_to_direction(&self.vbmap, &self.vrmap, false);
        self.vbmap = tilted_south;
        self.hbmap = Self::transpose_bouldermap(&self.vbmap);

        let tilted_east = Self::tilt_bouldermap_to_direction(&self.hbmap, &self.hrmap, false);
        self.hbmap = tilted_east;
        self.vbmap = Self::transpose_bouldermap(&self.hbmap);

    }


    fn postprocess_part_2(&mut self) {

        // Use hashing of map value and load total
        let mut previous_states = Vec::<(i64,u128)>::new();
        let debug = false;
        for k in 1..2000 {
            self.cycle_four_directions();
            //eprintln!("Cycle tilt map: (rows) {:?}", self.hbmap);

            let mut load:i64 = 0;
            // compute only the North load as usual
            for line in 0..self.hrmap.len() {
                let coefficient = (self.hrmap.len() - line) as i64; // 1-indexing and not 0-indexin
                let boulder_row_count = self.hbmap[line].count_ones() as i64;
                //eprintln!("There are {boulder_row_count} at row {line} (x{coefficient})");
                load += coefficient * boulder_row_count;
            }
            let computed_state:u128 = 
                self.hbmap.iter().fold(0, |acc, v| acc ^ v);

            eprintln!("Load after {k} cycles: {load}, bstate = {:#x}", computed_state);

            // There are a LOT of potential off-by-one errors between the vector indexing by 0,
            // and the cycle number by 1.
            if let Some(i) =  previous_states.iter().position(|&v| v.0 == load && v.1 == computed_state) {
                // cycle detected

                 // i from Vec is 0-index, we want cycles to start at 1
                let cycle_start = i+1;
                let current_i = previous_states.len()+1;
                let cycle_len = current_i - cycle_start;

                eprintln!("found possible cycle starting at {cycle_start}, len {cycle_len} for value {load}");

                // find modulo for 1 BILLION
                let index_for_billion = cycle_start + (1_000_000_000 - cycle_start) % cycle_len;
                eprintln!("Load for Billionth iterationis at cycle[{}] = {}", index_for_billion, previous_states[index_for_billion - 1].0);
                self.total = previous_states[index_for_billion - 1].0;
                if !debug {
                    return;
                }
            }
            previous_states.push((load,computed_state));
        }
        eprintln!("WARNING ! cycle not found for this test case.");
    }

    
    // Returns the final string of expected output
    fn result(&mut self) -> String {
        self.total.to_string()
    }
}

/* common to all problems */
fn main() {

    let mut s = Solver::new();
    s.process();

    if let Some(_) = std::env::args().find(|s| s == "-2") {
        eprintln!("doing part 2");
        s.postprocess_part_2();
    } else {
        eprintln!("doing part 1");
        s.postprocess_part_1();
    }
    
    println!("{}", s.result());

}
