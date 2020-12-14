use std::collections::HashMap;

const VALID_KEYS: &[&str] = &[
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
];

type Passport = HashMap<String, String>;

pub fn parse_passport(input_str: &str) -> Vec<Passport> {
    let mut passports = Vec::new();
    passports.push(Passport::new());
    for line in input_str.lines() {
        let current_password = &mut passports.last_mut().unwrap();
        if line.is_empty() {                
            if !valid_passport(&current_password) {  
                let _ = passports.pop();
            }            
            passports.push(Passport::new());
        } else {
            let elems: Vec<&str> = line.split([':',' '].as_ref()).collect();
            for chunk in elems.chunks(2) {
                let key  = chunk[0];
                let val  = chunk[1];                
                current_password.insert(key.into(), val.into());
            }
        }
    }

    if !valid_passport(&passports.last().unwrap()) {  
        let _ = passports.pop();
    }

    passports 
}

pub fn parse_passport_2(input_str: &str) -> Vec<HashMap<&str,&str>> {
    let passports = input_str
        .split("\r\n\r\n")
        .map(|p| {
            p.split_whitespace()
                .filter_map(|s| {
                    let tokens: Vec<&str> = s.split(":").collect();
                    if tokens.len() == 2 {
                        return Some((tokens[0], tokens[1]));
                    } else {
                        eprintln!("{:?}", tokens);
                    }
                    None
                }).collect()
        }).collect();
    passports
}

fn valid_passport(passport: &Passport) -> bool {
    let mut is_valid = true;
    for key in VALID_KEYS{
        if !passport.contains_key(*key){
            //eprintln!("Missing key: {} in Passport: {:?}", key, passport);
            is_valid = false;
        }
    }
    is_valid
}

fn valid_passport2(passport: &HashMap<&str,&str>) -> bool {
    let invalid = VALID_KEYS
        .iter()
        .any(|k| !passport.contains_key(*k));
    
    if invalid {return false};

    if !validate_entries(passport) {
        return false;
    }

    true
}

fn validate_entries(passport: &HashMap<&str,&str>) -> bool {
    let mut is_valid = true;
    for (key, val) in passport.iter() {
        is_valid = match *key {
            "byr" => valid_number(val, 1920, 2002),
            "iyr" => valid_number(val, 2010, 2020),
            "eyr" => valid_number(val, 2020, 2030),
            "hgt" => match val.get(val.len() - 2..) {
                Some("cm") => valid_number(val.get(..val.len() - 2).unwrap_or(""), 150, 193),
                Some("in") => valid_number(val.get(..val.len() - 2).unwrap_or(""), 59, 76),
                _ => false,
            },
            "hcl" => if val.starts_with("#") {
                    val.len() == 7 && val[1..val.len()].chars().all(|c| c.is_ascii_hexdigit())
                } else {
                    false
                },
            "ecl" => matches!(*val, "amb"|"blu"|"brn"|"gry"|"grn"|"hzl"|"oth"),
            "pid" => {
                val.len() == 9 && val.chars().all(|c| c.is_ascii_digit())
            }
            "cid" => true,
            _ => false,
        };

        if !is_valid {
            eprintln!("Failed at {}: {}", key, val);
            return is_valid
        }        
    }
    is_valid
}

fn valid_number(raw_num: &str, min: i32, max: i32) -> bool {
    let num = raw_num.parse().unwrap_or(0);
    if min <= num && num <= max {
        return true;
    }
    false 
}

pub fn parse_and_validate(input_str: &str) -> Vec<HashMap<&str,&str>> {
    let passports = parse_passport_2(input_str);
    passports.into_iter().filter(|p|valid_passport2(p)).collect()
}

#[cfg(test)]
mod tests {
    use super::{parse_passport_2, valid_passport2};
    use std::collections::HashMap;

    #[test]
    fn check_all_valid_entries() {
        let all_valid = include_str!("all_valid_day4.txt");
        let passports = parse_passport_2(all_valid);
        assert_eq!(passports.len(), 4);
        let valid_passports: Vec<HashMap<&str, &str>> = passports.into_iter().filter(|p|valid_passport2(p)).collect();
        assert_eq!(valid_passports.len(), 4);

    }
    #[test]
    fn check_all_invalid_entries() {
        let all_valid = include_str!("all_invalid_day4.txt");
        let passports = parse_passport_2(all_valid);
        assert_eq!(passports.len(), 4);
        let valid_passports: Vec<HashMap<&str, &str>> = passports.into_iter().filter(|p|valid_passport2(p)).collect();
        assert_eq!(valid_passports.len(), 0);

    }
}