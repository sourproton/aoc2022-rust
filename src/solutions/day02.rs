//! Day 2: Rock Paper Scissors
//!
//! solution to the day 02 of AoC2022
//!
//! https://adventofcode.com/2022/day/2

use {
    crate::helpers::{read_lines, Answer},
    std::time::SystemTime,
};

/// solves the part 1 of day 02 and return its result and elapsed time
pub fn pt1(filename: &str) -> Answer {
    let time = SystemTime::now();

    let answer = read_lines(filename)
        .map(|line| RoundMoves::from_line(line).total_points())
        .sum();

    Answer::new(answer, time.elapsed().unwrap().as_millis() as u32)
}

/// solves the part 2 of day 02 and return its result and elapsed time
pub fn pt2(filename: &str) -> Answer {
    let time = SystemTime::now();

    let answer = read_lines(filename)
        .map(|line| RoundOutcome::from_line(line).total_points())
        .sum();

    Answer::new(answer, time.elapsed().unwrap().as_millis() as u32)
}

#[derive(Debug)]
/// represents a round of the game, with a move from the opponent and a move from the player
struct RoundMoves {
    opponent: Move,
    player: Move,
}

impl RoundMoves {
    /// based on the moves and the winner, calculates the points of the round
    fn total_points(&self) -> u32 {
        self.move_points() + self.outcome_points()
    }

    /// calculates the points related to the move of the player
    fn move_points(&self) -> u32 {
        match self.player {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    /// calculates the points related to the result of the round
    fn outcome_points(&self) -> u32 {
        match self.opponent {
            Move::Rock => match self.player {
                Move::Rock => 3,
                Move::Paper => 6,
                Move::Scissors => 0,
            },
            Move::Paper => match self.player {
                Move::Rock => 0,
                Move::Paper => 3,
                Move::Scissors => 6,
            },
            Move::Scissors => match self.player {
                Move::Rock => 6,
                Move::Paper => 0,
                Move::Scissors => 3,
            },
        }
    }

    /// according to part 1 instructions, parses a `line` in the file to its equivalent `RoundMoves`
    fn from_line(line: String) -> Self {
        // each line should be two letters separated by a space
        let moves: Vec<&str> = line.split(' ').collect();

        if moves.len() != 2 {
            panic!("line contains more than one move");
        }

        let opponent = match moves[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("move {} not allowed", moves[0]),
        };

        let player = match moves[1] {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("move {} not allowed", moves[1]),
        };

        RoundMoves::new(opponent, player)
    }

    /// returns a new instance of `RoundMoves`
    fn new(opponent: Move, player: Move) -> Self {
        RoundMoves { opponent, player }
    }
}

#[derive(Debug)]
/// represents a round of the game, with a move from the opponent and the outcome of the game
struct RoundOutcome {
    opponent: Move,
    outcome: Outcome,
}

impl RoundOutcome {
    /// based on the moves and the winner, calculates the points of the round
    fn total_points(&self) -> u32 {
        self.move_points() + self.outcome_points()
    }

    /// calculates the points related to the move of the player
    fn move_points(&self) -> u32 {
        match self.opponent {
            Move::Rock => match self.outcome {
                Outcome::Lost => 3, // played scissors
                Outcome::Draw => 1, // played rock
                Outcome::Win => 2,  // played paper
            },
            Move::Paper => match self.outcome {
                Outcome::Lost => 1, // played rock
                Outcome::Draw => 2, // played paper
                Outcome::Win => 3,  // played scissors
            },
            Move::Scissors => match self.outcome {
                Outcome::Lost => 2, // played paper
                Outcome::Draw => 3, // played scissors
                Outcome::Win => 1,  // played rock
            },
        }
    }

    /// calculates the points related to the result of the round
    fn outcome_points(&self) -> u32 {
        match self.outcome {
            Outcome::Lost => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    /// according to part 1 instructions, parses a `line` in the file to its equivalent `RoundMoves`
    fn from_line(line: String) -> Self {
        // each line should be two letters separated by a space
        let moves: Vec<&str> = line.split(' ').collect();

        if moves.len() != 2 {
            panic!("line contains more than one move");
        }

        let opponent = match moves[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("move {} not allowed", moves[0]),
        };

        let outcome = match moves[1] {
            "X" => Outcome::Lost,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("move {} not allowed", moves[1]),
        };

        RoundOutcome::new(opponent, outcome)
    }

    /// returns a new instance of `RoundOutcome`
    fn new(opponent: Move, outcome: Outcome) -> Self {
        RoundOutcome { opponent, outcome }
    }
}

#[derive(Debug)]
/// represents the possible moves in the game
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
/// represents the possible outcomes of the game
enum Outcome {
    Lost,
    Draw,
    Win,
}

#[cfg(test)]
mod tests {
    use super::{pt1, pt2};

    const FILENAME: &str = "./data/examples/example02.txt";

    #[test]
    fn pt01() {
        let answer = pt1(FILENAME);
        assert_eq!(15, answer.answer());
    }

    #[test]
    fn pt02() {
        let answer = pt2(FILENAME);
        assert_eq!(12, answer.answer());
    }
}
