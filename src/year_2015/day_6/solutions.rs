#![allow(dead_code)]
use std::fmt;

enum Action {
	TurnOn,
	TurnOff,
	Toggle,
}

impl fmt::Display for Action {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Action::TurnOn => write!(f, "TurnOn"),
			Action::TurnOff => write!(f, "TurnOff"),
			Action::Toggle => write!(f, "Toggle"),
		}
	}
}

struct Coordinates {
	x: usize,
	y: usize,
}

fn get_action(instruction_line: &str) -> Action {
	if instruction_line.starts_with("turn on") {
		Action::TurnOn
	}
	else if instruction_line.starts_with("turn off") {
		Action::TurnOff
	}
	else {
		Action::Toggle
	}
}

fn get_coordinates(instruction_line: &str) -> (Coordinates, Coordinates) {
	let mut coordinates: Vec<usize> = Vec::new();

	let split_line: Vec<_> = instruction_line
		.split(
			|c: char| c == ',' || c.is_ascii_whitespace()
		).collect();

	for element in split_line {
		match element.parse::<usize>() {
			Ok(value) => coordinates.push(value),
			Err(_) => (),
		}
	}

	let start: Coordinates = Coordinates {
		x: coordinates[0],
		y: coordinates[1]
	};

	let end: Coordinates = Coordinates {
		x: coordinates[2],
		y: coordinates[3]
	};

	(start, end)
}

struct Instruction {
	action: Action,
	start: Coordinates,
	end: Coordinates,
}

impl Instruction {
	fn from(instruction_line: &str) -> Instruction {
		let action: Action = get_action(instruction_line);
		let (start, end): (Coordinates, Coordinates) = get_coordinates(instruction_line);

		Instruction {
			action,
			start,
			end
		}
	}
}

fn get_how_many_light_lit_grid(light_grid: [[bool; 1000]; 1000]) -> usize {
	let mut light_lit: usize = 0;

	for x in 0..1000 {
		for y in 0..1000 {
			if light_grid[x][y] {
				light_lit += 1;
			}
		}
	}

	light_lit
}

pub fn get_how_many_light_lit(inputs: Vec<String>) -> usize {
	let mut light_grid: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
	let mut instructions: Vec<Instruction> = Vec::new();
	
	for line in inputs {
		instructions.push(Instruction::from(&line));
	}

	for instruction in instructions {
		for x in instruction.start.x..=instruction.end.x {
			for y in instruction.start.y..=instruction.end.y {

				match instruction.action {
					Action::TurnOn => light_grid[x][y] = true,
					Action::TurnOff => light_grid[x][y] = false,
					Action::Toggle => light_grid[x][y] = !light_grid[x][y],
				}

			}
		}
	}

	get_how_many_light_lit_grid(light_grid)
}

fn get_total_brightness_grid(light_grid: [[u8; 1000]; 1000]) -> usize {
	let mut total_brightness: usize = 0;

	for x in 0..1000 {
		for y in 0..1000 {
			total_brightness += light_grid[x][y] as usize;
		}
	}

	total_brightness
}

pub fn get_total_brightness(inputs: Vec<String>) -> usize {
	let mut light_grid: [[u8; 1000]; 1000] = [[0; 1000]; 1000];
	let mut instructions: Vec<Instruction> = Vec::new();

	for line in inputs {
		instructions.push(Instruction::from(&line));
	}

	for instruction in instructions {
		for x in instruction.start.x..=instruction.end.x {
			for y in instruction.start.y..=instruction.end.y {

				match instruction.action {
					Action::TurnOn => light_grid[x][y] += 1,
					Action::TurnOff => {
						if light_grid[x][y] > 0 {
							light_grid[x][y] -= 1
						}
					},
					Action::Toggle => light_grid[x][y] += 2,
				}
			}
		}
	}

	get_total_brightness_grid(light_grid)
}
