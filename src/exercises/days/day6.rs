use super::super::super::utils;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct OrbitCalculator {
  orbits: HashMap<String, String>,
}

impl OrbitCalculator {
  pub fn new() -> Self {
    Self {
      orbits: HashMap::new(),
    }
  }

  pub fn run(&mut self, message: Vec<String>) -> (u32, u32) {
    self.parse_data(message);
    (
      self.count_orbits(),
      self.get_distance_between_nodes(&String::from("YOU"), &String::from("SAN")),
    )
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

  fn get_path_to_center(&self, node: &String) -> HashSet<&String> {
    let mut path: HashSet<&String> = HashSet::new();
    let mut maybe_center = self.orbits.get(node);
    loop {
      match maybe_center {
        Some(center) => {
          path.insert(center);
          maybe_center = self.orbits.get(center);
        }
        None => break,
      }
    }

    path
  }

  fn get_distance_between_nodes(&self, n1: &String, n2: &String) -> u32 {
    let path1: HashSet<&String> = self.get_path_to_center(n1);
    let path2: HashSet<&String> = self.get_path_to_center(n2);
    let path_to_san: HashSet<_> = path1.symmetric_difference(&path2).collect();
    path_to_san.len() as u32
  }
}

pub fn get_orbital_data(message: Vec<String>) -> (u32, u32) {
  let mut orbit_calculator = OrbitCalculator::new();
  orbit_calculator.run(message)
}

pub fn main() {
  println!("--- Day 6 ---");
  let orbits = utils::read_file_into_vector("./src/exercises/data/data-day6.txt");
  let (total_orbits, distance_to_santa) = get_orbital_data(orbits);
  println!("Num direct + indirect orbits: {}", total_orbits);
  println!(
    "Distance to planet santa is orbiting: {}",
    distance_to_santa
  );
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

    assert_eq!(get_orbital_data(orbits).0, 42);
  }

  #[test]
  fn transfers_needed_get_orbital_data() {
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

    assert_eq!(get_orbital_data(orbits).1, 4);
  }
}
