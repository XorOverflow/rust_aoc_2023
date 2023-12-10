/*
https://adventofcode.com/2023/day/1
--- Day 1: Trebuchet?! ---
 */

use std::io;
use std::str::FromStr;
//use std::fmt;

fn main() {

    let mut total_calibration:i32 = 0;
    
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Err(_) => { println!("input error, exit"); break; }
            Ok(0) => {
                eprintln!("Eof detected");
                break;
            },
            Ok(_) => {
                // Find the 1st and last ascii digit of the string.
                // (it can be the same character if it's the only one)
                // and concatenate them to parse a decimal value.

                // Pass a function/lambda as the "pattern"
                let digit1: char;
                match input.find(|c:char| c.is_ascii_digit()) {
                    Some(x) => digit1 = input[x..].chars().next().unwrap(),
                    None => panic!("Malformed input, no digit"),
                }
                let digit2: char;
                match input.rfind(|c:char| c.is_ascii_digit()) {
                    Some(x) => digit2 = input[x..].chars().next().unwrap(),
                    None => panic!("Malformed input, no digit"),
                }

                let value_string = format!("{}{}", digit1, digit2);

                match i32::from_str(&value_string) {
                    Ok(v) =>  {
                        eprintln!("parsed calibration value {}", v);
                        total_calibration += v;
                    },
                        
                    Err(_) => {
                        panic!("Malformed input, unparsable digit");
                    },
                }
            }
        }
        // must clear for next loop
        input = String::from("");
    }

    println!("{}", total_calibration);

}
