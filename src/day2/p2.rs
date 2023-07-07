use anyhow::Result;
use text_io::scan;

pub fn solve(input: String) -> Result<String> {
    let mut total = 0;
    for line in input.lines() {
        let trimmed_line = line.trim();
        let length: usize;
        let width: usize;
        let height: usize;
        scan!(trimmed_line.bytes() => "{}x{}x{}", length, width, height);
        let bow_length = length * width * height;
        let mut sides = vec![length, width, height];
        sides.sort();
        let ribbon_length = (sides[0] * 2) + (sides[1] * 2);
        total += ribbon_length + bow_length;
    }
    Ok(format!("{}", total))
}

#[cfg(test)]
mod tests {
    use crate::day2::p2;
    use anyhow::Result;

    #[test]
    fn returns_34() -> Result<()> {
        assert_eq!("34", p2::solve(String::from("2x3x4"))?);
        Ok(())
    }

    #[test]
    fn returns_14() -> Result<()> {
        assert_eq!("14", p2::solve(String::from("1x1x10"))?);
        Ok(())
    }

    #[test]
    fn returns_101() -> Result<()> {
        assert_eq!(
            "48",
            p2::solve(String::from(
                r#"2x3x4
                    1x1x10"#
            ))?
        );
        Ok(())
    }
}
