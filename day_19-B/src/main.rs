/*
https://adventofcode.com/2023/day/19

 */


use std::io;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::ops::Range;
use std::cmp;

const RMAX:i32 = 4001;  // for range ..RMAX (..=4000 would be of type RangeInclusive)

// represent a contiguous ranges of each ratings.
// (for example: x[1..4000] & m[1..50] & a[123..843] & s[2400..4000])
#[derive(Clone,Debug)]
struct PartsRange {
    ratings: HashMap::<char, Range<i32>>
}

impl PartsRange {

    // create the default Full range (1..=4000)
    fn new() -> Self {
	let mut ratings = HashMap::<char, Range<i32>>::new();
	for c in vec!['x','m','a','s'] {
	    ratings.insert(c, 1..RMAX);
	}

	Self{
	    ratings:ratings,
	}
    }

    // return the range for a rating name, or 1..4000 when not found
    fn get(&self, c:char) -> Range<i32> {
	match self.ratings.get(&c) {
	    None => 1..RMAX,
	    Some(v) => v.clone(),
	}
    }

    fn set(&mut self, c:char, r:Range<i32>) {
	self.ratings.insert(c,r);
    }

    // return true if any rating has an empty range
    fn is_empty(&self) -> bool {
	self.ratings.iter().fold( false, |acc, (_,r)| acc || r.is_empty() )
    }

    // return the possible combinations of internal ranges
    fn combinations(&self) -> i64 {
	self.ratings.iter().fold( 1i64, |acc, (_,r)| acc * r.len() as i64 )

    }
    
    // return the intersection of the ranges for the two elements.
    fn intersect(&self, other:&PartsRange) -> PartsRange {
	let mut ratings = HashMap::<char, Range<i32>>::new();
	for (c,r) in &self.ratings {
	    let other_r = other.get(*c);
	    let new_start = cmp::max(r.start, other_r.start);
	    let new_end = cmp::min(r.end, other_r.end);
	    if new_start < new_end {
		ratings.insert(*c, new_start..new_end);
	    } else {
		ratings.insert(*c, 0..0); // explicit empty range
	    }
	}

	Self{
	    ratings:ratings,
	}
    }
}

#[derive(Clone)]
struct Rule {
    rating: char,   // x/m/a/s
    cmp_gt: bool,   // true if need to ">" else "<"
    cmp_value: i32,
    dest: String,  // next workflow name
}

impl Rule {
    // sss[</>]num:sss2
    fn from_str(l: &str) -> Self {
	let (cond, dest) = l.split_once(':').expect("rule string should contain a ':'");
	let rating:&str;
	let value_s:&str;
	let cmp_op:bool;
	if let Some((rating1,value1)) = cond.split_once('<') {
	    rating = rating1;
	    value_s = value1;
	    cmp_op = false;
	} else if let Some((rating2,value2)) = cond.split_once('>') {
	    rating = rating2;
	    value_s = value2;
	    cmp_op = true;
	} else {
	    panic!("Rule operator is neither < or >");
	}
	let value = i32::from_str(value_s).expect("rule value should be numeric");
	// get the first character (should be the only one)
	let rating = rating.chars().next().unwrap();
	Self {
	    rating: rating,
	    cmp_gt: cmp_op,
	    cmp_value: value,
	    dest: String::from(dest),
	}
	
    }

//    fn matches(&self, p: &Parts) -> Option<&str> {
//	let v = p.get(self.rating)?;
//	let cmp_success:bool = if self.cmp_gt { v > self.cmp_value }
//	else { v < self.cmp_value };
//	if cmp_success {
//	    Some(self.dest.as_str())
//	} else {
//	    None
//	}
//    }
}

