use anyhow::{anyhow, Result};

pub fn solve(input: String) -> Result<String> {
    let mut count: usize = 1;
    let mut current_floor = 0;
    println!("In problem 2 solver!");
    for character in input.chars() {
        match character {
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => Err(anyhow!("Unrecognized character"))?,
        };
        if current_floor == -1 {
            return Ok(format!("{}", count));
        }
        count += 1;
    }
    Ok(format!("{}", count))
}

#[cfg(test)]
mod tests {
    use crate::day1::p2;
    use anyhow::Result;

    #[test]
    fn returns_1() -> Result<()> {
        assert_eq!("1", p2::solve(String::from(")"))?);
        Ok(())
    }

    #[test]
    fn returns_5() -> Result<()> {
        assert_eq!("5", p2::solve(String::from("()())"))?);
        Ok(())
    }
}
