use lazy_static::lazy_static;
use regex::Regex;
use twentyone::stdin::read_from_stdin;

fn main() {
    let input = read_from_stdin().unwrap();
    let parsed = parse(&input);
    let p1_position = solve_basic(parsed.clone());
    let p1_answer = p1_position.horizontal * p1_position.depth;

    println!("Part 1 --->");
    println!(
        "Horizontal: {}, Depth: {}, Answer: {}",
        p1_position.horizontal, p1_position.depth, p1_answer
    );
    println!();

    let p2_position = solve_with_aim(parsed);
    let p2_answer = p2_position.horizontal * p2_position.depth;

    println!("Part 2 --->");
    println!(
        "Horizontal: {}, Depth: {}, Answer: {}",
        p2_position.horizontal, p2_position.depth, p2_answer
    );
}

struct SubmarinePosition {
    horizontal: i32,
    depth: i32,
}

#[derive(Clone)]
enum Command {
    Forward(u8),
    Up(u8),
    Down(u8),
}

fn parse(input: &str) -> Vec<Command> {
    input.lines().map(line_to_command).collect()
}

fn solve_basic(parsed: Vec<Command>) -> SubmarinePosition {
    let mut position = SubmarinePosition {
        horizontal: 0,
        depth: 0,
    };

    for command in parsed {
        match command {
            Command::Forward(x) => position.horizontal += x as i32,
            Command::Up(x) => position.depth -= x as i32,
            Command::Down(x) => position.depth += x as i32,
        }
    }

    position
}

fn solve_with_aim(parsed: Vec<Command>) -> SubmarinePosition {
    let mut aim = 0;

    let mut position = SubmarinePosition {
        horizontal: 0,
        depth: 0,
    };

    for command in parsed {
        match command {
            Command::Forward(x) => {
                position.horizontal += x as i32;
                position.depth += aim * (x as i32);
            }
            Command::Up(x) => aim -= x as i32,
            Command::Down(x) => aim += x as i32,
        }
    }

    position
}

fn line_to_command(line: &str) -> Command {
    lazy_static! {
        static ref COMMAND_REGEX: Regex =
            Regex::new(r"(?P<cmd>forward|down|up).(?P<amount>\d)").unwrap();
    }

    let cap = COMMAND_REGEX.captures(line).unwrap();
    let amount = cap
        .name("amount")
        .expect("Amount not found on line")
        .as_str()
        .parse::<u8>()
        .expect("Failed to parse amount");

    match cap
        .name("cmd")
        .expect("Command not matched on line")
        .as_str()
    {
        "forward" => Command::Forward(amount),
        "up" => Command::Up(amount),
        "down" => Command::Down(amount),
        _ => panic!("Unexpected command"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "};

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day2.txt");

    #[test]
    fn calculates_first_example() {
        let parsed = parse(EXAMPLE);
        let position = solve_basic(parsed);
        let answer = position.horizontal * position.depth;

        assert_eq!(answer, 150);
    }

    #[test]
    fn calculates_part_1() {
        let parsed = parse(PUZZLE_INPUT);
        let position = solve_basic(parsed);
        let answer = position.horizontal * position.depth;

        assert_eq!(answer, 1670340);
    }

    #[test]
    fn calculates_second_example() {
        let parsed = parse(EXAMPLE);
        let position = solve_with_aim(parsed);
        let answer = position.horizontal * position.depth;

        assert_eq!(answer, 900);
    }

    #[test]
    fn calculates_part_2() {
        let parsed = parse(PUZZLE_INPUT);
        let position = solve_with_aim(parsed);
        let answer = position.horizontal * position.depth;

        assert_eq!(answer, 1954293920);
    }
}
