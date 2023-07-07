use crate::utils::point::Point;
use anyhow::{anyhow, Result};
use std::collections::HashSet;

pub fn solve(input: String) -> Result<String> {
    let mut current_position = Point::new(0, 0);
    let mut visited_positions: HashSet<Point> = HashSet::new();
    visited_positions.insert(current_position.clone());
    for char in input.chars() {
        match char {
            '^' => {
                current_position = current_position.up(1);
            }
            'v' => {
                current_position = current_position.down(1);
            }
            '>' => {
                current_position = current_position.east(1);
            }
            '<' => {
                current_position = current_position.west(1);
            }
            _ => Err(anyhow!("unrecognized character"))?,
        };
        visited_positions.insert(current_position.clone());
    }
    Ok(visited_positions.len().to_string())
}

#[cfg(test)]
mod tests {
    use crate::day3::p1;
    use anyhow::Result;

    #[test]
    fn returns_2() -> Result<()> {
        assert_eq!("2", p1::solve(String::from(">"))?);
        assert_eq!("2", p1::solve(String::from("^v^v^v^v^v"))?);
        Ok(())
    }

    #[test]
    fn returns_4() -> Result<()> {
        assert_eq!("4", p1::solve(String::from("^>v<"))?);
        Ok(())
    }
}
