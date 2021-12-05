//! --- Day 4: Giant Squid ---

use twentyone::stdin::read_from_stdin;
use twentyone::util::transpose_vec;

fn main() {
    let input = read_from_stdin().unwrap();
    let (game, all_draws) = parse(&input);

    let p1_answer = solve_first_winning(game.clone(), all_draws.clone());
    println!("Part 1: {}", p1_answer);

    let p2_answer = solve_last_winning(game, all_draws);
    println!("Part 2: {}", p2_answer);
}

#[derive(Clone, Debug)]
struct Game {
    boards: Vec<Board>,
    draws: Vec<u8>,
}

impl Game {
    /// Add a number to the drawed numbers in the game
    fn draw(&mut self, number: u8) {
        self.draws.push(number);
    }

    /// Pop the winning boards from the game based on the drawed numbers.
    /// This removes the winners from the game. (Note: There might be more than one winner!)
    fn pop_winners(&mut self) -> Option<Vec<Board>> {
        let (winners, remaining): (Vec<Board>, Vec<Board>) =
            self.boards.iter().cloned().partition(|board| {
                board.has_winning_row(&self.draws) || board.has_winning_col(&self.draws)
            });

        self.boards = remaining;

        if winners.is_empty() {
            None
        } else {
            Some(winners)
        }
    }
}

#[derive(Clone, Debug)]
struct Board(Vec<Vec<u8>>);

impl Board {
    /// Does the board have a winning column? Shameless lazy trick: Just transpose the 5x5 board
    fn has_winning_col(&self, draws: &[u8]) -> bool {
        let transposed = transpose_vec(self.0.clone());

        transposed
            .iter()
            .any(|row| row.iter().all(|number| draws.contains(number)))
    }

    /// Does the board have a winning row?
    fn has_winning_row(&self, draws: &[u8]) -> bool {
        self.0
            .iter()
            .any(|row| row.iter().all(|number| draws.contains(number)))
    }

    /// Calculate the answer for a winning board: Remove all drawn numbers and sum the rest.
    /// Then multiply by the last drawn number.
    fn calculate_answer(&self, draws: &[u8]) -> u32 {
        let sum: u32 = self
            .all_numbers()
            .iter()
            .filter(|number| !draws.contains(number))
            .map(|&x| x as u32)
            .sum();

        let last_draw = *draws.last().unwrap() as u32;

        sum * last_draw
    }

    /// Return all numbers on the board.
    fn all_numbers(&self) -> Vec<u8> {
        Vec::new()
            .into_iter()
            .chain(self.0[0].clone().into_iter())
            .chain(self.0[1].clone().into_iter())
            .chain(self.0[2].clone().into_iter())
            .chain(self.0[3].clone().into_iter())
            .chain(self.0[4].clone().into_iter())
            .collect()
    }
}

/// Create the board from 5 input lines
impl From<&[&str]> for Board {
    fn from(input_lines: &[&str]) -> Self {
        assert_eq!(input_lines.len(), 5);

        let inner = input_lines
            .iter()
            .map(|line| {
                let line = line
                    .split_whitespace()
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>();

                assert_eq!(line.len(), 5);
                line
            })
            .collect::<Vec<Vec<u8>>>();

        Board(inner)
    }
}

fn parse(input: &str) -> (Game, Vec<u8>) {
    // Parse first line as draws
    let all_draws = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    // Create a Vec<&str> from all other lines, skipping empty
    let lines = input
        .lines()
        .skip(2)
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    // Create a board from chunks of 5 lines
    let boards = lines[..].chunks(5).map(Board::from).collect::<Vec<Board>>();

    let game = Game {
        boards,
        draws: Vec::new(),
    };

    (game, all_draws)
}

// Solves part 1: Find first winning board
fn solve_first_winning(mut game: Game, all_draws: Vec<u8>) -> u32 {
    let mut i = 0;

    let winning_boards = loop {
        let number = all_draws[i];
        game.draw(number);

        if let Some(board) = game.pop_winners() {
            break board;
        }

        i += 1;
    };

    match &winning_boards[..] {
        [board] => board.calculate_answer(&game.draws),
        _ => panic!("More than one board won first"),
    }
}

/// Solves part 2: Find last winning board. This one looked easy, but got me into a lot of trouble,
/// especially because multiple boards can win at the same time!
fn solve_last_winning(mut game: Game, all_draws: Vec<u8>) -> u32 {
    let mut results: Vec<(Vec<Board>, Vec<u8>)> = Vec::new();

    for draw in all_draws.clone() {
        game.draw(draw);

        if let Some(boards) = game.pop_winners() {
            results.push((boards, game.draws.clone()));
        }
    }

    let (last_winners, last_winner_draws) = results.last().unwrap();

    match &last_winners[..] {
        [board] => board.calculate_answer(last_winner_draws),
        _ => panic!("More than one board won first"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_DATA: &str = indoc! {"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
        
        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
    "};

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day4.txt");

    #[test]
    fn d4_part1_example() {
        let (game, all_draws) = parse(EXAMPLE_DATA);
        let answer = solve_first_winning(game, all_draws);

        assert_eq!(answer, 4512);
    }

    #[test]
    fn d4_part1_answer() {
        let (game, all_draws) = parse(PUZZLE_INPUT);
        let answer = solve_first_winning(game, all_draws);

        assert_eq!(answer, 89001);
    }

    #[test]
    fn d4_part2_example() {
        let (game, all_draws) = parse(EXAMPLE_DATA);
        let answer = solve_last_winning(game, all_draws);

        assert_eq!(answer, 1924);
    }

    #[test]
    fn d4_part2_answer() {
        let (game, all_draws) = parse(PUZZLE_INPUT);
        let answer = solve_last_winning(game, all_draws);

        assert_eq!(answer, 7296);
    }
}
