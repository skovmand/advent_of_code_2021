//! --- Day 1: Sonar Sweep ---

use twentyone::stdin::read_from_stdin;

fn main() {
    let input = read_from_stdin().unwrap();
    let parsed = parse(&input).unwrap();
    let result_1 = count_increments(parsed.clone());
    let result_2 = count_increments_3(parsed);

    println!("Part 1: {}", result_1);
    println!("Part 2: {}", result_2);
}

fn parse(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    input
        .lines()
        .map(|x| x.trim())
        .map(|x| x.parse::<i32>())
        .collect()
}

// Solves part 1
fn count_increments(numbers: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut prev_number = numbers[0];

    for number in numbers.into_iter().skip(1) {
        if number > prev_number {
            count += 1
        }

        prev_number = number;
    }

    count
}

// Solves part 2
fn count_increments_3(numbers: Vec<i32>) -> i32 {
    let sums = group_to_sums_of_3(numbers);
    count_increments(sums)
}

// Slide a size-3 window over the numbers and sum them, e.g. [1,2,3,4,5,6] -> [6,9,12,15]
fn group_to_sums_of_3(numbers: Vec<i32>) -> Vec<i32> {
    let mut prev_2 = numbers[0];
    let mut prev_1 = numbers[1];
    let mut sums = Vec::new();

    for number in numbers.into_iter().skip(2) {
        sums.push(prev_2 + prev_1 + number);

        prev_2 = prev_1;
        prev_1 = number;
    }

    sums
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_DATA: &str = indoc! {"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "};

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day1.txt");

    #[test]
    fn solves_first_example() {
        let parsed = parse(EXAMPLE_DATA).unwrap();
        let solution = count_increments(parsed);

        assert_eq!(solution, 7);
    }

    #[test]
    fn solves_d1p1() {
        let parsed = parse(PUZZLE_INPUT).unwrap();
        let solution = count_increments(parsed);

        assert_eq!(solution, 1715);
    }

    #[test]
    fn solves_second_example() {
        let parsed = parse(EXAMPLE_DATA).unwrap();
        let solution = count_increments_3(parsed);

        assert_eq!(solution, 5);
    }

    #[test]
    fn solves_d1p2() {
        let parsed = parse(PUZZLE_INPUT).unwrap();
        let solution = count_increments_3(parsed);

        assert_eq!(solution, 1739);
    }
}
