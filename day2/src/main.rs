use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");

    part_one(input);
    part_two(input);
}

pub fn part_one(input: &str) {
    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)\s*$").unwrap();

    let mut valid_passwords = 0;
    for line in input.lines() {
        let cpts = re.captures(line.trim()).unwrap();
        let lower_bound: usize = cpts.get(1).unwrap().as_str().parse().unwrap();
        let upper_bound: usize = cpts.get(2).unwrap().as_str().parse().unwrap();
        let character_rule = cpts.get(3).unwrap().as_str().chars().next().unwrap();
        let password = cpts.get(4).unwrap().as_str();

        let char_count = password.chars().filter(|c| *c == character_rule).count();

        if char_count >= lower_bound && char_count <= upper_bound {
            valid_passwords += 1;
        }
    }

    println!("[Part1] {} passwords are valid.", valid_passwords);
}

pub fn part_two(input: &str) {
    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)\s*$").unwrap();

    let mut valid_passwords = 0;
    for line in input.lines() {
        let cpts = re.captures(line.trim()).unwrap();
        let first_pos: usize = cpts.get(1).unwrap().as_str().parse().unwrap();
        let second_pos: usize = cpts.get(2).unwrap().as_str().parse().unwrap();
        let character_rule = cpts.get(3).unwrap().as_str().chars().next().unwrap();
        let password = cpts.get(4).unwrap().as_str();

        let at_1st_pos = password
            .chars()
            .nth(first_pos - 1)
            .map(|c| c == character_rule)
            .unwrap_or(false);
        let at_2nd_pos = password
            .chars()
            .nth(second_pos - 1)
            .map(|c| c == character_rule)
            .unwrap_or(false);

        if at_1st_pos ^ at_2nd_pos {
            valid_passwords += 1;
        }
    }

    println!("[Part2] {} passwords are valid.", valid_passwords);
}
