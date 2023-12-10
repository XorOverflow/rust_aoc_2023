/*
https://adventofcode.com/2023/day/6
--- Day 6: Wait For It ---
 */


use std::io;
use std::str::FromStr;


// "solver" pattern not interesting for this problem
// with only 2 lines to parse with their own specific meanings.


// For a Max time T, the race is divided by button-press time 'P' in [0..T]
// and Moving time M = T-P.
// Speed V is equal to P, so distance covered = M * V = (T-P) * P = TP - P^2
// and we want it to be superior to the record D (distance) so
// -P^2 + TP - D > 0
// and P an integer.
fn compute_number_of_ways_winning(time: i32, distance: i32) -> i32 {
    let a: f32 = -1.0;
    let b: f32 = time as f32;
    // case where the previous distance was also an integer requires to be
    // strictly better. Simpler is to add a small epsilon to force new
    // roots to be inside and pray.
    let c: f32 = -0.01 + (-distance) as f32 ;
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
    let i_root1:i32 = root1.ceil() as i32;
    let i_root2:i32 = root2.trunc() as i32;


    let total = i_root2 - i_root1 + 1;
    eprintln!("{}/{} : {} winning moves between {} and {}",
              time, distance, total, i_root1, i_root2);
    return total;
}

fn main() {

    let mut input_time = String::new();
    let mut input_distance = String::new();
    io::stdin().read_line(&mut input_time).unwrap();
    io::stdin().read_line(&mut input_distance).unwrap();
    // drop the "header" and keep the values
    input_time = input_time.split(":").nth(1).unwrap().to_string();
    input_distance = input_distance.split(":").nth(1).unwrap().to_string();

    // parallel iterates between the two lists to create a vector
    // of data pairs
    let mut times_it = input_time.split_whitespace();
    let mut distances_it = input_distance.split_whitespace();
    let mut races = Vec::<(i32,i32)>::new();
    loop {
        match &times_it.next() {
            Some(t) => {
                // It would be an input error if both didn't
                // have the same size
                let d = &distances_it.next().unwrap();
                races.push((i32::from_str(t).unwrap(),i32::from_str(d).unwrap()));
            }
            None => break,
        }
    }
    eprintln!("Races = {:?}", races);

    let total_number_of_ways: i32 = races
        .iter()
        .map(|race| compute_number_of_ways_winning(race.0, race.1))
        .product();

    println!("{}", total_number_of_ways);
    
}
