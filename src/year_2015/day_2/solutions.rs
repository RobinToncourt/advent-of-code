#![allow(dead_code)]

pub mod utils {
    use super::Present;

    fn string_to_present(present_string: &str) -> Present {
        let splits: Vec<&str> = present_string.split("x").collect();
        
        let length: u32 = splits[0].parse::<u32>().unwrap();
        let width: u32 = splits[1].parse::<u32>().unwrap();
        let height: u32 = splits[2].parse::<u32>().unwrap();
        
        Present {length, width, height}
    }
    
    pub fn inputs_to_presents(inputs: Vec<String>) -> Vec<Present> {
        let mut presents: Vec<Present> = Vec::new();
        
        inputs.iter().for_each(|input| presents.push(string_to_present(input.as_str())));
        
        presents
    }
}

pub struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    pub const fn new(length: u32, width: u32, height: u32,) -> Self {
        Self { length, width, height }
    }
    
    pub fn to_string(&self) -> String {
        format!("{}x{}x{}", self.length, self.width, self.height)
    }
    
    pub const fn calculate_surface_area(&self) -> u32 {
        2*self.length*self.width + 2*self.width*self.height + 2*self.height*self.length
    }
    
    pub fn get_smallest_side_area(&self) -> u32 {
        let surfaces_area: Vec<u32> = vec![self.length*self.width, self.width*self.height, self.height*self.length];
        *surfaces_area.iter().min().unwrap()
    }
    
    pub fn get_wrapping_paper_needed(&self) -> u32 {
        self.calculate_surface_area() + self.get_smallest_side_area()
    }
    
    pub fn get_smallest_perimeter(&self) -> u32 {
        let mut sides: Vec<u32> = vec![self.length, self.width, self.height];
        sides.sort();
        
        2 * sides[0] + 2 * sides[1]
    }
    
    pub fn get_volume(&self) -> u32 {
        self.length * self.width * self.height
    }
    
    pub fn get_ribbon_needed(&self) -> u32 {
        self.get_smallest_perimeter() + self.get_volume()
    }
}

pub fn wrapping_paper_order(presents: Vec<Present>) -> u32 {
    let mut wrapping_paper_total: u32 = 0;
    
    presents.iter().for_each(|present| wrapping_paper_total += present.get_wrapping_paper_needed());
    
    wrapping_paper_total
}

pub fn ribbon_order(presents: Vec<Present>) -> u32 {
    let mut ribbon_total: u32 = 0;
    
    presents.iter().for_each(|present| ribbon_total += present.get_ribbon_needed());
    
    ribbon_total
}