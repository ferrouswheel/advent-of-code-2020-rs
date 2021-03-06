mod utils;

use std::char;

use utils::read_lines;

fn main() -> Result<(), String> {
    let mut valid_passwords_part_1 = 0u64;
    let mut valid_passwords_part_2 = 0u64;
    let mut line_no = 0u64;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input/day2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            line_no += 1;
            if let Ok(password_line) = line {
                valid_passwords_part_1 += match check_password_part_1(&password_line) {
                    Ok(()) => 1,
                    Err(fail_msg) => {
                        println!("part 1 failure {:?} at line {:?}: {:?}", fail_msg, line_no, password_line);
                        0
                    }
                };

                valid_passwords_part_2 += match check_password_part_2(&password_line) {
                    Ok(()) => 1,
                    Err(fail_msg) => {
                        println!("part 2 failure {:?} at line {:?}: {:?}", fail_msg, line_no, password_line);
                        0
                    }
                };
            }
        }
    }
    println!("part 1 num valid passwords = {:?}", valid_passwords_part_1);
    println!("part 2 num valid passwords = {:?}", valid_passwords_part_2);

    Ok(())
}

struct PasswordInfo {
    password: String,
    policy_char: char,
    policy_part_1: usize,
    policy_part_2: usize,
}

fn parse_line(password_line: &String) -> Result<PasswordInfo, String>
{
    let mut parts = password_line.split(":");
    
    let policy = parts.next().ok_or("bad pw line")?;
    let policy = String::from(policy.trim());

    let password = parts.next().ok_or("bad pw line")?;
    let password = String::from(password.trim());

    let mut policy_parts = policy.split(" ");
    let p_count = policy_parts.next().ok_or("bad pw line")?;
    let p_count = String::from(p_count.trim());

    let p_char = policy_parts.next().ok_or("bad pw line")?;
    let p_char = String::from(p_char.trim());
    let p_char = p_char.chars().next().unwrap();

    let mut count_parts = p_count.split("-");
    let count_min = count_parts.next().ok_or("bad pw line")?;
    let count_min = match String::from(count_min.trim()).parse() {
        Ok(x) => x,
        Err(err) => return Err("blah".to_string())
    };

    let count_max = count_parts.next().ok_or("bad pw line")?;
    let count_max = match String::from(count_max.trim()).parse() {
        Ok(x) => x,
        Err(err) => return Err("blah".to_string())
    };

    Ok(PasswordInfo {
        password,
        policy_char: p_char,
        policy_part_1: count_min,
        policy_part_2: count_max,
    })
}

fn check_password_part_1(password_line: &String) -> Result<(), String>
{
    let pw_info = parse_line(password_line).unwrap();
    let mut p_char_count: usize = 0;

    for x in pw_info.password.chars() {
        if x == pw_info.policy_char {p_char_count += 1}
    }

    if p_char_count < pw_info.policy_part_1 {
        return Err("invalid password".to_string())
    }
    if p_char_count > pw_info.policy_part_2 {
        return Err("invalid password".to_string())
    }

    Ok(())

}

fn check_password_part_2(password_line: &String) -> Result<(), String>
{
    let pw_info = parse_line(password_line).unwrap();
    let mut p_char_count: u64 = 0;

    let mut pw_iter = pw_info.password.chars();

    if pw_iter.nth(pw_info.policy_part_1 - 1).unwrap() == pw_info.policy_char {
        p_char_count += 1
    };
    if pw_iter.nth((pw_info.policy_part_2 - pw_info.policy_part_1) - 1).unwrap() == pw_info.policy_char {
        p_char_count += 1
    }
    
    if p_char_count != 1 {
        return Err("invalid password".to_string())
    }
    
    Ok(())

}

