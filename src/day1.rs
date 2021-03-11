mod utils;

use utils::read_lines;

fn main() -> Result<(), String> {
    let mut entries: Vec<u64> = Vec::new();

    if let Ok(lines) = read_lines("input/day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(expense) = line {
                entries.push(expense.parse().unwrap());
            }
        }
    }
    entries.sort();
    let _pair_result = find_pair(&entries);
    let _triple_result = find_triple(&entries);
    
    Ok(())
}

fn find_pair(entries: &Vec<u64>) -> Result<(), String>
{
    for entry in entries {
        let remainder = 2020 - entry;
        match entries.binary_search(&remainder) {
            Ok(idx) => {
                let partner = entries[idx];
                show_answer_2(*entry, partner);
                return Ok(())
            },
            Err(_idx) => (),
        }
    }
    Err("No match found".to_string())
}

fn find_triple(entries: &Vec<u64>) -> Result<(), String>
{
    for entry in entries {
        let remainder = 2020 - entry;
        let max_idx = match entries.binary_search(&remainder) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        for second_entry in entries[..max_idx].iter() {
            let second_remainder = remainder - second_entry;
            match entries.binary_search(&second_remainder) {
                Ok(idx) => {
                    let partner = entries[idx];
                    show_answer_3(*entry, *second_entry, partner);
                    return Ok(())
                },
                Err(_idx) => (),
            }
        }
    }
    Err("No match found".to_string())
}

fn show_answer_2(a: u64, b: u64) {
    println!("{:?} + {:?} = 2020", a, b);
    println!("{:?} x {:?} = {:?}", a, b, a * b);
}

fn show_answer_3(a: u64, b: u64, c: u64) {
    println!("{:?} + {:?} + {:?} = 2020", a, b, c);
    println!("{:?} x {:?} x {:?} = {:?}", a, b, c, a * b * c);
}
