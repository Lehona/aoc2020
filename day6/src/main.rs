use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let input = input.replace("\r\n", "\n"); // Fuck you, windows, once again.
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let groups = input.split("\n\n");
    let answers: Vec<_> = groups
        .map(|group| {
            group
                .chars()
                .filter(char::is_ascii_lowercase)
                .collect::<HashSet<_>>()
        })
        .collect();
    let answer_nr: usize = answers.iter().map(|h| h.len()).sum();
    println!("[Part 1] {} Questions were answered.", answer_nr);
}

fn part_two(input: &str) {
    let groups = input.split("\n\n");

    let mut group_answers = vec![];
    for group in groups {
        let base: HashSet<char> = (b'a'..=b'z').map(char::from).collect();
        let group_answer = group
            .lines()
            .map(|l| l.chars().collect::<HashSet<_>>())
            .fold(base, |acc, x| {
                acc.intersection(&x).copied().collect::<HashSet<_>>()
            });
        group_answers.push(group_answer);
    }
    let answer_nr: usize = group_answers.iter().map(|h| h.len()).sum();
    println!("[Part 2] {} Questions were answered.", answer_nr);
}
