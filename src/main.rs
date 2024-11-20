#![allow(dead_code)]
mod utils;
mod year_2015;

use crate::utils::utils::load_from_file;

use std::cell::RefCell;
use year_2015::day_7::solutions::*;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    day_20();
}

const PRESENTS_OBJECTIVE: usize = 29_000_000;

fn day_20() {

    for house_number in 1..PRESENTS_OBJECTIVE {
        let presents: usize = factors_presents(house_number);

        println!("{house_number}");

        if presents >= PRESENTS_OBJECTIVE {
            println!("{house_number}");
            break;
        }
    }

}

fn factors_presents(value: usize) -> usize {
    let mut presents: usize = 10;

    for factor in 2..=value/2 {
        if value % factor == 0 {
            presents += factor * 10;
        }
    }

    presents
}

fn get_all_factors(value: usize) -> Vec<usize> {
    let mut factors: Vec<usize> = Vec::new();

    for factor in 1..=value/2 {
        if value % factor == 0 {
            factors.push(factor);
        }
    }

    factors
}

fn presents_delivered(elfs: &Vec<usize>) -> usize {
    let mut presents: usize = 0;

    for elf in elfs {
        presents += elf * 10;
    }

    presents
}


// ------------------------------------------------------------------

fn day_7_save() {
    let mut wire_list: Vec<RefCell<Wire>> = Vec::new();

    println!("Création des fils...");

    // Crée des fils sans connexion.
    for line in INPUT.lines() {
        let splits: Vec<&str> = line.split(" -> ").collect::<Vec<&str>>();

        let wire = Wire {
            name: splits[1].to_owned(),
            signal: None,
            connection: Box::new(Connection::Nothing),
        };

        wire_list.push(RefCell::new(wire));
    }

    println!("Connexions des fils...");

    INPUT.lines().for_each(|line| set_connection(&wire_list, line));

    //wire_list.iter().for_each(|wire| println!("{} -> {}", wire.borrow().display_connection(), wire.borrow().name));

    println!("Recherche la valeur d'un fil...");

    let mut wire_value: HashMap<String, u16> = HashMap::new();

    get_wire_by_name(&wire_list, "b").unwrap().borrow_mut().signal = Some(16076);
    println!("{}", get_wire_by_name(&wire_list, "a").unwrap().borrow().get_signal(&mut wire_value));
}
