use anyhow::Result;

pub fn solve(input: String) -> Result<String> {
    Ok(format!("Jeff"))
}

#[cfg(test)]
mod tests {
    use crate::day7::p2;
    use anyhow::Result;

    #[test]
    fn stub_test() -> Result<()> {
        assert_eq!(
            1_000_000.to_string(),
            p2::solve(String::from("turn on 0,0 through 999,999"))?
        );
        Ok(())
    }
}
