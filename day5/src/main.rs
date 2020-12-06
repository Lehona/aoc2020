fn main() {
    let input = include_str!("../input.txt");
    find_highest_id(input);
    find_missing_seat(input);
}

fn get_seats<'a>(input: &'a str) -> impl Iterator<Item=usize> + 'a {
	input
        .lines()
        .map(|l| decode_row(&l[..7]) * 8 + decode_col(&l[7..10]))
}

fn find_highest_id(input: &str) {
    let highest = get_seats(input)
        .max()
        .unwrap();

    println!("Highest ID I got is: {}", highest);
}

fn find_missing_seat(input: &str) {
	let mut seats: Vec<_> = get_seats(input).collect();
	seats.sort();
	
	let missing_seat = seats.windows(2).find(|w| w[0] + 1 != w[1]);

	println!("The missing seat has nr. {}", missing_seat.unwrap()[0] + 1);
}

fn decode_row(row_str: &str) -> usize {
    let (mut lower, mut upper) = (0, 127);

    for c in row_str.chars() {
        let midpoint = (upper - lower) / 2 + lower;
        match c {
            'F' => {
                upper = midpoint;
            }
            'B' => {
                lower = midpoint + 1;
            }
            _ => panic!("Got char {}", c),
        }

        // println!("M: {}, H: ({} - {})", midpoint, lower, upper);
    }

    // println!("After decoding we get ({}, {})", lower, upper);

    if upper != lower {
        panic!("Upper != Lower");
    }

    upper
}

fn decode_col(col_str: &str) -> usize {
    let (mut lower, mut upper) = (0, 7);

    for c in col_str.chars() {
        let midpoint = (upper - lower) / 2 + lower;
        match c {
            'L' => {
                upper = midpoint;
            }
            'R' => {
                lower = midpoint + 1;
            }
            _ => panic!(),
        }

        // println!("M: {}, H: ({} - {})", midpoint, lower, upper);
    }

    // println!("After decoding we get ({}, {})", lower, upper);

    if upper != lower {
        panic!("Upper != Lower");
    }

    upper
}
