//! --- Day 3: Binary Diagnostic ---

use std::cmp::Ordering;
use twentyone::{stdin::read_from_stdin, util::transpose_vec};

fn main() {
    let input = read_from_stdin().unwrap();
    let parsed = parse(&input);
    let report = solve(parsed);

    println!("Part 1: Power consumption is {}", report.power_consumption);
    println!(
        "Part 2: Life support rating is {}",
        report.life_support_rating
    );
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(line_to_vec).collect()
}

fn line_to_vec(line: &str) -> Vec<u8> {
    line.trim()
        .chars()
        .map(|c| match c {
            '0' => 0,
            '1' => 1,
            c => panic!("Unknown character: '{}'", c),
        })
        .collect()
}

#[allow(unused)]
struct DiagnosticReportResult {
    epsilon_rate: u32,
    gamma_rate: u32,
    power_consumption: u32,
    oxygen_generator_rating: u32,
    co2_scrubber_rating: u32,
    life_support_rating: u32,
}

fn solve(diagnostics: Vec<Vec<u8>>) -> DiagnosticReportResult {
    let transposed = transpose_vec(diagnostics.clone());
    let (gamma_rate, epsilon_rate) = calculate_basic_rates(transposed);

    let oxygen_generator_rating = filter_binaries(diagnostics.clone(), find_most_common_digit);
    let co2_scrubber_rating = filter_binaries(diagnostics, find_least_common_digit);

    DiagnosticReportResult {
        gamma_rate,
        epsilon_rate,
        power_consumption: gamma_rate * epsilon_rate,
        oxygen_generator_rating,
        co2_scrubber_rating,
        life_support_rating: oxygen_generator_rating * co2_scrubber_rating,
    }
}

// Calculates the metrics for P1
fn calculate_basic_rates(transposed_diagnostics: Vec<Vec<u8>>) -> (u32, u32) {
    let row_length = transposed_diagnostics[0].len();

    // Summarize each row into a single 1 or 0 bit, collect as String
    // (Remember: We are now using rows instead of columns, since the vec is transposed)
    let gamma_rate_bits: Vec<bool> = transposed_diagnostics
        .into_iter()
        .map(|row| row.into_iter().filter(|&c| c == 1).count())
        .map(|count| count > (row_length / 2))
        .collect();

    let gamma_rate = bitvec_to_u32(gamma_rate_bits.clone());

    let epsilon_rate_bits: Vec<bool> = gamma_rate_bits.iter().map(|x| !x).collect();
    let epsilon_rate = bitvec_to_u32(epsilon_rate_bits);

    (gamma_rate, epsilon_rate)
}

// Generalized function for calculting metrics in P2.
// Takes a function to select the digit to keep during filtering.
fn filter_binaries<F>(diagnostics: Vec<Vec<u8>>, digit_selection_fn: F) -> u32
where
    F: Fn(Vec<Vec<u8>>, usize) -> u8,
{
    let row_length = diagnostics[0].len();

    let remaining_binary_list = (0..row_length).fold(diagnostics, |acc, column| {
        // If length is 1, return acc for the remaining of the iteration (kind of equivalent to stopping :-)
        // Otherwise, the list will be emptied
        if acc.len() == 1 {
            return acc;
        }

        let selected_digit = digit_selection_fn(acc.clone(), column);

        acc.into_iter()
            .filter(|row| row[column] == selected_digit)
            .collect()
    });

    match &remaining_binary_list[..] {
        [remaining_binary] => {
            let with_bools = remaining_binary.iter().map(|&x| x == 1).collect();
            bitvec_to_u32(with_bools)
        }
        _ => panic!("Unexpected remaining binary count, expected exactly one binary"),
    }
}

fn find_least_common_digit(binaries: Vec<Vec<u8>>, column: usize) -> u8 {
    match zero_count(binaries, column) {
        Ordering::Greater => 1,
        Ordering::Less => 0,
        Ordering::Equal => 0,
    }
}

fn find_most_common_digit(binaries: Vec<Vec<u8>>, column: usize) -> u8 {
    match zero_count(binaries, column) {
        Ordering::Greater => 0,
        Ordering::Less => 1,
        Ordering::Equal => 1,
    }
}

fn zero_count(binaries: Vec<Vec<u8>>, column: usize) -> Ordering {
    let total_rows = binaries.len();
    let zeroes = binaries
        .iter()
        .map(|row| row[column])
        .filter(|&x| x == 0)
        .count();

    zeroes.cmp(&(total_rows / 2))
}

// Convert a Vec<bool> to u32 by parsing it as radix 2
fn bitvec_to_u32(bool_vec: Vec<bool>) -> u32 {
    let gamma_rate_string: String = bool_vec
        .into_iter()
        .map(|b| if b { '1' } else { '0' })
        .collect();

    usize::from_str_radix(&gamma_rate_string, 2).unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010 
    "};

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day3.txt");

    #[test]
    fn d3_part1_example() {
        let parsed = parse(EXAMPLE);
        let report = solve(parsed);

        assert_eq!(report.gamma_rate, 0b10110);
        assert_eq!(report.epsilon_rate, 0b01001);
        assert_eq!(report.power_consumption, 198);
    }

    #[test]
    fn d3_part1_answer() {
        let parsed = parse(PUZZLE_INPUT);
        let report = solve(parsed);

        assert_eq!(report.power_consumption, 2743844);
    }

    #[test]
    fn d3_part2_example() {
        let parsed = parse(EXAMPLE);
        let report = solve(parsed);

        assert_eq!(report.oxygen_generator_rating, 0b10111);
        assert_eq!(report.co2_scrubber_rating, 0b01010);
        assert_eq!(report.life_support_rating, 230);
    }

    #[test]
    fn d3_part2_answer() {
        let parsed = parse(PUZZLE_INPUT);
        let report = solve(parsed);

        assert_eq!(report.life_support_rating, 6677951);
    }
}
