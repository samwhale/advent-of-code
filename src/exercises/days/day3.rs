use super::super::super::utils;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coordinate(i32, i32);

fn get_distance_to_origin(coord: &Coordinate) -> i32 {
    coord.0.abs() + coord.1.abs()
}

fn get_coordinates(wire_path: &str) -> HashSet<Coordinate> {
    let mut set = HashSet::new();
    let mut current_location = Coordinate(0, 0);
    set.insert(current_location);

    for command in wire_path.split(",") {
        let direction = &command[0..1];
        let dist = &command[1..].parse::<i32>().unwrap();

        match direction {
            "U" => {
                let new_y = current_location.1 + dist;

                for _i in current_location.1..new_y {
                    current_location.1 += 1;
                    set.insert(current_location);
                }
            }
            "R" => {
                let new_x = current_location.0 + dist;

                for _i in current_location.0..new_x {
                    current_location.0 += 1;
                    set.insert(current_location);
                }
            }
            "D" => {
                let new_y = current_location.1 - dist;

                for _i in new_y..current_location.1 {
                    current_location.1 -= 1;
                    set.insert(current_location);
                }
            }
            "L" => {
                let new_x = current_location.0 - dist;

                for _i in new_x..current_location.0 {
                    current_location.0 -= 1;
                    set.insert(current_location);
                }
            }
            _ => (),
        }
    }

    set
}

pub fn get_min_manhattan_intersection_distance(wire_1: &str, wire_2: &str) -> i32 {
    let wire_1_coords = get_coordinates(wire_1);
    let wire_2_coords = get_coordinates(wire_2);

    let intersecting_coords = wire_1_coords.intersection(&wire_2_coords);

    let result = intersecting_coords
        .filter_map(|coord| {
            let result = get_distance_to_origin(coord);

            return if result != 0 { Some(result) } else { None };
        })
        .min()
        .unwrap();

    result
}

pub fn main() {
    let message = utils::read_file_into_vector("src/exercises/data/data-day3.txt");

    let wire_1 = message.get(0);
    let wire_2 = message.get(1);

    let min_distance =
        get_min_manhattan_intersection_distance(wire_1.unwrap().trim(), wire_2.unwrap().trim());

    println!("--- Day 3 ---");
    println!("min_distance: {:?}", min_distance);
}

// ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_min_manhattan_intersection_distance_test() {
        assert_eq!(
            get_min_manhattan_intersection_distance("R8,U5,L5,D3", "U7,R6,D4,L4"),
            6
        );
        assert_eq!(
            get_min_manhattan_intersection_distance(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
        assert_eq!(
            get_min_manhattan_intersection_distance(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }

    #[test]
    fn get_coordinates_test() {
        let mut result = HashSet::new();
        result.insert(Coordinate(0, 0));
        result.insert(Coordinate(1, 0));
        result.insert(Coordinate(2, 0));
        result.insert(Coordinate(2, 1));
        result.insert(Coordinate(2, -1));
        result.insert(Coordinate(1, -1));

        assert_eq!(get_coordinates("R2,U1,D2,L1"), result);
    }
}
