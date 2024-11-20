#![allow(dead_code)]

use std::collections::HashSet;

pub fn at_least_one_present(directions: &str) -> u32 {
    let mut houses: HashSet<(i32, i32)> = HashSet::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    houses.insert((x, y));

    directions.chars().for_each(
        |c| {
            match c {
                '^' => y+=-1,
                'v' => y+=1,
                '<' => x+=-1,
                '>' => x+=1,
                _ => x+=0,
            };
            houses.insert((x, y));
        }
    );
    
    houses.len() as u32
}

pub fn at_least_one_present_with_robo_santa(directions: &str) -> u32 {
    let mut houses: HashSet<(i32, i32)> = HashSet::new();
    let mut x_santa: i32 = 0;
    let mut y_santa: i32 = 0;
    let mut x_robot: i32 = 0;
    let mut y_robot: i32 = 0;
    
    houses.insert((x_santa, y_santa));

    for (index, c) in directions.chars().enumerate() {
        let x: &mut i32;
        let y: &mut i32;

        if index & 1 == 1 {
            x = &mut x_santa;
            y = &mut y_santa;
        }
        else {
            x = &mut x_robot;
            y = &mut y_robot;

        }

        match c {
            '^' => *y+=-1,
            'v' => *y+=1,
            '<' => *x+=-1,
            '>' => *x+=1,
            _ => *x+=0,
        };

        houses.insert((*x, *y));
    }
    
    houses.len() as u32
}