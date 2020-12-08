#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::{HashMap, HashSet};

type Rules = HashMap<String, Vec<(usize, String)>>;

fn main() {
    let input = include_str!("../input.txt");
    let rules = parse_rules(input);
    part_one(&rules);
    part_two(&rules);
}

fn part_one(rules: &Rules) {
    let mut reverse_dependencies = find_direct_containers(rules, "shiny gold");
    let mut length = 0;

    while reverse_dependencies.len() > length {
        length = reverse_dependencies.len();

        println!(
            "I got the following dependencies: {:?}\n",
            reverse_dependencies
        );

        reverse_dependencies = reverse_dependencies
            .iter()
            .map(|dep| find_direct_containers(rules, dep))
            .fold(reverse_dependencies.clone(), |acc, deps| {
                acc.union(&deps).cloned().collect()
            });
    }

    println!(
        "[Part 1] Found the following indirect containers: {:?} which contain {} possible bags",
        reverse_dependencies,
        reverse_dependencies.len()
    );
}

fn part_two(rules: &Rules) {
    let starting_rule = &rules["shiny gold"];
    let nr_of_bags = amount_of_contained_bags(rules, starting_rule);

    println!(
        "[Part 2] Shiny gold bags contain {} other bags.",
        nr_of_bags
    );
}

fn amount_of_contained_bags(rules: &Rules, bag: &[(usize, String)]) -> usize {
    bag.iter()
        .map(|(nr, child)| nr + nr * amount_of_contained_bags(rules, &rules[child]))
        .sum()
}

fn find_direct_containers<'r, 's>(rules: &'r Rules, bag: &'s str) -> HashSet<&'r String> {
    rules
        .iter()
        .filter_map(|(name, children)| {
            children
                .iter()
                .find(|(_nr, child)| child == bag)
                .map(|_| name)
        })
        .collect()
}

fn parse_bag_list(input: &str) -> Vec<(usize, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+) (\w+ \w+)").unwrap();
    }

    if input == "no other" {
        Vec::new()
    } else {
        input
            .split(", ")
            .map(|rule| {
                let cpts = RE.captures(rule).unwrap();
                (
                    cpts.get(1).unwrap().as_str().parse().unwrap(),
                    cpts.get(2).unwrap().as_str().to_string(),
                )
            })
            .collect()
    }
}

fn parse_rules(input: &str) -> Rules {
    let re = Regex::new(r"^(\w+ \w+) bags contain (.*?) bags?\.$").unwrap();
    let mut rules = HashMap::new();

    for line in input.lines() {
        let cpts = re.captures(line).expect("Unable to parse line");
        let name = cpts.get(1).unwrap().as_str();
        let children = cpts.get(2).unwrap().as_str();

        rules.insert(name.to_string(), parse_bag_list(children));
    }

    rules
}

