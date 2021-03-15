mod utils;

use std::char;

use utils::{read_lines, SplitExt};

const INPUT_FN : &str = "input/day6.txt";



fn main() -> Result<(), String> {
    
    if let Ok(lines) = read_lines(INPUT_FN) {
        //let grouped_lines = split(lines.map(Result::unwrap), |x| x.is_empty());
        let grouped_lines = lines.map(Result::unwrap).split(|x| x.is_empty());
        for group in grouped_lines {
            for line in group {
                println!("{:?}", line);
            }
            println!("");
        }
        
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
