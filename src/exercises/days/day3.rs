use super::super::super::utils;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::hash_set::Intersection;
use std::collections::hash_map::RandomState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn get_distance_to_origin(coord: &Coordinate) -> i32 {
    coord.x.abs() + coord.y.abs()
}

fn get_smallest_distance(distances: &HashMap<Coordinate, u32>, location: &Coordinate, distance: u32) -> u32 {
    match distances.get(location) {
        Some(dist) => {
            if distance < *dist {
                distance
            } else {
                *dist
            }
        },
        None => {
            distance
        }
    }
}

fn get_coordinates(wire_path: &str) -> (HashSet<Coordinate>, HashMap<Coordinate, u32>) {
    let mut coordinates = HashSet::new();
    let mut distances: HashMap<Coordinate, u32> = HashMap::new();

    let mut current_location = Coordinate { x: 0, y: 0 } ;
    let mut current_distance = 0;

    // initialize first point
    coordinates.insert(current_location);
    distances.insert(current_location, current_distance);

    for command in wire_path.split(",") {
        let direction = &command[0..1];
        let dist = &command[1..].parse::<i32>().unwrap();

        match direction {
            "U" => {
                for _i in 0..*dist {
                    current_location.y += 1;
                    current_distance += 1;
                    coordinates.insert(current_location);
                    distances.insert(current_location, get_smallest_distance(&distances, &current_location, current_distance));
                }
            }
            "R" => {
                for _i in 0..*dist {
                    current_location.x += 1;
                    current_distance += 1;
                    coordinates.insert(current_location);
                    distances.insert(current_location, get_smallest_distance(&distances, &current_location, current_distance));
                }
            }
            "D" => {
                for _i in 0..*dist {
                    current_location.y -= 1;
                    current_distance += 1;
                    coordinates.insert(current_location);
                    distances.insert(current_location, get_smallest_distance(&distances, &current_location, current_distance));
                }
            }
            "L" => {
                for _i in 0..*dist {
                    current_location.x -= 1;
                    current_distance += 1;
                    coordinates.insert(current_location);
                    distances.insert(current_location, get_smallest_distance(&distances, &current_location, current_distance));
                }
            }
            _ => (),
        }
    }

    (coordinates, distances)
}

fn get_fewest_combined_steps(intersecting_coords: Intersection<Coordinate, RandomState>, wire_1_distances: HashMap<Coordinate, u32>, wire_2_distances: HashMap<Coordinate, u32>) -> u32 {
    intersecting_coords.filter_map(|coord| {
        let wire_1_distance = wire_1_distances.get(coord).unwrap();
        let wire_2_distance = wire_2_distances.get(coord).unwrap();
        let distance = wire_1_distance + wire_2_distance;

        return if distance > 0 { Some(distance) } else { None }
    }).min().unwrap()
}

fn get_min_manhattan_distance(intersecting_coords: Intersection<Coordinate, RandomState>) -> i32 {
    let min_distance = intersecting_coords
        .filter_map(|coord| {
            let min_distance = get_distance_to_origin(coord);

            return if min_distance != 0 { Some(min_distance) } else { None };
        })
        .min()
        .unwrap();

    min_distance
}

fn process_wires(wire_1: &str, wire_2: &str) -> (i32, u32) {
    let (wire_1_coords, wire_1_dists) = get_coordinates(wire_1);
    let (wire_2_coords, wire_2_dists) = get_coordinates(wire_2);

    let intersecting_coords = wire_1_coords.intersection(&wire_2_coords);

    let min_distance = get_min_manhattan_distance(intersecting_coords.clone());
    let fewest_steps = get_fewest_combined_steps(intersecting_coords, wire_1_dists, wire_2_dists);

    (min_distance, fewest_steps)
}

pub fn main() {
    let message = utils::read_file_into_vector("src/exercises/data/data-day3.txt");

    let wire_1 = message.get(0);
    let wire_2 = message.get(1);

    let (min_distance, fewest_steps) = process_wires(wire_1.unwrap().trim(), wire_2.unwrap().trim());

    println!("--- Day 3 ---");
    println!("min_distance: {}", min_distance);
    println!("fewest_steps: {}", fewest_steps);
}

// ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_wires_test() {
        assert_eq!(
            process_wires("R8,U5,L5,D3", "U7,R6,D4,L4"),
            (6, 30)
        );
        assert_eq!(
            process_wires(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            (159, 610)
        );
        assert_eq!(
            process_wires(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            (135, 410)
        );
    }

    #[test]
    fn get_coordinates_test() {
        let mut result = HashSet::new();
        result.insert(Coordinate{ x: 0, y: 0 });
        result.insert(Coordinate{ x: 1, y: 0 });
        result.insert(Coordinate{ x: 2, y: 0 });
        result.insert(Coordinate{ x: 2, y: 1 });
        result.insert(Coordinate{ x: 2, y: -1 });
        result.insert(Coordinate{ x: 1, y: -1 });

        assert_eq!(get_coordinates("R2,U1,D2,L1").0, result);
    }
}
