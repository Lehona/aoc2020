use regex::Regex;
use std::str::FromStr;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let mut instructions = parse_instructions(input);
    part_one(&instructions);
    part_two(&instructions);
}

fn part_one(instrs: &[Instruction]) {
	let result = match execute(instrs) {
		ExecutionResult::Loop(val) => val,
		ExecutionResult::Terminate(val) => val,
	};

	println!("[Part 1] After seeing the same instruction for the second time, the acc is {}", result);
}

fn part_two(instrs: &[Instruction]) {
	let mut possible_mutations = Vec::new();

	for (i, instr) in instrs.iter().enumerate() {
		match instr.op {
			Op::Jmp | Op::Nop => possible_mutations.push(i),
			Op::Acc => {},
		}
	}

	for i in possible_mutations {
		let mut mutated_instructions = instrs.to_vec();
		match mutated_instructions[i].op {
			Op::Jmp => mutated_instructions[i].op = Op::Nop,
			Op::Nop => mutated_instructions[i].op = Op::Jmp,
			Op::Acc => {}
		}

		match execute(&mutated_instructions) {
			ExecutionResult::Loop(_) => {},
			ExecutionResult::Terminate(val) => {
				println!("[Part 2] The program terminated after mutating index {} and halted with value {}", i, val);
				return;
			}
		}
	}


}

fn execute(instrs: &[Instruction]) -> ExecutionResult {
	let mut visited_instrs = HashSet::new();

	let mut PC: isize = 0;
	let mut acc = 0;
	loop {
		if !visited_instrs.insert(PC) {
			return ExecutionResult::Loop(acc);
		}

		if PC >= instrs.len() as isize {
			return ExecutionResult::Terminate(acc);
		}

		let instr = &instrs[PC as usize];
		match instr.op {
			Op::Acc => acc += instr.param,
			Op::Nop => {},
			Op::Jmp => PC += instr.param - 1,
		}

		PC += 1
	}
}

enum ExecutionResult {
	Loop(isize),
	Terminate(isize)
}

#[derive(Debug, Clone)]
enum Op {
    Jmp,
    Acc,
    Nop,
}

impl FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Op, ()> {
        Ok(match s {
            "jmp" => Op::Jmp,
            "acc" => Op::Acc,
            "nop" => Op::Nop,
            _ => panic!("Illegal opcode"),
        })
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    op: Op,
    param: isize,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"^(\w{3}) (.\d+)$").unwrap();
    input
        .lines()
        .map(|l| re.captures(l).unwrap())
        .map(|cpts| {
            let op: Op = cpts.get(1).unwrap().as_str().parse().unwrap();
            let param: isize = cpts.get(2).unwrap().as_str().parse().unwrap();
            Instruction { op, param }
        })
        .collect()
}
