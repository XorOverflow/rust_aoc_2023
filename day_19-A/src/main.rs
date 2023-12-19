/*
https://adventofcode.com/2023/day/19

 */


use std::io;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Clone)]
struct Parts {
    //x: i32,
    //m: i32,
    //a: i32,
    //s: i32,
    ratings: HashMap::<char, i32>  // easier indexing
}

impl Parts {
    // parse from the complete {...} string, curly braces included
    fn from_str(l: &str) -> Self {
	// trim first and last char {}
	let l = &l[1..l.len()-1];
	let mut ratings = HashMap::<char, i32>::new();
	for s in l.split(',') {
	    let (c,v) = s.split_once('=').expect("parts rating should have a '=' symbol");
	    // get the first character (should be the only one)
	    let rating = c.chars().next().unwrap();
	    let value = i32::from_str(v).expect("rating value should be numeric");
	    ratings.insert(rating, value);
	}

	Self{
	    ratings:ratings,
	}
    }
    
    fn get(&self, c:char) -> Option<i32> {
	match self.ratings.get(&c) {
	    None => None,
	    Some(&v) => Some(v),
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

    fn matches(&self, p: &Parts) -> Option<&str> {
	let v = p.get(self.rating)?;
	let cmp_success:bool = if self.cmp_gt { v > self.cmp_value }
	else { v < self.cmp_value };
	if cmp_success {
	    Some(self.dest.as_str())
	} else {
	    None
	}
	
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
    
    // apply the rules to a Parts and return the next
    // workflow to match
    fn apply(&self, p: &Parts) -> &str {
	for r in &self.rules {
	    if let Some(wnext) = r.matches(p) {
		return wnext;
	    }
	}
	return &self.default.as_str();
    }
}



// Solver for this particular problem

struct Solver {
    total: i32,
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

	input = String::from("");

	eprintln!("Parsed {} workflows", workflows.len());

	// parse the parts
        let mut parts = Vec::<Parts>::new();
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
                    let part = Parts::from_str(input_clean);
		    parts.push(part);
                }
            }
            // must clear for next loop
            input = String::from("");
        }
	eprintln!("Parsed {} parts", parts.len());
	
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
