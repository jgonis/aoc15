use anyhow::Result;

pub fn solve(input: String) -> Result<String> {
    Ok(String::from("jeff"))
}

#[cfg(test)]
mod tests {
    use crate::day6::p2;
    use anyhow::Result;

    #[test]
    fn nice_strings() -> Result<()> {
        assert!(p2::is_string_nice("ugknbfddgicrmopn"));
        assert!(p2::is_string_nice("aaa"));
        Ok(())
    }

    #[test]
    fn naughty_strings() -> Result<()> {
        assert!(!p2::is_string_nice("jchzalrnumimnmhp"));
        assert!(!p2::is_string_nice("haegwjzuvuyypxyu"));
        assert!(!p2::is_string_nice("dvszwmarrgswjxmb"));
        Ok(())
    }
}
