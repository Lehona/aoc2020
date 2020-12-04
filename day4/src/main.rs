use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let input = input.replace("\r\n", "\n"); // Fuck windows line endings

    // Splitting at double line breaks means splitting at empty lines
    let passport_lines = input.split("\n\n");

    let passports: Vec<_> = passport_lines.map(|p| parse_passport(p)).collect();
    let valid_passports = passports
        .iter()
        .filter(|pp| validate_passport(pp, false))
        .count();

    println!("[Part 1] {} passports are valid.", valid_passports);

    let valid_passports = passports
        .iter()
        .filter(|pp| validate_passport(pp, true))
        .count();
        
    println!("[Part 2] {} passports are valid.", valid_passports);
}

type Passport = HashMap<String, String>;


fn validate_height(height: &str) -> bool {
    let length = height.len();
    if height.ends_with("cm") {
        height[..length - 2]
            .parse::<i64>()
            .map(|h| h >= 150 && h <= 193)
            .unwrap_or(false)
    } else if height.ends_with("in") {
        height[..length - 2]
            .parse::<i64>()
            .map(|h| h >= 59 && h <= 76)
            .unwrap_or(false)
    } else {
        false
    }
}

fn validate_hair(hair: &str) -> bool {
    if hair.len() != 7 {
        return false;
    }

    hair.chars().skip(1).all(|c| c.is_ascii_hexdigit())
}

fn validate_eye(eye: &str) -> bool {
    let allowed = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    allowed.iter().any(|cl| *cl == eye)
}

fn validate_pid(pid: &str) -> bool {
    pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit())
}

fn validate_passport(pp: &Passport, validate_data: bool) -> bool {
    let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let validators: [(&str, fn(&str) -> bool); 7] = [
        ("byr", |byr: &str| {
            byr.parse::<i64>()
                .map(|i| i >= 1920 && i <= 2002)
                .unwrap_or(false)
        }),
        ("iyr", |iyr: &str| {
            iyr.parse::<i64>()
                .map(|i| i >= 2010 && i <= 2020)
                .unwrap_or(false)
        }),
        ("eyr", |eyr: &str| {
            eyr.parse::<i64>()
                .map(|i| i >= 2020 && i <= 2030)
                .unwrap_or(false)
        }),
        ("hgt", |hgt: &str| validate_height(&hgt)),
        ("hcl", |hcl: &str| validate_hair(&hcl)),
        ("ecl", |ecl: &str| validate_eye(&ecl)),
        ("pid", |pid: &str| validate_pid(&pid)),
    ];

    if !required_keys.iter().all(|key| pp.contains_key(*key)) {
        return false;
    } else if !validate_data {
        return true;
    }

    validators
        .iter()
        .all(|(key, func)| func(&pp[*key]))
}

fn parse_passport(pp: &str) -> Passport {
    let attributes = pp.split(' ').map(|attr| attr.split('\n')).flatten();

    attributes
        .map(|attr| attr.split_at(attr.find(':').unwrap()))
        .map(|(key, val)| {
            (
                key.trim_end_matches(':').to_owned(),
                val.trim_start_matches(':').to_owned(),
            )
        })
        .collect()
}
