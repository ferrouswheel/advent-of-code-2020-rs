mod utils;

use utils::read_lines;

fn main() -> Result<(), String> {
    let mut valid_passwords: u64 = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input/day2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(password_line) = line {
                valid_passwords += match check_password(&password_line) {
                    Ok(()) => 1,
                    Err(fail_msg) => 0,
                }
            }
        }
    }
    println!("num valid passwords = {:?}", valid_passwords);

    Ok(())
}

fn check_password(password_line: &String) -> Result<(), String>
{
    return Ok(())
}

