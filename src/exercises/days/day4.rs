use super::super::super::utils;

// part 1
fn has_adjacent(value: &str) -> bool {
    let mut prev_digit = String::from("");

    for digit in value.chars() {
        let digit = digit.to_string();

        if prev_digit == digit {
            return true;
        }

        prev_digit = digit;
    }

    false
}

// part 2
fn has_double(value: &str) -> bool {
    let mut split: Vec<String> = vec![];

    for i in 0..value.len() {
        let char = &value[i..i + 1];

        if let Some(string) = split.last_mut() {
            if string[0..1] == char.to_string() {
                *string = format!("{}{}", string, char);
            } else {
                split.push(char.to_string());
            }
        } else {
            split.push(char.to_string());
        }
    }

    for string in split.iter() {
        if string.len() == 2 {
            return true;
        }
    }
    false
}

fn is_never_decreasing(value: &str) -> bool {
    let mut prev_char = value[0..1].to_string();

    for char in value.chars() {
        let digit = char.to_string().parse::<u32>().unwrap();
        let prev_digit = prev_char.parse::<u32>().unwrap();

        if prev_digit > digit {
            return false;
        }

        prev_char = char.to_string();
    }

    true
}

fn validate(value: &str) -> bool {
    if value.len() != 6 {
        return false;
    }

    has_adjacent(value) && is_never_decreasing(value) && has_double(value)
}

pub fn main() {
    let message = utils::read_file_into_string("src/exercises/data/data-day4.txt");
    let message: Vec<&str> = message.split("-").collect();
    let min = message[0].to_string();
    let max = message[1].to_string();
    let min: u32 = min.parse().unwrap();
    let max: u32 = max.parse().unwrap();

    let mut num_valid = 0;
    for i in min..max {
        if validate(&i.to_string()) {
            num_valid += 1;
        }
    }

    println!("--- Day 4 ---");
    println!("{}", num_valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_test() {
        assert_eq!(validate("111111"), false);
        assert_eq!(validate("122345"), true);
        assert_eq!(validate("1"), false);
        assert_eq!(validate("123350"), false);
        assert_eq!(validate("123789"), false);
    }

    #[test]
    fn has_adjacent_test() {
        assert_eq!(has_adjacent("11"), true);
        assert_eq!(has_adjacent("111111"), true);
        assert_eq!(has_adjacent("122345"), true);
        assert_eq!(has_adjacent("1"), false);
        assert_eq!(has_adjacent("123350"), true);
        assert_eq!(has_adjacent("123789"), false);
    }

    #[test]
    fn is_never_decreasing_test() {
        assert_eq!(is_never_decreasing("11"), true);
        assert_eq!(is_never_decreasing("111111"), true);
        assert_eq!(is_never_decreasing("122345"), true);
        assert_eq!(is_never_decreasing("1"), true);
        assert_eq!(is_never_decreasing("123350"), false);
        assert_eq!(is_never_decreasing("1023"), false);
        assert_eq!(is_never_decreasing("123789"), true);
    }

    #[test]
    fn has_double_test() {
        assert_eq!(has_double("111111"), false);
        assert_eq!(has_double("112233"), true);
        assert_eq!(has_double("1"), false);
        assert_eq!(has_double("123444"), false);
        assert_eq!(has_double("111122"), true);
    }
}
