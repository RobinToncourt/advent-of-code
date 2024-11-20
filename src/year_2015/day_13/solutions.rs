#![allow(dead_code)]
use regex::Regex;

const REGEX_STRING: &str = r"(?<attendee>\w+) would (?<lose_gain>\w+) (?<value>\w+) happiness units by sitting next to (?<neighbor>\w+)";

struct Matrice<Label, Data> {
	labels: Vec<Label>,
	content: Vec<Vec<Data>>,
}

impl<Label, Data> Matrice<Label, Data>
where
	Data: Default,
{
	fn new(labels: Vec<Label>) -> Matrice<Label, Data> {
		let size: usize = labels.len();
		
		let mut content: Vec<Vec<Data>> = Vec::new();
		for _ in 0..size {
			let mut inner_content: Vec<Data> = Vec::new();
			for _ in 0..size {
				inner_content.push(Data::default());
			}
			content.push(inner_content);
		}
		
		Matrice::<Label, Data> {
			labels: labels,
			content,
		}
	}
}

impl<Label, Data> Matrice<Label, Data>
where
	Label: std::cmp::PartialEq,
{
	fn get_key_pos(&self, key: &Label) -> usize {
		self.labels
			.iter()
			.enumerate()
			.find(|&label| label.1 == key)
			.unwrap()
			.0
	}
	
	fn set(&mut self, x_key: &Label, y_key: &Label, value: Data) {
		let x_index: usize = self.get_key_pos(x_key);
		let y_index: usize = self.get_key_pos(y_key);
		
		self.content[x_index][y_index] = value;
	}
	
	fn get(&self, x_key: &Label, y_key: &Label) -> &Data {
		let x_index: usize = self.get_key_pos(x_key);
		let y_index: usize = self.get_key_pos(y_key);
		
		&self.content[x_index][y_index]
	}
	
	fn get_path_data(&self, path: &Vec<Label>) -> Vec<&Data> {
		let mut path_values: Vec<&Data> = Vec::new();
		
		for i in 0..path.len()-1 {
			path_values.push(self.get(&path[i], &path[i+1]));
		}
		
		path_values
	}
}

impl<Label, Data> Matrice<Label, Data>
where
	Data: std::fmt::Debug,
{
	fn print_data(&self) {
		for line in &self.content {
			println!("{:?}", line);
		}
	}
}

#[derive(Debug)]
struct AttendeeComfort  {
	name: String,
	happiness: isize,
	neighbor_name: String,
}

impl AttendeeComfort {
	fn parse(line: &str) -> AttendeeComfort {
		let re = Regex::new(REGEX_STRING).unwrap();
		let captures = re.captures(line).unwrap();
		
		let attendee: String = (&captures["attendee"]).to_string();
		let lose_gain: String = (&captures["lose_gain"]).to_string();
		let mut value: isize = (&captures["value"]).parse::<isize>().unwrap();
		let neighbor: String = (&captures["neighbor"]).to_string();
		
		match lose_gain.as_str() {
			"lose" => value = -value,
			_ => {},
		}
		
		AttendeeComfort {
			name: attendee,
			happiness: value,
			neighbor_name: neighbor,
		}
	}
}

fn get_attendees_confort(inputs: &Vec<String>) -> Vec<AttendeeComfort> {
	let mut attendees_confort: Vec<AttendeeComfort> = Vec::new();
	
	for line in inputs {
		attendees_confort.push(AttendeeComfort::parse(&line));
	}
	
	attendees_confort
}

fn get_attendees_list(attendees_confort: &Vec<AttendeeComfort>) -> Vec<String> {
	let mut attendees: Vec<String> = Vec::new();
	
	for attendee_confort in attendees_confort {
		if !attendees.contains(&attendee_confort.name) {
			attendees.push(attendee_confort.name.clone());
		}
	}
	
	attendees
}

fn create_and_fill_matrice
(attendees: Vec<String>, attendees_confort: &Vec<AttendeeComfort>)
-> Matrice<String, isize> {
	let mut matrice: Matrice<String, isize> = Matrice::new(attendees);
	
	for ac in attendees_confort {
		matrice.set(&ac.name, &ac.neighbor_name, ac.happiness);
	}
	
	matrice
}

fn get_combinations<T>(list: Vec<T>) -> Vec<Vec<T>>
where
	T: Clone
{
	let mut result: Vec<Vec<T>> = Vec::new();
	
	if list.len() == 1 {
		result.push(list);
	}
	else {
		for index in 0..list.len() {
			let mut list_copy = list.clone();
			let actual = list_copy.remove(index);
			
			for sub_vec in get_combinations(list_copy) {
				let mut tmp = sub_vec.clone();
				tmp.insert(0, actual.clone());
				result.push(tmp);
			}
		}
	}
	
	result
}

fn loop_back(path: &Vec<String>) -> Vec<String> {
	let mut loop_back_path: Vec<String> = path.clone();
	
	loop_back_path.push(path[0].clone());
	loop_back_path.append(&mut {
		let mut tmp: Vec<String> = path.clone();
		tmp.reverse();
		tmp
	});
	
	loop_back_path
}

fn get_max_happiness<'a>
(attendees_list: &'a Vec<String>, matrice: &'a Matrice<String, isize>)
-> (Vec<String>, isize) {
	let mut result: (Vec<String>, isize) = (vec![], 0);
	
	let paths: Vec<Vec<String>> = get_combinations(attendees_list.clone());
	for path in paths {
		let path: Vec<String> = loop_back(&path);
		let path_data: Vec<&isize> = matrice.get_path_data(&path);
		let path_sum: isize = path_data.iter().map(|&&val| val).sum::<isize>();
		
		if result.1 < path_sum {
			result = (path, path_sum);
		}
	}
	
	result
}

fn add_me_to_attendees
(attendees_list: &mut Vec<String>, ac: &mut Vec<AttendeeComfort>) {
	for attendee in &mut *attendees_list {
		ac.push(
			AttendeeComfort {
				name: "Me".to_string(),
				happiness: 0,
				neighbor_name: attendee.clone(),
			}
		);
		ac.push(
			AttendeeComfort {
				name: attendee.clone(),
				happiness: 0,
				neighbor_name: "Me".to_string(),
			}
		);
	}
	
	attendees_list.push("Me".to_string());
}

pub fn resolve_puzzle_one(inputs: Vec<String>) -> isize {
	let ac: Vec<AttendeeComfort> = get_attendees_confort(&inputs);
	let attendees_list: Vec<String> = get_attendees_list(&ac);
	let matrice: Matrice<String, isize> =
		create_and_fill_matrice(attendees_list.clone(), &ac);
	get_max_happiness(&attendees_list, &matrice).1
}

pub fn resolve_puzzle_two(inputs: Vec<String>) -> isize {
	let mut ac: Vec<AttendeeComfort> = get_attendees_confort(&inputs);
	let mut attendees_list: Vec<String> = get_attendees_list(&ac);
	add_me_to_attendees(&mut attendees_list, &mut ac);
	
	let matrice: Matrice<String, isize> =
		create_and_fill_matrice(attendees_list.clone(), &ac);
	get_max_happiness(&attendees_list, &matrice).1
}
