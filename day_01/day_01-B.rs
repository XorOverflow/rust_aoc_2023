/*
https://adventofcode.com/2023/day/1
--- Day 1: Trebuchet?! ---
 */

use std::io;
use std::str::FromStr;

fn find_from(s: &String, l: &Vec<(&str,char)>) -> Option<(usize, char)> {
    let mut minfound = usize::MAX;
    let mut charfound = '0';
    for k in l.iter() {
        match s.find(k.0) {
            Some(i) => {
                if i < minfound { minfound = i; charfound = k.1;}
            },
            None => {},
        }
    }
    if minfound != usize::MAX {
        return Some((minfound, charfound));
    } else {
        return None;
    }
       
}

fn rfind_from(s: &String, l: &Vec<(&str,char)>) -> Option<(usize, char)> {
    let mut maxfound = 0;
    let mut found = false;
    let mut charfound = '0';
    for k in l.iter() {
        match s.rfind(k.0) {
            Some(i) => {
                if i > maxfound {
                    found = true; maxfound = i; charfound = k.1;
                }
            },
            None => {},
        }
    }
    if found {
        return Some((maxfound, charfound));
    } else {
        return None;
    }
       
}

fn main() {

    let mut total_calibration:i32 = 0;

    let literals: Vec<(&str,char)> = vec![("one", '1'),
                                          ("two", '2'),
                                          ("three", '3'),
                                          ("four", '4'),
                                          ("five", '5'),
                                          ("six", '6'),
                                          ("seven", '7'),
                                          ("eight", '8'),
                                          ("nine", '9')];
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Err(_) => { println!("input error, exit"); break; }
            Ok(0) => {
                eprintln!("Eof detected");
                break;
            },
            Ok(_) => {
                // Find the 1st and last ascii digit of the string,
                // or strings from spelled-out "one" .. "nine"

                eprintln!(" parsing input-line {} ", input);

                
                let mut digit1: char = 'x';
                let mut digit1dex = 0;
                let mut hasdigit1 = false;
                match input.find(|c:char| c.is_ascii_digit()) {
                    Some(dex1) => {
                        digit1dex = dex1;
                        digit1 = input[digit1dex..].chars().next().unwrap();
                        hasdigit1 = true;
                    }
                    None => { eprintln!(" 1: wait for literal string match "); },
                }
                match find_from(&input, &literals) {
                    Some((idx, digit)) => {
                        if !hasdigit1 || (idx < digit1dex) {
                            digit1 = digit;
                            //digit1dex = idx; // useless, but cleanup.
                        }
                    },
                    None => {
                        if !hasdigit1 {
                            panic!("Malformed input: no digit1 or literal string");
                        }
                    },
                }

                let mut digit2: char = 'y';
                let mut digit2dex = usize::MAX;
                match input.rfind(|c:char| c.is_ascii_digit()) {
                    Some(dex2) => {
                        digit2dex = dex2;
                        digit2 = input[digit2dex..].chars().next().unwrap();
                    },
                    None => { eprintln!(" 2: wait for literal string match ");},
                }
                match rfind_from(&input, &literals) {
                    Some((idx, digit)) => {
                        if digit2dex == usize::MAX || idx > digit2dex {
                            digit2 = digit;
                            //digit2dex = idx; // useless, but cleanup.
                        } 
                    },
                    None => {
                        if digit2dex == usize::MAX {
                            panic!("Malformed input: no digit2 or literal string");
                        }
                    },
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
