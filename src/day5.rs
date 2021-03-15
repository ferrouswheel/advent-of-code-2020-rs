mod utils;

use core::cmp::*;
use std::char;

use utils::read_lines;

const INPUT_FN : &str = "input/day5.txt";

#[derive(Eq)]
pub struct Seat {
    data_binary: Vec<bool>,
    row: u8,
    column: u8
}

impl Seat {
    pub fn load(line: &String) -> Seat {
        let mut decoded: Vec<bool> = Vec::new();
        //let chars: Vec<char> = line.chars().collect();
        for c in line.chars() {
            decoded.push(match c {
                'F' | 'L' => false,
                'B' | 'R' => true,
                _ => panic!("Unknown character, must be one of: FBLR")
            })
        }
        let row = &decoded[0..7];
        let col = &decoded[7..];

        let row = row.iter().fold(0, |acc, &b| acc*2 + b as u8);
        let col = col.iter().fold(0, |acc, &b| acc*2 + b as u8);
        
        Seat {
            data_binary: decoded,
            row,
            column: col,
        }
    }

    pub fn seat_id(&self) -> u32 {
        self.row as u32 * 8 + self.column as u32
    }

}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data_binary.cmp(&(other.data_binary))
    }
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Seat {
    fn eq(&self, other: &Self) -> bool {
        (self.data_binary) == (other.data_binary)
    }
}

fn main() -> Result<(), String> {
    let max_id;
    let min_id;
    
    if let Ok(lines) = read_lines(INPUT_FN) {
        // Consumes the iterator, returns an (Optional) String
        let seats: Vec<Seat> = lines.map(Result::unwrap).map(|x| Seat::load(&x)).collect();
        
        min_id = seats.iter().min().unwrap().seat_id();
        max_id = seats.iter().max().unwrap().seat_id();
        let mut occupied = vec![false; (max_id - min_id + 1) as usize];
        for s in seats {
            occupied[(s.seat_id() - min_id) as usize] = true;
        }
        let free_space = occupied.iter().enumerate().find(|x| !x.1).unwrap();
    
        println!("part 1 answer = {:?}", max_id);
        println!("part 2 answer = {:?}", free_space.0 as u32 + min_id);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_seat_line_1() {
        let s = Seat::load(&"BFFFBBFRRR".to_string());
        assert_eq!(s.row, 70);
        assert_eq!(s.column, 7);
        assert_eq!(s.seat_id(), 567);
    }

    #[test]
    fn decode_seat_line_2() {
        let s = Seat::load(&"FFFBBBFRRR".to_string());
        assert_eq!(s.row, 14);
        assert_eq!(s.column, 7);
        assert_eq!(s.seat_id(), 119);
    }

    #[test]
    fn decode_seat_line_3() {
        let s = Seat::load(&"BBFFBBFRLL".to_string());
        assert_eq!(s.row, 102);
        assert_eq!(s.column, 4);
        assert_eq!(s.seat_id(), 820);
    }

    #[test]
    fn decode_false_max_id() {
        let s = Seat::load(&"BBBFBFBRRR".to_string());
        assert_eq!(s.row, 117);
        assert_eq!(s.column, 7);
        assert_eq!(s.seat_id(), 943);
    }
}
