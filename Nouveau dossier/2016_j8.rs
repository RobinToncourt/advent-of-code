#![allow(dead_code)]

use std::cmp;
use regex::Regex;

const RECT_REGEX_STR: &str =
    r"rect (?<width>\w+)x(?<height>\w+)";
const ROTATE_ROW_REGEX_STR: &str =
    r"rotate row y=(?<nb_row>\w+) by (?<shift>\w+)";
const ROTATE_COLUMN_REGEX_STR: &str =
    r"rotate column x=(?<nb_column>\w+) by (?<shift>\w+)";

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

struct Screen {
    width: usize,
    height: usize,
    pixels: Vec<Vec<bool>>,
}

fn rotate_vec<T>(vec: Vec<T>, shift: usize) -> Vec<T> {
    let len = vec.len();
    let mut result: Vec<Option<T>> = Vec::new();
    (0..len).for_each(|_| result.push(None));
    
    for (index, val) in vec.into_iter().enumerate() {
        result[(index + shift) % len] = Some(val);
    }
    
    result.into_iter().map(|e| e.unwrap()).collect::<Vec<T>>()
}

impl Screen {
    fn new(x: usize, y: usize) -> Self {
        Self {
            width: x,
            height: y,
            pixels: vec![vec![false; y]; x],
        }
    }
    
    fn rect(&mut self, width: usize, height: usize) {
        let width = cmp::min(self.width, width);
        let height = cmp::min(self.height, height);
        
        for x in 0..width {
            for y in 0..height {
                self.pixels[x][y] = true;
            }
        }
    }
    
    fn rotate_row(&mut self, nb_row: usize, shift: usize) {
        let mut row: Vec<bool> = Vec::new();
		
		for column in &self.pixels {
			row.push(column[nb_row]);
		}
		
		let new_row = rotate_vec(row, shift);
		
		for column_index in 0..self.pixels.len() {
			self.pixels[column_index][nb_row] = new_row[column_index];
		}
    }
    
    fn rotate_column(&mut self, nb_column: usize, shift: usize) {
        let column = self.pixels[nb_column].clone();
        let new_column = rotate_vec(column, shift);
		self.pixels[nb_column] = new_column;
    }
    
	fn get_nb_light_lit(&self) -> usize {
		let mut light_lit: usize = 0;
		
		self.pixels
			.iter()
			.for_each(
				|column| light_lit += column.iter()
					.filter(
						|&&light| light
					).count()
			);
		
		light_lit
	}
	
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.pixels[x][y].then(|| '#').unwrap_or('.'));
            }
            println!();
        }
    }
}

fn instructions_to_nb_lights_lit(input: &str) -> usize {
    let mut screen = Screen::new(WIDTH, HEIGHT);
    let rect_regex = Regex::new(RECT_REGEX_STR).unwrap();
    let rotate_row = Regex::new(ROTATE_ROW_REGEX_STR).unwrap();
    let rotate_column = Regex::new(ROTATE_COLUMN_REGEX_STR).unwrap();
    
    for line in input.lines() {
        if let Some(capture) = rect_regex.captures(line) {
            let width: usize = capture["width"].parse::<usize>().unwrap();
            let height: usize = capture["height"].parse::<usize>().unwrap();
            
            screen.rect(width, height);
        }
        else if let Some(capture) = rotate_row.captures(line) {
            let nb_row: usize = capture["nb_row"].parse::<usize>().unwrap();
            let shift: usize = capture["shift"].parse::<usize>().unwrap();
            
            screen.rotate_row(nb_row, shift);
        }
        else if let Some(capture) = rotate_column.captures(line) {
            let nb_column: usize = capture["nb_column"].parse::<usize>().unwrap();
            let shift: usize = capture["shift"].parse::<usize>().unwrap();
            
            screen.rotate_column(nb_column, shift);
        }
    }
    
    screen.print();
    screen.get_nb_light_lit()
}

fn main() {
    println!("Advent of code");
    println!("Year 2016, day 08");
    
    println!("{}", instructions_to_nb_lights_lit(INPUT));
}

#[cfg(test)]
mod test {
    use crate::{
        
    };

}

const EXAMPLE: &str = "rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";
