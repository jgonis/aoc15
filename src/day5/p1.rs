use anyhow::Result;

const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];
const FORBIDDEN_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

pub fn solve(input: String) -> Result<String> {
    let mut nice_count = 0;
    for line in input.lines() {
        if is_string_nice(line) {
            nice_count += 1;
        }
    }
    Ok(format!("{}", nice_count))
}

fn is_string_nice(input: &str) -> bool {
    if FORBIDDEN_STRINGS
        .iter()
        .any(|forbidden_string| input.contains(forbidden_string))
    {
        return false;
    }

    if VOWELS
        .iter()
        .map(|vowel| input.matches(vowel).count())
        .sum::<usize>()
        < 3
    {
        return false;
    }
    if !input
        .chars()
        .zip(input.chars().skip(1))
        .any(|(a, b)| a == b)
    {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::day5::p1;
    use anyhow::Result;

    #[test]
    fn nice_strings() -> Result<()> {
        assert!(p1::is_string_nice("ugknbfddgicrmopn"));
        assert!(p1::is_string_nice("aaa"));
        Ok(())
    }

    #[test]
    fn naughty_strings() -> Result<()> {
        assert!(!p1::is_string_nice("jchzalrnumimnmhp"));
        assert!(!p1::is_string_nice("haegwjzuvuyypxyu"));
        assert!(!p1::is_string_nice("dvszwmarrgswjxmb"));
        Ok(())
    }
}
