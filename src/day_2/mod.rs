#[derive(Debug)]
struct PasswordPolicy {
    min: usize,
    max: usize,
    char: char,
    password: String,
}

pub fn solve(input_str: &str) {

    let num_valid_passwords = input_str.lines()
        .map(|s| {
            parse_password(s)
        })
        .filter(|p|{
            if let Some(pass) = p{
                return validate_password(pass);
            }
            false
        })
        .count();
    eprintln!("{}", num_valid_passwords)
}

fn parse_password(line: &str) -> Option<PasswordPolicy> {
    let parts:Vec<&str> = line.split(['-',' ',':',].as_ref()).collect();
    if parts.len() == 5 {
       return Some(PasswordPolicy {
            min: parts[0].parse().unwrap(),
            max: parts[1].parse().unwrap(),
            char: parts[2].parse().unwrap(),
            password: parts[4].parse().unwrap(),
        })
    } else {
        eprintln!("{:?}", parts);
    }
    None
}

fn validate_password(password_policy: &PasswordPolicy) -> bool {
    let count =  password_policy.password.chars().filter(|s| {*s == password_policy.char}).count();
    if password_policy.min <= count && count <= password_policy.max {
        return true
    }
    false
}

pub fn solve2(input_str: &str) {

    let num_valid_passwords = input_str.lines()
        .map(|s| {
            parse_password(s)
        })
        .filter(|p|{
            if let Some(pass) = p{
                return validate_password_part2(pass);
            }
            false
        })
        .count();
    eprintln!("{}", num_valid_passwords)
}

fn validate_password_part2(password_policy: &PasswordPolicy) -> bool {
    let at_first_pos = password_policy.char == password_policy.password.chars().nth(password_policy.min-1).expect("no min value");
    let at_second_pos = password_policy.char == password_policy.password.chars().nth(password_policy.max-1).expect("max more than string length");
        
    if at_first_pos ^ at_second_pos {
        return true
    }
    false
}