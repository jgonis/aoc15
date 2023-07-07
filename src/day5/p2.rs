use anyhow::Result;
use std::collections::HashMap;

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
    if !contains_repeat_with_break(input) {
        return false;
    }
    if !contains_two_nonoverlapping_repeats(input) {
        return false;
    }
    true
}

fn contains_two_nonoverlapping_repeats(input: &str) -> bool {
    let pairs: Vec<String> = input
        .chars()
        .zip(input.chars().skip(1))
        .map(|pair| format!("{}{}", pair.0, pair.1))
        .collect();
    let pair_match_locations: HashMap<String, Vec<usize>> = HashMap::new();
    let candidate_matches: Vec<_> = pairs
        .iter()
        .map(|pair| {
            let v: Vec<_> = input.match_indices(pair).collect();
            v
        })
        .filter(|matches| matches.len() >= 2)
        .collect();
    candidate_matches.iter().any(|matches| {
        matches
            .iter()
            .zip(matches.iter().skip(1))
            .any(|match_pair| match_pair.1 .0 - match_pair.0 .0 >= 2)
    })
}

fn contains_repeat_with_break(input: &str) -> bool {
    input
        .chars()
        .zip(input.chars().skip(1).zip(input.chars().skip(2)))
        .map(|x| (x.0, x.1 .0, x.1 .1))
        .any(|triple| triple.0 == triple.2)
}

#[cfg(test)]
mod tests {
    use crate::day5::p2;
    use anyhow::Result;

    #[test]
    fn nice_strings() -> Result<()> {
        assert!(p2::is_string_nice("qjhvhtzxzqqjkmpb"));
        assert!(p2::is_string_nice("xxyxx"));
        Ok(())
    }

    #[test]
    fn naughty_strings() -> Result<()> {
        assert!(!p2::is_string_nice("uurcxstgmygtbstg"));
        assert!(!p2::is_string_nice("ieodomkazucvgmuy"));
        Ok(())
    }
}
