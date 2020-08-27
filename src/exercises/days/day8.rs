use super::super::super::utils;
use std::iter::Iterator;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Layer {
  num_zeroes: u32,
  layer: Vec<u32>,
}

pub fn create_layer_from_range(input: &str, start: usize, end: usize) -> Layer {
  let mut num_zeroes = 0;
  let mut layer: Vec<u32> = Vec::new();
  let slice = &input[start..end];

  for char in slice.chars() {
    let parsed_digit = char.to_string().parse::<u32>().unwrap();
    if parsed_digit == 0 {
      num_zeroes += 1;
    };
    layer.push(parsed_digit);
  }
  Layer { layer, num_zeroes }
}

pub fn read_image(image: &str, [width, height]: [u32; 2]) -> Vec<Layer> {
  let mut processed_image: Vec<Layer> = Vec::new();
  let area = (width * height) as usize;
  let slice_length = image.len() / area;

  for i in 0..slice_length {
    let start = i * area;
    let end = start + area;
    let layer = create_layer_from_range(image, start, end);
    processed_image.push(layer);
  }
  processed_image
}

pub fn get_layer_with_fewest_zeroes(image: &Vec<Layer>) -> Layer {
  image
    .iter()
    .min_by_key(|layer| layer.num_zeroes)
    .unwrap()
    .to_owned()
}

pub fn validate_transmission(image: Vec<u32>) -> u32 {
  let mut num_ones = 0;
  let mut num_twos = 0;

  for digit in image.iter() {
    match digit {
      1 => num_ones += 1,
      2 => num_twos += 1,
      _ => {}
    }
  }

  num_ones * num_twos
}

/**
 * Find the color of the pixel that shows in the image
 *
 * 0 is black
 * 1 is white
 * 2 is transparent
 */
pub fn get_pixel_value(image: &Vec<Layer>, index: usize) -> u32 {
  for layer in image.iter() {
    match layer.layer[index] {
      0 => {
        return 0;
      }
      1 => {
        return 1;
      }
      2 => {}
      _ => {
        panic!("Non valid integer");
      }
    }
  }

  2
}

pub fn format_image(image: Vec<u32>, [num_columns, num_rows]: [u32; 2]) -> Vec<Vec<u32>> {
  let mut result = Vec::new();
  for row_index in 0..num_rows {
    let mut row = Vec::new();

    for column_index in 0..num_columns {
      let digit_index = ((row_index * num_columns) + column_index) as usize;
      row.push(image[digit_index]);
    }
    result.push(row);
  }

  result
}

pub fn create_image(image: Vec<Layer>, dimensions: [u32; 2]) -> Vec<Vec<u32>> {
  let mut result: Vec<u32> = Vec::new();
  for index in 0..image[0].layer.len() {
    let pixel_value = get_pixel_value(&image, index);
    result.push(pixel_value);
  }

  format_image(result, dimensions)
}

pub fn main() {
  println!("--- Day 8 ---");
  let message = utils::read_file_into_string("./src/exercises/data/data-day8.txt");
  let dimensions = [25, 6];
  let processed_image = read_image(&message.trim(), dimensions);
  let layer = get_layer_with_fewest_zeroes(&processed_image);

  println!(
    "num ones * num twos: {:?}",
    validate_transmission(layer.layer)
  );
  let result = create_image(processed_image, dimensions);
  println!("Message:");
  for row in result {
    println!("{:?}", row);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn read_image_test() {
    let result = vec![
      Layer {
        layer: vec![1, 2, 3, 4, 5, 6],
        num_zeroes: 0,
      },
      Layer {
        layer: vec![7, 8, 9, 0, 1, 2],
        num_zeroes: 1,
      },
    ];
    assert_eq!(read_image("123456789012", [3, 2]), result);
  }

  #[test]
  pub fn get_layer_with_fewest_zeroes_test() {
    let result = Layer {
      layer: vec![0, 0, 1, 1, 1, 1],
      num_zeroes: 2,
    };
    let input = vec![
      Layer {
        layer: vec![0, 0, 0, 1, 1, 1],
        num_zeroes: 3,
      },
      Layer {
        layer: vec![0, 0, 1, 1, 1, 1],
        num_zeroes: 2,
      },
      Layer {
        layer: vec![0, 0, 0, 0, 1, 1],
        num_zeroes: 4,
      },
    ];
    assert_eq!(get_layer_with_fewest_zeroes(&input), result);
  }

  #[test]
  pub fn validate_transmission_test() {
    assert_eq!(validate_transmission(vec![0, 0, 1, 1, 0, 2, 2, 0, 1, 2]), 9);
    assert_eq!(
      validate_transmission(vec![0, 1, 1, 1, 0, 2, 2, 0, 1, 2]),
      12
    );
    assert_eq!(validate_transmission(vec![1, 1, 1, 1, 1]), 0);
    assert_eq!(validate_transmission(vec![2, 2, 2, 2]), 0);
  }

  #[test]
  pub fn create_image_test() {
    let image = read_image("0222112222120000", [2, 2]);
    let result = vec![vec![0, 1], vec![1, 0]];
    assert_eq!(create_image(image, [2, 2]), result);
  }
}
