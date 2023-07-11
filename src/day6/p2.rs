use crate::utils::action_type::ActionType;
use anyhow::Result;
use std::collections::HashMap;

pub fn solve(input: String) -> Result<String> {
    let mut light_grid: HashMap<(usize, usize), usize> = HashMap::new();
    for line in input.lines() {
        let (start_x, start_y, end_x, end_y, action) = parse_line(line);
        for x in start_x..=end_x {
            for y in start_y..=end_y {
                match action {
                    ActionType::TurnOn => {
                        if let Some(x) = light_grid.get_mut(&(x, y)) {
                            *x += 1;
                        } else {
                            light_grid.insert((x, y), 1);
                        }
                    }
                    ActionType::TurnOff => {
                        if let Some(x) = light_grid.get_mut(&(x, y)) {
                            if *x > 0 {
                                *x -= 1;
                            }
                        }
                    }
                    ActionType::Toggle => {
                        if let Some(x) = light_grid.get_mut(&(x, y)) {
                            *x += 2;
                        } else {
                            light_grid.insert((x, y), 2);
                        }
                    }
                }
            }
        }
    }
    Ok(format!("{}", light_grid.values().sum::<usize>()))
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
    use crate::day6::p2;
    use anyhow::Result;

    #[test]
    fn returns_1() -> Result<()> {
        assert_eq!(
            1.to_string(),
            p2::solve(String::from("turn on 0,0 through 0,0"))?
        );
        Ok(())
    }

    #[test]
    fn returns_2_000_000() -> Result<()> {
        assert_eq!(
            2_000_000.to_string(),
            p2::solve(String::from("toggle 0,0 through 999,999"))?
        );
        Ok(())
    }
}
