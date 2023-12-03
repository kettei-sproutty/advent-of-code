fn main() {
  let input = include_str!("../assets/input.txt");
  println!("Part one: {}", tester_part_one(input));
  println!("Part two: {}", tester_part_two(input));
}

#[derive(Debug, Clone)]
struct Node {
  row: usize,
  columns: (usize, usize),
  value: String,
  is_counted: bool,
}

impl Node {
  fn check_surrounding(&mut self, input: &str) {
    let rows_delta = if self.row == 0 {
      self.row
    } else {
      self.row - 1
    };
    let rows = rows_delta..=(self.row + 1);
    let columns_delta = if self.columns.0 == 0 {
      self.columns.0
    } else {
      self.columns.0 - 1
    };
    let columns = columns_delta..=(self.columns.1 + 1);

    rows.for_each(|row| match input.lines().nth(row) {
      Some(line) => {
        line
          .trim()
          .chars()
          .enumerate()
          .for_each(|(char_index, char)| {
            if columns.contains(&char_index)
              && !char.is_numeric()
              && char != '.'
              && !char.is_whitespace()
            {
              self.is_counted = true;
            }
          });
      }
      None => {}
    });
  }
}

fn tester_part_one(input: &str) -> i32 {
  let mut count: i32 = 0;
  let mut nodes: Vec<Node> = Vec::new();

  for (line_index, line) in input.lines().enumerate() {
    for (char_index, char) in line.chars().enumerate() {
      if !char.is_numeric() {
        continue;
      }

      if let Some(node) = nodes.last_mut() {
        if node.columns.1 + 1 == char_index && node.row == line_index {
          node.columns.1 = char_index;
          node.value.push(char);

          if !node.is_counted {
            node.check_surrounding(input);
          }
        } else {
          nodes.push(Node {
            row: line_index,
            columns: (char_index, char_index),
            value: char.to_string(),
            is_counted: false,
          });

          let node = nodes.last_mut().unwrap();
          node.check_surrounding(input);
        }
      } else if char.is_numeric() {
        nodes.push(Node {
          row: line_index,
          columns: (char_index, char_index),
          value: char.to_string(),
          is_counted: false,
        });

        let node = nodes.last_mut().unwrap();
        node.check_surrounding(input);
      }
    }
  }

  nodes.iter().for_each(|node| {
    if node.is_counted {
      count += node.value.parse::<i32>().unwrap();
    }
  });

  count
}

#[derive(Debug, Clone)]
struct GearNumber {
  row: usize,
  column: (usize, usize),
  value: String,
}

#[derive(Debug, Clone)]
struct Gear {
  row: usize,
  column: usize,
  numbers: Vec<GearNumber>,
}

impl Gear {
  fn check_surrounding(&mut self, input: &str) {
    let rows_delta = if self.row == 0 {
      self.row
    } else {
      self.row - 1
    };
    let rows = rows_delta..=(self.row + 1);
    let columns_delta = if self.column == 0 {
      self.column
    } else {
      self.column - 1
    };
    let columns = columns_delta..=(self.column + 1);

    let mut numbers: Vec<GearNumber> = Vec::new();

    rows.for_each(|row| match input.lines().nth(row) {
      Some(line) => line
        .trim()
        .chars()
        .enumerate()
        .for_each(|(char_index, char)| {
          if char.is_numeric() {
            let number = numbers
              .iter_mut()
              .find(|number| number.column.1 + 1 == char_index && number.row == row);
            if let Some(number) = number {
              number.column.1 = char_index;
              number.value.push(char);
            } else {
              numbers.push(GearNumber {
                row,
                column: (char_index, char_index),
                value: char.to_string(),
              });
            }
          }

          self.numbers = numbers.clone();
        }),
      None => {}
    });

    let allowed_numbers: Vec<_> = numbers
      .iter()
      .filter_map(|number| {
        if columns.contains(&number.column.0) || columns.contains(&number.column.1) {
          return Some(number.to_owned());
        }

        None
      })
      .to_owned()
      .collect();

    self.numbers = allowed_numbers.clone()
  }
}

fn tester_part_two(input: &str) -> i32 {
  let mut count: i32 = 0;

  let mut gears: Vec<Gear> = Vec::new();

  for (line_index, line) in input.lines().enumerate() {
    for (char_index, char) in line.chars().enumerate() {
      if char != '*' {
        continue;
      }

      gears.push(Gear {
        row: line_index,
        column: char_index,
        numbers: Vec::new(),
      });

      let gear = gears.last_mut().unwrap();
      gear.check_surrounding(input);
    }
  }

  gears.iter().for_each(|gear| {
    if gear.numbers.len() > 1 {
      let product = gear
        .numbers
        .iter()
        .fold(1, |acc, number| acc * number.value.parse::<i32>().unwrap());
      count += product;
    }
  });

  count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_part_one() {
    let example: &str = include_str!("../assets/example.txt");

    assert_eq!(tester_part_one(example), 4361);
  }

  #[test]
  fn exmaple_parte_two() {
    let example: &str = include_str!("../assets/example.txt");

    assert_eq!(tester_part_two(example), 467835);
  }
}
