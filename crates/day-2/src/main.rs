use std::collections::HashMap;

fn main() {
  let input = include_str!("../assets/input.txt");
  println!("Part one: {}", tester_part_one(input));
  println!("Part two: {}", tester_part_two(input));
}

fn tester_part_one(input: &str)-> i32 {
  let mut allowed_games: i32 = 0;

  let max_cubes = HashMap::from([
    ("red", 12),
    ("green", 13),
    ("blue", 14),
  ]);

  for line in input.lines() {
    let mut is_game_allowed = true;
    let (info, game) = line.split_once(":").unwrap();
    game.split(";").for_each(|set| {
      for set_match in set.trim().split(",") {
        let (value, color) = set_match.trim().split_once(" ").unwrap();
        let value = value.parse::<i32>().unwrap();
        if value > *max_cubes.get(color).unwrap() {
          is_game_allowed = false;
          break;
        }
      }
    });

    if is_game_allowed {
      let (_, game_id) = info.trim().split_once(" ").unwrap();
      allowed_games += game_id.parse::<i32>().unwrap();
    }
  }

  allowed_games
}

fn tester_part_two(input: &str)-> i32 {
  let mut games_power: i32 = 0;

  for line in input.lines() {
    let (info, game) = line.split_once(":").unwrap();
    let mut min_cubes: HashMap<&str, i32> = HashMap::new();

    game.split(";").for_each(|set| {
      for set_match in set.trim().split(",") {
        let (value, color) = set_match.trim().split_once(" ").unwrap();
        let value = value.parse::<i32>().unwrap();
        let color = color.trim();
        if min_cubes.contains_key(color) {
          if value > *min_cubes.get(color).unwrap() {
            min_cubes.insert(color, value);
          }
        } else {
          min_cubes.insert(color, value);
        }
      }
    });

    println!("{:?}", min_cubes);
    let product: i32 = min_cubes.values().product();
    games_power += product;
  }

  games_power
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_part_one() {
    let example: &str =
      "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
      Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
      Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
      Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
      Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(tester_part_one(example), 8);
  }

  #[test]
  fn exmaple_parte_two() {
    let example: &str = 
      "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
      Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
      Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
      Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
      Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(tester_part_two(example), 2286);
  }
}
