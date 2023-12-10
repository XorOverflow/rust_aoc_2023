/*
https://adventofcode.com/2023/day/6
--- Day 6: Wait For It ---
 */


use std::io;
use std::str::FromStr;

// Compared to part A, part B needs 64 bits values to avoid overflow (right from the
// input parsing)


// For a Max time T, the race is divided by button-press time 'P' in [0..T]
// and Moving time M = T-P.
// Speed V is equal to P, so distance covered = M * V = (T-P) * P = TP - P^2
// and we want it to be superior to the record D (distance) so
// -P^2 + TP - D > 0
// and P an integer.
fn compute_number_of_ways_winning(time: i64, distance: i64) -> i64 {
    let a: f64 = -1.0;
    let b: f64 = time as f64;
    // case where the previous distance was also an integer requires to be
    // strictly better. Simpler is to add a small epsilon to force new
    // roots to be inside and pray.
    let c: f64 = -0.01 + (-distance) as f64 ;
    let delta = b*b-4.0*a*c;
    if delta < 0.0 {
        return  0;
    }
    // with our coefficients, root1 < root2
    let root1 = (-b + delta.sqrt()) / (2.0 * a);
    let root2 = (-b - delta.sqrt()) / (2.0 * a);

    eprintln!("float {root1} and {root2}");

    // for counting the integral number of winning cases,
    // get the first integers inside the roots range,
    // so above root1 and below root2
    let i_root1:i64 = root1.ceil() as i64;
    let i_root2:i64 = root2.trunc() as i64;


    let total = i_root2 - i_root1 + 1;
    eprintln!("{}/{} : {} winning moves between {} and {}",
              time, distance, total, i_root1, i_root2);
    return total;
}

fn main() {

    let mut input_time = String::new();
    let mut input_distance = String::new();
    io::stdin().read_line(&mut input_time).
    io::stdin().read_line(&mut input_distance).unwrap();
    // drop the "header" and keep the values
    input_time = input_time.split(":").nth(1).unwrap().to_string();
    input_distance = input_distance.split(":").nth(1).unwrap().to_string();

    // concatenates the elements to eliminate the fake spaces of the
    // bad kerning to get one single integer:
    let mut bigtime = String::new();
    for k in input_time.split_whitespace() {
        bigtime.push_str(k);
    }
    let mut bigdistance = String::new();
    for k in input_distance.split_whitespace() {
        bigdistance.push_str(k);
    }
    eprintln!("big : {}, {}", bigtime, bigdistance);
    let total_number_of_ways = compute_number_of_ways_winning( i64::from_str(bigtime.as_str()).unwrap(),
                                                               i64::from_str(bigdistance.as_str()).unwrap());
    
    println!("{}", total_number_of_ways);
    
}
