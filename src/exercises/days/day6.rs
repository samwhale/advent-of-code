use super::super::super::utils;
use std::collections::HashMap;

pub struct OrbitCalculator {
  orbits: HashMap<String, String>,
}

impl<'b> OrbitCalculator {
  pub fn new() -> Self {
    Self {
      orbits: HashMap::new(),
    }
  }

  pub fn run(&mut self, message: Vec<String>) -> u32 {
    self.parse_data(message);
    self.count_orbits()
  }

  fn parse_data(&mut self, message: Vec<String>) {
    for orbit in message.iter() {
      let parsed_orbit: Vec<&str> = orbit.split(")").collect();
      let center = parsed_orbit[0];
      let orbiter = parsed_orbit[1];
      self.orbits.insert(orbiter.to_string(), center.to_string());
    }
  }

  fn count_orbits_from_node(&self, node: &String) -> u32 {
    let center = self.orbits.get(node);
    match center {
      Some(orbitee) => 1 + self.count_orbits_from_node(orbitee),
      None => 0,
    }
  }

  fn count_orbits(&self) -> u32 {
    let mut num_orbits: u32 = 0;
    for (_orbiter, center) in self.orbits.iter() {
      num_orbits += 1 + self.count_orbits_from_node(center);
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

  fn transfers_needed_test() {
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
    orbits.push(String::from("K)YOU"));
    orbits.push(String::from("I)SAN"));

    // assert_eq!(get_min_transfers(orbits), 4);
  }
}
