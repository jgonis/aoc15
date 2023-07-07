use anyhow::{anyhow, Result};

pub fn solve(input: String) -> Result<String> {
    let mut count: i128 = 0;
    for character in input.chars() {
        match character {
            '(' => count += 1,
            ')' => count -= 1,
            _ => Err(anyhow!("Unrecognized character"))?,
        };
    }
    Ok(format!("{}", count))
}

#[cfg(test)]
mod tests {
    use crate::day1::p1;
    use anyhow::Result;

    #[test]
    fn returns_0() -> Result<()> {
        assert_eq!("0", p1::solve(String::from("(())"))?);
        assert_eq!("0", p1::solve(String::from("()()"))?);
        Ok(())
    }

    #[test]
    fn returns_3() -> Result<()> {
        assert_eq!("3", p1::solve(String::from("((("))?);
        assert_eq!("3", p1::solve(String::from("(()(()("))?);
        assert_eq!("3", p1::solve(String::from("))((((("))?);
        Ok(())
    }

    #[test]
    fn returns_negative_1() -> Result<()> {
        assert_eq!("-1", p1::solve(String::from("())"))?);
        assert_eq!("-1", p1::solve(String::from("))("))?);
        Ok(())
    }

    #[test]
    fn returns_negative_3() -> Result<()> {
        assert_eq!("-3", p1::solve(String::from(")))"))?);
        assert_eq!("-3", p1::solve(String::from(")())())"))?);
        Ok(())
    }
}
