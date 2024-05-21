use std::collections::HashMap;

fn main() {
  let input = include_str!("../assets/input.txt");
  println!("Day 7, Part 1: {}", part1(input));
}

enum HandType {
  HighCard,
  OnePair,
  TwoPairs,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind
}

impl HandType {
  fn get_rank(self) -> i32 {
    match self {
      HandType::FiveOfAKind => 0,
      HandType::FourOfAKind => 1,
      HandType::FullHouse => 2,
      HandType::ThreeOfAKind => 3,
      HandType::TwoPairs => 4,
      HandType::OnePair => 5,
      HandType::HighCard => 6
    }
  }
}

struct Hand {
  cards: Vec<i32>,
  hand_type: HandType,
  bid: i32,
  /// espressed as (highest_card, index_of_highest_card)
  highest_card: (i32, i32)
}

impl HandType {
  fn analyze_hand(hand: &str) -> Self {
    let (cards, bid) = hand.split_once(' ').unwrap();
    let cards = hand.split("").map(|card| card.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let bid = bid.parse::<i32>().unwrap();

    
  }
}


fn part1(input: &str) -> i32 {
  let mut total_winning: i32 = 0;
  let mut hands_by_rank: HashMap<i32, (&str, i32)> = HashMap::new();

  for lines in input.lines() {
      
  }

  total_winning
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_first_part() {
    let input = include_str!("../assets/example.txt");
    let first_part = part1(input);
    assert_eq!(first_part, 6440);
    assert_eq!(1, 1);
  }
}