impl PartsRange {
    // 
    fn from_rule(rule: &Rule) -> Self {
	let mut p = Self::new();
	let limit = rule.cmp_value;
	if rule.cmp_gt {
	    p.set(rule.rating, limit+1..RMAX);
	} else {
	    p.set(rule.rating, 1..limit);
	}

	p
    }
    // inverted/non matching rule uses <= and >= not < or >
    fn from_invert_rule(rule: &Rule) -> Self {
	let mut p = Self::new();
	let limit = rule.cmp_value;
	if rule.cmp_gt {
	    p.set(rule.rating, 1..limit+1);
	} else {
	    p.set(rule.rating, limit..RMAX);
	}

	p
    } 
}
	

struct Workflow {
    rules: Vec::<Rule>,
    default: String, // final condition name if not matching rules
}

impl Workflow {
    // expects the string between {...}  (curly braces not included)
    fn from_str(l: &str) -> Self {
	let mut list:Vec<&str> = l.split(',').collect();
	let default = list.pop().unwrap();
	let mut rules = Vec::<Rule>::new();
	for s in list {
	    rules.push(Rule::from_str(s));
	}
	Self {
	    rules: rules,
	    default: String::from(default),
	}
    }
    
    // apply the rules to an input range and return the list
    // of matching sub-ranges + next-workflow

    fn apply_to_range(&self, pr: &PartsRange) -> Vec<(String,PartsRange)> {
	let mut vec = Vec::<(String, PartsRange)>::new();
	let mut pr = pr.clone();
	for r in &self.rules {
	    let r_match = PartsRange::from_rule(r);
	    let r_non_match = PartsRange::from_invert_rule(r);
	    let matching = pr.intersect(&r_match);
	    pr = pr.intersect(&r_non_match);
	    if !matching.is_empty() {
		vec.push( (r.dest.clone(), matching) );
	    } else {
		eprintln!("Workflow apply_to_range() intermediate range can never match do next worflow");
	    }
	}
	if !pr.is_empty() {
	    vec.push( (self.default.clone(), pr) );
	} else {
	    eprintln!("Workflow apply_to_range() final range can never match do default worflow");
	}
	
	return vec;
    }
}



// Solver for this particular problem

struct Solver {
    total: i64,
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,
        }
    }

        // process input
    fn process_all(&mut self) {
	let mut input = String::new();

	// parse the workflows
        let mut workflows = HashMap::<String, Workflow>::new();
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
		    if input_clean.is_empty() {
			break; // end of section
		    }
                    let (name,wf) = input_clean.split_once('{').expect("Workflow should have {...} markers");
		    let wf_str = &wf[0..wf.len()-1]; // drop the final '}'
		    let workflow = Workflow::from_str(wf_str);
		    
		    workflows.insert(String::from(name), workflow);
                }
            }
            // must clear for next loop
            input = String::from("");
        }


	eprintln!("Parsed {} workflows", workflows.len());


	// solve splitting range of parts through all worflows
	let mut queue = VecDeque::<(String, PartsRange)>::new();
	queue.push_back( (String::from("in"), PartsRange::new()) );
	// all the sub-ranges reaching the final Accepted state
	let mut ranges_to_a = Vec::<PartsRange>::new();

	while let Some((wf, sr)) = queue.pop_front() {
	    let  wf = workflows.get(&wf).expect("Workflow names recursion should always be valid!");
	    for (next_wf, next_range) in wf.apply_to_range(&sr) {
		if next_wf == "A" {
		    ranges_to_a.push(next_range);
		} else if next_wf != "R" {
		    queue.push_front( (next_wf, next_range) );
		}
	    }
	}
	eprintln!("Final Accepted subranges are counted {}",
		  ranges_to_a.len());

	//eprintln!("ranges = {:?}", ranges_to_a);

	// If the subranges are never overlapping on all of their
	// subparts at the same time (which should always be the case
	// by their method of construction):
	// Sum of each range internal product combinations
	self.total = ranges_to_a.iter().map(|r| r.combinations() ).sum();
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

    s.process_all();
    
    println!("{}", s.result());

}
