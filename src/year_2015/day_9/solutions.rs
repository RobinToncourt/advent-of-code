#![allow(dead_code)]
use regex::Regex;

struct Matrice {
	labels: Vec<String>,
	content: Vec<Vec<usize>>,
}

impl Matrice {
	fn new(labels: Vec<String>) -> Matrice {
		let size: usize = labels.len();
		let content: Vec<Vec<usize>> = vec![vec![0; size]; size];
		
		Matrice {
			labels,
			content,
		}
	}
	
	fn get_key_pos(&self, key: &str) -> usize {
		self.labels
			.iter()
			.enumerate()
			.find(|&label| label.1 == key)
			.unwrap()
			.0
	}
	
	fn set(&mut self, x_key: &str, y_key: &str, value: usize) {
		let x_index: usize = self.get_key_pos(x_key);
		let y_index: usize = self.get_key_pos(y_key);
		
		self.content[x_index][y_index] = value;
		self.content[y_index][x_index] = value;
	}
	
	fn get(&self, x_key: &str, y_key: &str) -> usize {
		let x_index: usize = self.get_key_pos(x_key);
		let y_index: usize = self.get_key_pos(y_key);
		
		self.content[x_index][y_index]
	}
	
	fn get_route_length(&self, route: &Vec<String>) -> usize {
		let mut route_length: usize = 0;
		
		for i in 0..route.len()-1 {
			route_length += self.get(&route[i], &route[i+1]);
		}
		
		route_length
	}
}

#[derive(Debug)]
struct LocationPair {
	from: String,
	to: String,
	distance: usize,
}

impl LocationPair {
	fn parse(line: &str) -> LocationPair {
		let re = Regex::new(
			r"(?<from>\w+) to (?<to>\w+) = (?<distance>\w+)"
		).unwrap();
		
		let captures = re.captures(line).unwrap();
		
		let from: String = (&captures["from"]).to_string();
		let to: String = (&captures["to"]).to_string();
		let distance: usize = (&captures["distance"]).parse::<usize>().unwrap();
		
		LocationPair {
			from,
			to,
			distance,
		}
	}
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

fn create_location_pair_list(inputs: &Vec<String>) ->Vec<LocationPair> {
	let mut location_pair_list: Vec<LocationPair> = Vec::new();
	
	for line in inputs {
		let location_pair: LocationPair = LocationPair::parse(&line);
		location_pair_list.push(location_pair);
	}
	
	location_pair_list
}

fn get_unique_cities(location_pair_list: &Vec<LocationPair>) -> Vec<String> {
	let mut cities: Vec<String> = Vec::new();
	
	for location_pair in location_pair_list {
		if !cities.contains(&location_pair.from) {
			cities.push(location_pair.from.clone());
		}
		if !cities.contains(&location_pair.to) {
			cities.push(location_pair.to.clone());
		}
	}
	
	cities
}

fn create_matrice
(location_pair_list: &Vec<LocationPair>, cities: Vec<String>) -> Matrice
{
	let mut matrice: Matrice = Matrice::new(cities.clone());
	
	for location_pair in location_pair_list {
		let from: &str = &location_pair.from;
		let to: &str = &location_pair.to;
		let distance: usize = location_pair.distance;
		
		matrice.set(from, to, distance);
	}
	
	matrice
}

pub fn get_shortest_route(inputs: &Vec<String>) -> (Vec<String>, usize) {
	let location_pair_list: Vec<LocationPair> =
		create_location_pair_list(inputs);
	let cities: Vec<String> = get_unique_cities(&location_pair_list);
	let matrice: Matrice = create_matrice(&location_pair_list, cities.clone());
	
	let mut shortest_route: (Vec<String>, usize) =
		(Vec::<String>::new(), std::usize::MAX);
	let combinations: Vec<Vec<String>> = get_combinations(cities);
	
	for comb in &combinations {
		let route_length: usize = matrice.get_route_length(comb);
		
		if route_length < shortest_route.1 {
			shortest_route = (comb.clone(), route_length);
		}
	}
	
	shortest_route
}

pub fn get_longest_route(inputs: &Vec<String>) -> (Vec<String>, usize) {
	let location_pair_list: Vec<LocationPair> =
		create_location_pair_list(inputs);
	let cities: Vec<String> = get_unique_cities(&location_pair_list);
	let matrice: Matrice = create_matrice(&location_pair_list, cities.clone());
	
	let mut longest_route: (Vec<String>, usize) =
		(Vec::<String>::new(), 0);
	let combinations: Vec<Vec<String>> = get_combinations(cities);
	
	for comb in &combinations {
		let route_length: usize = matrice.get_route_length(comb);
		
		if route_length > longest_route.1 {
			longest_route = (comb.clone(), route_length);
		}
	}
	
	longest_route
}