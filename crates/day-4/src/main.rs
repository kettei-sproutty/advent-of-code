use std::collections::HashMap;

fn main() {
  let input = include_str!("../assets/input.txt");
  println!("Part one: {}", tester_part_one(input));
  println!("Part two: {}", tester_part_two(input));
}

fn tester_part_one(input: &str) -> i32 {
  let mut count: i32 = 0;

  for line in input.trim().lines() {
    let game = line.split_once(':').unwrap().1;
    let (winning_numbers, played_numbers) = game.split_once('|').unwrap();
    let winning_numbers = winning_numbers.trim().split(' ').map(|number| number.trim()).collect::<Vec<_>>();
    let played_numbers = played_numbers.trim().split(' ').map(|number| number.trim()).collect::<Vec<_>>();

    count += played_numbers.iter().fold(0, |accumulator, played_number| {
      if played_number.trim() == "" {
        return accumulator
      }

      if winning_numbers.contains(played_number) {
        if accumulator.eq(&0) {
          accumulator + 1
        } else {
          accumulator * 2
        }
      } else {
        accumulator
      }
    });
  };

  count
}

fn tester_part_two(input: &str) -> i32 {
  let mut copies: HashMap<usize, i32> = HashMap::new();
  
  for (line_index, line) in input.trim().lines().enumerate() {
    let game = line.split_once(':').unwrap().1;
    let (winning_numbers, played_numbers) = game.split_once('|').unwrap();
    let winning_numbers = winning_numbers.trim().split(' ').map(|number| number.trim()).collect::<Vec<_>>();
    let played_numbers = played_numbers.trim().split(' ').map(|number| number.trim()).collect::<Vec<_>>();
  
    let played_cards = copies.get(&line_index).unwrap_or(&0);

    for _ in 0..=*played_cards {
      let _ = played_numbers.iter().fold(1, |accumulator, played_number| {
        if played_number.trim() == "" {
          return accumulator
        }

        let next_index = line_index + accumulator;

        if winning_numbers.contains(played_number) {
          let next_card = copies.get(&next_index).unwrap_or(&0);
          copies.insert(next_index, next_card + 1);

          return accumulator + 1
        }

        accumulator
      });

    }
  }

  let sum: i32 = copies.values().sum();
  sum + input.lines().count() as i32
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_part_one() {
    let example: &str = include_str!("../assets/example.txt");

    assert_eq!(tester_part_one(example), 13);
  }

  #[test]
  fn example_part_two() {
    let example: &str = include_str!("../assets/example.txt");

    assert_eq!(tester_part_two(example), 30);
  }
}
