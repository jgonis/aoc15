use anyhow::Result;
use std::collections::HashMap;

enum ActionType {
    TurnOn,
    TurnOff,
    Toggle,
}

pub fn solve(input: String) -> Result<String> {
    let mut light_grid: HashMap<(usize, usize), bool> = HashMap::new();
    for line in input.lines() {
        let (start_x, start_y, end_x, end_y, action) = parse_line(line);
        for x in start_x..=end_x {
            for y in start_y..=end_y {
                match action {
                    ActionType::TurnOn => {
                        light_grid.insert((x, y), true);
                    }
                    ActionType::TurnOff => {
                        light_grid.insert((x, y), false);
                    }
                    ActionType::Toggle => {
                        let light = light_grid.(&(x, y))
                        light_grid.insert((x, y), !*light);
                    }
                }
            }
        }
    }
    Ok(format!("{}", light_grid.values().filter(|&v| *v).count()))
}

fn parse_line(line: &str) -> (usize, usize, usize, usize, ActionType) {
    let parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    let mut action_type = ActionType::Toggle;
    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;

    if parts[0] == "turn" {
        if parts[1] == "on" {
            action_type = ActionType::TurnOn;
        } else if parts[1] == "off" {
            action_type = ActionType::TurnOff
        }
        let start_coords: Vec<String> = parts[2].split(",").map(|s| s.to_string()).collect();
        start_x = start_coords[0].parse::<usize>().unwrap();
        start_y = start_coords[1].parse::<usize>().unwrap();
        let end_coords: Vec<String> = parts[4].split(",").map(|s| s.to_string()).collect();
        end_x = end_coords[0].parse::<usize>().unwrap();
        end_y = end_coords[1].parse::<usize>().unwrap();
    } else {
        action_type = ActionType::Toggle;
        let start_coords: Vec<String> = parts[1].split(",").map(|s| s.to_string()).collect();
        start_x = start_coords[0].parse::<usize>().unwrap();
        start_y = start_coords[1].parse::<usize>().unwrap();
        let end_coords: Vec<String> = parts[3].split(",").map(|s| s.to_string()).collect();
        end_x = end_coords[0].parse::<usize>().unwrap();
        end_y = end_coords[1].parse::<usize>().unwrap();
    }

    (start_x, start_y, end_x, end_y, action_type)
}

#[cfg(test)]
mod tests {
    use crate::day6::p1;
    use anyhow::Result;

    #[test]
    fn nice_strings() -> Result<()> {
        assert!(p1::is_string_nice("turn on 0,0 through 999,999"));
        Ok(())
    }

    #[test]
    fn naughty_strings() -> Result<()> {
        assert!(p1::is_string_nice("toggle 0,0 through 999,0"));
        Ok(())
    }

    #[test]
    fn naughty_strings() -> Result<()> {
        assert!(!p1::is_string_nice("turn off 499,499 through 500,500"));
        Ok(())
    }
}
