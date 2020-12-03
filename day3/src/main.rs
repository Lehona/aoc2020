fn main() {
    let input = include_str!("../input.txt");

    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
	// Trim to ensure no linebreaks are included
	let lines: Vec<_> = input.lines().map(|l|l.trim()).collect();
	let width = lines[0].len();
	let height = lines.len();

	let (x_off, y_off) = (3, 1);
	let (mut x, mut y) = (0, 0);

	let mut trees = 0;
	loop {
		x = (x + x_off) % width;
		y += y_off;

		if y >= height {
			break;
		} 

		// The input is ascii-only so this is correct
		if lines[y].as_bytes()[x] == b'#' {
			trees += 1;
		}
	}

	println!("[Part 1] I encountered {} trees.", trees);
}

fn part_two(input: &str) {
	let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2),];

	let prod: usize = slopes.iter().map(|slope| eval_slope(input, *slope)).product();

	println!("[Part 2] Multiplying all slopes gives {}", prod);
}

fn eval_slope(input: &str, (x_off, y_off): (usize, usize)) -> usize {
	// Trim to ensure no linebreaks are included
	let lines: Vec<_> = input.lines().map(|l|l.trim()).collect();
	let width = lines[0].len();
	let height = lines.len();

	let (mut x, mut y) = (0, 0);

	let mut trees = 0;
	loop {
		x = (x + x_off) % width;
		y += y_off;

		if y >= height {
			break;
		} 

		// The input is ascii-only so this is correct
		if lines[y].as_bytes()[x] == b'#' {
			trees += 1;
		}
	}

	trees
}
