mod utils;

use std::{char, collections::HashSet};

use utils::{read_lines, SplitExt};

const INPUT_FN : &str = "input/day6.txt";


fn unique_chars(group: &Vec<String>) -> usize {
    let mut chars = HashSet::new();
    for line in group {
        for c in line.chars() {
            chars.insert(c);
        }
    }
    chars.len()
}

fn common_chars(group: &Vec<String>) -> usize {
    let mut intersection: Option<HashSet<char>> = None;
    for line in group {
        let mut line_set = HashSet::new();
        for c in line.chars() {
            line_set.insert(c);
            
        }
        intersection = match intersection {
            Some(set) => Some(set.intersection(&line_set).cloned().collect()),
            None => Some(line_set)
        };
    }
    intersection.unwrap().len()
}


fn main() -> Result<(), String> {
    let mut sum = 0usize;
    let mut sum_2 = 0usize;
    
    if let Ok(lines) = read_lines(INPUT_FN) {
        //let grouped_lines = split(lines.map(Result::unwrap), |x| x.is_empty());
        let grouped_lines = lines.map(Result::unwrap).split(|x| x.is_empty());
        for group in grouped_lines {
            sum += unique_chars(&group);
            sum_2 += common_chars(&group);
        }
        
        println!("part 1 answer = {:?}", sum);
        println!("part 2 answer = {:?}", sum_2);
    }

    Ok(())
}
