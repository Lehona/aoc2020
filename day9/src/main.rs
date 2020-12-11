use itertools::iproduct;

fn main() {
    let input = include_str!("../input.txt");
    let input: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
    let invalid = part_one(&input);
    println!("[Part 1] Nr {} cannot be displayed as a sum of two preceding numbers (in range 25)", invalid);
    part_two(&input, invalid);
}

fn part_one(numbers: &[usize]) -> usize {
	let invalid = numbers.windows(26).enumerate().find(|(_i, w)|!validate_sequence(w));

	match invalid {
		Some((i, _seq)) => numbers[i+25],
		None => panic!("Part one was unsolvable!")
	}
}

fn part_two(numbers: &[usize], invalid: usize)  {
	let length = numbers.len();

	for size in 2..length {
		let sum = numbers.windows(size).enumerate().map(|(i, w)| (i, w.iter().sum::<usize>()) ).find(|(_i, s)| *s == invalid); 
		match sum {
			Some((start, _sum)) => println!("[Part 2] Found: [{}-{}] => {}", start, start+size, sum_minmax(&numbers[start..start+size])),
			None => {}
		}
	}

		// let mut acc = 0;
		// let (end, _) = numbers[start..].iter().take_while(|n| { acc += *n; acc < invalid} ).enumerate().last().unwrap();
		// if acc == invalid {
		// 	println!("The range is {} - {}", start, start+end);
		// 	return numbers[start..start+end].iter().max().unwrap() + numbers[start..start+end].iter().min().unwrap();
		// }

}

fn sum_minmax(input: &[usize]) -> usize {
	let min = input.iter().min().unwrap();
	let max = input.iter().max().unwrap();
	min + max
}

// Given a slice of numbers, checks whether the last number
// can be computed as the sum of two other numbers in the sequence.
fn validate_sequence(seq: &[usize]) -> bool {
    let target = seq[seq.len() - 1];
    let length = seq.len() - 1;

    iproduct!(0..length, 0..length)
        .filter(|(a, b)| a != b)
        .any(|(a, b)| seq[a] + seq[b] == target)
}