use super::super::super::utils;
use std::collections::HashMap;

pub struct OrbitCalculator {
  orbits: HashMap<String, Vec<String>>,
}

impl OrbitCalculator {
  pub fn new() -> Self {
    Self {
      orbits: HashMap::new(),
    }
  }

  pub fn run(&mut self, message: Vec<String>) -> u32 {
    self.parse_data(message);
    self.count_orbits()
  }

  fn get_orbits(&self, key: &str) -> Vec<String> {
    match self.orbits.get(key) {
      Some(value) => value.clone(),
      None => vec![],
    }
  }

  fn parse_data(&mut self, message: Vec<String>) {
    for orbit in message.iter() {
      let parsed_orbit: Vec<&str> = orbit.split(")").collect();
      let mut orbits = self.get_orbits(parsed_orbit[0]);
      orbits.push(parsed_orbit[1].to_string());
      self.orbits.insert(parsed_orbit[0].to_string(), orbits);
    }
  }

  fn get_orbiters(&self, orbitee: &String) -> u32 {
    let value = &self.orbits.get(orbitee);
    match value {
      Some(orbiters) => {
        let mut num_orbits = orbiters.len() as u32;
        for orbiter in orbiters.iter() {
          num_orbits += self.get_orbiters(orbiter);
        }
        num_orbits
      }
      None => 0,
    }
  }

  fn count_orbits(&self) -> u32 {
    // (center | orbiter)
    let mut num_orbits: u32 = 0;
    for (_orbitee, orbiters) in self.orbits.iter() {
      num_orbits += orbiters.len() as u32;

      for orbiter in orbiters.iter() {
        num_orbits += self.get_orbiters(orbiter);
      }
    }
    num_orbits
  }
}

pub fn get_num_orbits(message: Vec<String>) -> u32 {
  let mut orbit_calculator = OrbitCalculator::new();
  orbit_calculator.run(message)
}

pub fn main() {
  println!("--- Day 6 ---");
  let orbits = utils::read_file_into_vector("./src/exercises/data/data-day6.txt");
  println!("Num direct + indirect orbits: {}", get_num_orbits(orbits));
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn counts_orbits() {
    let mut orbits: Vec<String> = Vec::new();
    orbits.push(String::from("COM)B"));
    orbits.push(String::from("B)C"));
    orbits.push(String::from("C)D"));
    orbits.push(String::from("D)E"));
    orbits.push(String::from("E)F"));
    orbits.push(String::from("B)G"));
    orbits.push(String::from("G)H"));
    orbits.push(String::from("D)I"));
    orbits.push(String::from("E)J"));
    orbits.push(String::from("J)K"));
    orbits.push(String::from("K)L"));

    assert_eq!(get_num_orbits(orbits), 42);
  }
}
