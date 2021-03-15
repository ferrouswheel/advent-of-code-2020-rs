mod utils;

use std::char;

use utils::read_lines;

const INPUT_FN : &str = "input/dayX.txt";

fn main() -> Result<(), String> {
    
    if let Ok(lines) = read_lines(INPUT_FN) {
        // Consumes the iterator, returns an (Optional) String
        let seats: Vec<Seat> = lines.map(Result::unwrap).map(|x| Seat::load(&x)).collect();
        
        println!("part 1 answer = {:?}", 0);
        println!("part 2 answer = {:?}", 0);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let s = Seat::load(&"BFFFBBFRRR".to_string());
        assert_eq!(s.row, 70);
    }
}
