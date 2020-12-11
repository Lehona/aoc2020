fn main() {
    let input = include_str!("../input.txt");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
	let mut joltages: Vec<usize> = input.lines().map(|l|l.parse().unwrap()).collect();

	// Add outlet (0) and device (max + 3)
	let max = *joltages.iter().max().unwrap();
	joltages.push(0);
	joltages.push(max + 3);
	joltages.sort();
	let one = joltages.windows(2).filter(|w| w[1] - w[0] == 1).count();
	let three = joltages.windows(2).filter(|w| w[1] - w[0] == 3).count();

	println!("[Part 1] One: {}, Three: {}, Product: {}", one, three, one * three);
}

fn part_two(input: &str) {
	let mut joltages: Vec<usize> = input.lines().map(|l|l.parse().unwrap()).collect();

	// Add outlet (0) and device (max + 3)
	let max = *joltages.iter().max().unwrap();
	joltages.push(0);
	joltages.push(max + 3);
	joltages.sort();

	let mut fixpoints: Vec<_> = joltages.windows(2).enumerate().filter(|(_i, w)| w[0] + 3 == w[1]).map(|(i, _w)| i).collect();
	fixpoints.insert(0, 0);



	let solution: Vec<_> = fixpoints.windows(2).map(|w|number_of_paths(joltages[w[0]], &joltages[w[0]+1..=w[1]])).collect();
	println!("[Part 2] Number of arrangements: {}", solution.iter().product::<usize>());
}


fn number_of_paths(start_val: usize, numbers: &[usize]) -> usize {
	if numbers.len() <= 1 {
		return 1;
	}

	let extra_steps = if numbers[1] - start_val <= 3 {
		// we can skip numbers[0]
		number_of_paths(start_val, &numbers[1..])
	} else {
		0
	};

	extra_steps + number_of_paths(numbers[0], &numbers[1..])
}