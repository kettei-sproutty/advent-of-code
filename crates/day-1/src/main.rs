fn main() {
  let input = include_str!("../assets/input.txt");
  println!("Part 1: {}", tester_part_one(input));
  println!("Part 2: {}", tester_part_two(input));
}

pub fn tester_part_one(input: &str) -> i32 {
  let mut sum: i32 = 0;

  for line in input.lines() {
    let first_digit = match line.chars().find(|char| char.is_numeric()) {
      Some(char) => char,
      None => continue,
    };

    let last_digit = line.chars().rfind(|char| char.is_numeric()).unwrap();

    let line_sum = format!("{}{}", first_digit, last_digit);
    sum += line_sum.parse::<i32>().unwrap_or(0);
  }

  sum
}

pub fn tester_part_two(input: &str) -> i32 {
  let mut sum: i32 = 0;

  for line in input.lines() {
    let line = String::from(
      line
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
        .as_str(),
    );

    let first_digit = match line.chars().find(|char| char.is_numeric()) {
      Some(char) => char,
      None => continue,
    };

    let last_digit = line.chars().rfind(|char| char.is_numeric()).unwrap();

    let line_sum = format!("{}{}", first_digit, last_digit);

    sum += line_sum.parse::<i32>().unwrap_or(0);
  }

  sum
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_part_one() {
    let example_value: &str = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

    assert_eq!(tester_part_one(example_value), 142);
  }

  #[test]
  fn example_part_two() {
    let example_value: &str = "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";

    assert_eq!(tester_part_two(example_value), 281);
  }
}
