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
        let side1 = length * width;
        let side2 = width * height;
        let side3 = height * length;
        let smallest_side = std::cmp::min(side3, std::cmp::min(side1, side2));
        total += (2 * side1) + (2 * side2) + (2 * side3) + smallest_side;
    }
    Ok(format!("{}", total))
}

#[cfg(test)]
mod tests {
    use crate::day2::p1;
    use anyhow::Result;

    #[test]
    fn returns_58() -> Result<()> {
        assert_eq!("58", p1::solve(String::from("2x3x4"))?);
        Ok(())
    }

    #[test]
    fn returns_43() -> Result<()> {
        assert_eq!("43", p1::solve(String::from("1x1x10"))?);
        Ok(())
    }

    #[test]
    fn returns_101() -> Result<()> {
        assert_eq!(
            "101",
            p1::solve(String::from(
                r#"2x3x4
                    1x1x10"#
            ))?
        );
        Ok(())
    }
}
