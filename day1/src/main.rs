use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let numbers: Vec<_> = input.lines().map(|l|l.trim().parse::<i64>().unwrap()).collect();

	for (a, b) in numbers.iter().cartesian_product(numbers.iter()) {
		if a + b == 2020 {
			println!("Solutio to part 1 is {}", a * b);
			break;
		}
	}


	for ((a, b), c) in numbers.iter().cartesian_product(numbers.iter()).cartesian_product(numbers.iter()) {
		if a + b + c == 2020 {
			println!("Solutio to part 2 is {}", a * b * c);
			break;
		}
	}

}
