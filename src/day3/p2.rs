use crate::utils::point::Point;
use anyhow::{anyhow, Result};
use std::collections::HashSet;

pub fn solve(input: String) -> Result<String> {
    let mut santa_position = Point::new(0, 0);
    let mut robot_position = Point::new(0, 0);
    let mut visited_positions: HashSet<Point> = HashSet::new();
    visited_positions.insert(santa_position.clone());
    visited_positions.insert(robot_position.clone());

    for (count, char) in input.chars().enumerate() {
        let current_mover = if count % 2 == 0 {
            &mut santa_position
        } else {
            &mut robot_position
        };
        match char {
            '^' => {
                *current_mover = current_mover.up(1);
            }

            'v' => {
                *current_mover = current_mover.down(1);
            }
            '>' => {
                *current_mover = current_mover.east(1);
            }
            '<' => {
                *current_mover = current_mover.west(1);
            }
            _ => Err(anyhow!("unrecognized character"))?,
        }
        visited_positions.insert(current_mover.clone());
    }
    Ok(visited_positions.len().to_string())
}

#[cfg(test)]
mod tests {
    use crate::day3::p2;
    use anyhow::Result;

    #[test]
    fn returns_3() -> Result<()> {
        assert_eq!("3", p2::solve(String::from("^v"))?);
        assert_eq!("3", p2::solve(String::from("^>v<"))?);
        Ok(())
    }

    #[test]
    fn returns_11() -> Result<()> {
        assert_eq!("11", p2::solve(String::from("^v^v^v^v^v"))?);
        Ok(())
    }
}
