use super::super::super::utils;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Asteroid {
  x: u16,
  y: u16,
  visible_asteroids: u32,
}

pub fn get_slope(center: Asteroid, asteroid: Asteroid) -> f32 {
  (center.y - asteroid.y) as f32 / (center.x - asteroid.x) as f32
}

pub fn get_asteroids(field: Vec<String>) -> HashSet<Asteroid> {
  let mut asteroids = HashSet::new();
  for (y, row) in field.iter().enumerate() {
    for (x, char) in row.chars().enumerate() {
      if char.to_string() == "#" {
        asteroids.insert(Asteroid {
          x: x as u16,
          y: y as u16,
          visible_asteroids: 0,
        });
      }
    }
  }

  asteroids
}

pub fn main() {
  println!("--- Day 10 ---");
  let field = utils::read_file_into_vector("src/exercises/data/data-day10.txt");
  let asteroids = get_asteroids(field);
  println!("asteroids: {:?}", asteroids);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_asteroid_creation() {
    let field = vec![".#..#".to_string(), "..#..".to_string()];
    let mut result: HashSet<Asteroid> = HashSet::new();
    result.insert(Asteroid {
      x: 1,
      y: 0,
      visible_asteroids: 0,
    });
    result.insert(Asteroid {
      x: 2,
      y: 1,
      visible_asteroids: 0,
    });
    result.insert(Asteroid {
      x: 4,
      y: 0,
      visible_asteroids: 0,
    });

    assert_eq!(get_asteroids(field), result);
  }
}
