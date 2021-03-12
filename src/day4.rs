mod utils;

use std::char;
use std::fmt;
use std::io;
use std::u64;
use std::fs::File;
use std::collections::HashMap;

use utils::read_lines;

const INPUT_FN : &str = "input/day4.txt";


struct Passport {
    has_required_fields: bool,
    fields_are_valid: bool,
}

fn field_validations(field_name: &str, val: &str) -> bool {
    // part 2 validate fields
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    // cid (Country ID) - ignored, missing or not.
    match field_name {
        "byr" => {
            let v : u64 = val.parse().unwrap();
            if v >= 1920 && v <= 2002 {
                true
            } else {
                false
            }
        },
        "iyr" => {
            let v : u64 = val.parse().unwrap();
            if v >= 2010 && v <= 2020 {
                true
            } else {
                false
            }
        },
        "eyr" => {
            let v : u64 = val.parse().unwrap();
            if v >= 2020 && v <= 2030 {
                true
            } else {
                false
            }
        },
        "hgt" => {
            if val.ends_with("cm") {
                let v : u64 = val.trim_end_matches("cm").parse().unwrap();
                if v >= 150 && v <= 193 {
                    true
                } else {
                    false
                }
            }
            else if val.ends_with("in") {
                let v : u64 = val.trim_end_matches("in").parse().unwrap();
                if v >= 59 && v <= 76 {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        },
        "hcl" => {
            if val.len() != 7 || !val.starts_with("#") {
                false
            } else {
                val.chars().skip(1).all(|x| x.is_ascii_digit() || (x.is_ascii_hexdigit() && x.is_ascii_lowercase()))
            }
        },
        "ecl" => {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&val[..])
        },
        "pid" => {
            if val.len() != 9 {
                false
            } else {
                let val_num :Result<u64, _> = val.parse();
                match val_num {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
        },
        _ => {
            true
        }
    }
}

fn read_passport(lines: &mut io::Lines<io::BufReader<File>>) -> Option<Passport> {
    let mut reading_passport = true;
    let mut prefix_whitespace = true;
    let mut passport_fields = HashMap::new();

    let expected_labels = vec!["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];
    let optional_labels = vec!["sid"];

    // could simplify all this with a regex, but trying to learn rust patterns.
    while reading_passport {
        let passport_line = match lines.next() {
            Some(line_result) => line_result.unwrap(),
            None => {
                if prefix_whitespace {
                    return None;
                }
                reading_passport = false;
                "".to_string()
            }
        };
        if passport_line.trim().is_empty() && !prefix_whitespace {
            reading_passport = false;
            continue;
        } else {
            prefix_whitespace = false;
            for pair in passport_line.split(" ") {
                let field_tuple: Vec<&str> = pair.split(":").collect();
                passport_fields.insert(field_tuple[0].to_string(), field_tuple[1].to_string());
            }
        }

    }
    let mut is_valid = true;
    let mut has_fields = true;
    for label in expected_labels {
        let result = match passport_fields.get(label) {
            Some(val) => {
                println!("{:?}:{:?}", label, val);
                (true, field_validations(label, val))
            },
            None => {
                println!("{:?}:missing!", label);
                (false, false)
            }
        };
        has_fields = result.0;
        is_valid = result.1;
        if !has_fields || !is_valid { break; }
    }

    println!("");
    Some(Passport {
        has_required_fields: has_fields,
        fields_are_valid: is_valid,
    })
}


fn main() -> Result<(), String> {
    let mut line_no = 0u64;
    let mut part1_valid_count = 0u64;
    let mut part2_valid_count = 0u64;
    let mut total_count = 0u64;

    if let Ok(mut lines) = read_lines(INPUT_FN) {
        let mut finished = false;

        while !finished {
            match read_passport(&mut lines) {
                Some(p) => {
                    if p.has_required_fields {
                        part1_valid_count += 1;
                        if p.fields_are_valid {
                            part2_valid_count += 1;
                        }
                    }
                    total_count += 1;
                },
                None => {
                    finished = true;
                }
            }
        }
    }
    
    println!("part 1 answer, valid passports = {:?} / {:?}", part1_valid_count, total_count);
    println!("part 2 answer, valid passports = {:?} / {:?}", part2_valid_count, total_count);
    
    Ok(())
}