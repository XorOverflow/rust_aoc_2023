/*
https://adventofcode.com/2023/day/7
--- Day 7: Camel Cards ---
 */

#[macro_use]
extern crate lazy_static;

use std::io;
use std::str::FromStr;
use std::collections::HashMap;
use std::cmp::Ordering;


lazy_static! {
static ref STRENGTH_MAPPER:HashMap<char, char> = HashMap::<char,char>::from([
    ('A', 'm'), // start with the higher letter "m" so it sorts "after" (higher than)
    ('K', 'l'),
    ('Q', 'k'),
    ('T', 'j'),
    ('9', 'i'),
    ('8', 'h'),
    ('7', 'g'),
    ('6', 'f'),
    ('5', 'e'),
    ('4', 'd'),
    ('3', 'c'),
    ('2', 'b'),
    ('J', 'a')]);  // J is now weakest card
}

// declare the enum values in increasing relative value
// for automatic derivation to match our semantic
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

use HandType::*;
    
struct HandBid {
    hand: String,  // with characters converted by STRENGTH_MAPPER
    bid: i32,
    hand_type: HandType,
}

impl HandBid {
    // input : a 5-letter string in the input format ("KK677")
    // and the bid value.
    // Internally, store a modified string easier to compare,
    // and compute the type of the hand once.
    fn new(s: &String, b: i32) -> Self {
        let strength_hand = s.chars()
            .map(|c|  STRENGTH_MAPPER.get(&c).unwrap().clone())
            .collect::<String>();

        // brute-force: try all possible replacement value of "J"
        // and get the max hand type for that case.
        let max_ht;
        if s.contains('J') {
            // It seems that replacing ALL jokers with the same value is
            // always better than trying to have them take different values
            // (because Four of Kind is better than Two Pair, for example)
            max_ht = "bcdefghijklm"
                .chars()
                .map(|c| HandBid::get_type(&str::replace(&strength_hand, 'a',  c.to_string().as_str() ) ) )
                .max().unwrap();
        } else {
            // minor optimization: don't loop on J is there's no J
            max_ht = HandBid::get_type(&strength_hand);
        }

        eprintln!("Hand {s} is of type {:?}", max_ht);
        Self {  hand: strength_hand,
                bid: b,
                hand_type: max_ht,
        }
    }

    fn get_type(s: &String) -> HandType {
        
        //eprintln!("get type: trying {s}");
        let mut sorted_hand : Vec<char> = s.chars().collect();
        sorted_hand.sort_by(|a, b| b.cmp(a));
        // To find the hand type, split the sorted_hand into consecutive
        // sections of identical value, and sort the sizes of those sections.
        let mut section_sizes =  Vec::<u32>::new();
        let mut current_size:u32 = 0;
        let mut current_char = '!'; // This value is not present in the string
        for c in sorted_hand {
            if c == current_char {
                current_size += 1;
            } else {
                if current_size != 0 {
                    section_sizes.push(current_size);
                }
                current_size = 1;
                current_char = c;
            }
        }
        section_sizes.push(current_size); // don't forget the last section being built

        section_sizes.sort_by(|a,b| b.cmp(a));  // sort with bigger first
        let section_1 = section_sizes.get(0).unwrap();

        // return
        match section_sizes.len() {
            1 => FiveOfAKind, // 1 segment, all cards are the same
            2 => // either 4+1 or 3+2 (due to sorting)
                match section_1 {
                    4 => FourOfAKind,
                    3 => FullHouse,
                    _ => panic!("Hand decomposition of {s} is not possible (2)"),
                },
            3 => // either 3 + 1 +1, or 2 + 2 + 1
                match section_1 {
                    3 => ThreeOfAKind,
                    2 => TwoPair,
                    _ => panic!("Hand decomposition of {s} is not possible (3)"),
                }
            4 => OnePair, // Can only be 2 + 1 + 1 + 1
            5 => HighCard,
            x => panic!("Hand decomposition of {s} in {x} elements is not possible"),
        }
    }
       
}

// Implement ordering traits to use default sorting operations
impl Ord for HandBid {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_order = self.hand_type.cmp(&other.hand_type);
        if type_order == Ordering::Equal {
            self.hand.cmp(&other.hand)
        } else {
            type_order
        }
    }
}

impl PartialOrd for HandBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandBid {
    fn eq(&self, other: &Self) -> bool {
        (self.hand_type == other.hand_type) && (self.hand == other.hand)
    }
}
impl Eq for HandBid {}


// Solver for this particular problem

struct Solver {
    total: i32,
    hands: Vec<HandBid>,
}

impl Solver {
    fn new() -> Self {
        Self{total : 0,
             hands : Vec::new(),
        }
    }

    // process one text line of input
    fn process(&mut self, l: &str) {
        if let Some((hand,bid)) = l.split_once(' ') {
            let h = HandBid::new(&hand.to_string(), i32::from_str(bid).unwrap());
            self.hands.push(h);
        }
    }


    fn postprocess(&mut self) {
        self.hands.sort(); // will use the Ord trait from HandBid
        // hands are now ordered on their rank
        let mut rank = 1;
        for hb in &self.hands {
            eprintln!("Adding {} to {}   | {} x {:?}",
                      rank * hb.bid, self.total,
                      hb.hand, hb.bid); 
            self.total += rank * hb.bid;
            rank += 1;
        }
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

    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Err(_) => { println!("input error, exit"); break; }
            Ok(0) => {
                eprintln!("Eof detected");
                break;
            },
            Ok(_) => {
                let input_clean = input.trim(); // remove the \n
                s.process(input_clean);
            }
        }
        // must clear for next loop
        input = String::from("");
    }

    println!("{}", s.result());

}
