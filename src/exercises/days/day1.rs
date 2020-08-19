use super::super::super::utils;

pub mod fuel_counter {
    // Calculates the fuel requirement for a given mass
    pub fn calculate_fuel(mass: i32) -> i32 {
        mass / 3 - 2
    }

    // Calculates the fuel requirement for a given mass
    // Recursively adds fuel for given mass of fuel
    pub fn calculate_fuel_with_fuel_mass(mass: i32) -> i32 {
        let fuel = calculate_fuel(mass);
        match fuel <= 0 {
            true => 0,
            false => fuel + calculate_fuel_with_fuel_mass(fuel),
        }
    }
}

pub fn main(ignore_fuel_mass: bool) {
    let data: Vec<String> = utils::read_file_into_vector("./src/exercises/data/data-day1.txt");
    let mut result: i32 = 0;

    for val in &data {
        let mass = val.parse::<i32>().unwrap();

        if ignore_fuel_mass {
            result += fuel_counter::calculate_fuel(mass);
        } else {
            let calc_mass = fuel_counter::calculate_fuel_with_fuel_mass(mass);
            result += calc_mass
        }
    }

    println!("--- Day 1 ---");
    println!("Account for mass of fuel: {}", !ignore_fuel_mass);
    // println!("message: {:?}", data);
    println!("result: {:?}", result);
}
