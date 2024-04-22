#![allow(dead_code)]

pub fn which_floor(directions: &str) -> i32 {
    let mut floor: i32 = 0;

    directions.chars().for_each(
            |c: char| floor += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
        );
        
    floor
}

pub fn position_enter_basement(directions: &str) -> usize {
    let mut floor: i32 = 0;
    
    for (i, c) in directions.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if floor < 0 {
            return i+1;
        }
    }
    
    0
}