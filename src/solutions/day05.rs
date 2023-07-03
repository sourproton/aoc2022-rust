//! Day 5: Supply Stacks
//!
//! solution to the day 05 of AoC2022
//!
//! https://adventofcode.com/2022/day/5

use {
    crate::helpers::{read_lines, Answer},
    std::time::SystemTime,
};

/// solves the part 1 of day 05 and return its result and elapsed time
pub fn pt1(filename: &str) -> Answer {
    let time = SystemTime::now();

    let mut crates = Crates::parse(filename);

    read_lines(filename)
        .skip_while(|line| {
            if let Some(c) = line.chars().next() {
                c != 'm'
            } else {
                true
            }
        })
        .for_each(|line| {
            let instruction = Instruction::from_str(line);
            crates.move_from_top(instruction);
        });

    Answer::new(
        crates.last_state(),
        time.elapsed().unwrap().as_millis() as u32,
    )
}

/// solves the part 2 of day 05 and return its result and elapsed time
pub fn pt2(filename: &str) -> Answer {
    let time = SystemTime::now();

    let mut crates = Crates::parse(filename);

    read_lines(filename)
        .skip_while(|line| {
            if let Some(c) = line.chars().next() {
                c != 'm'
            } else {
                true
            }
        })
        .for_each(|line| {
            let instruction = Instruction::from_str(line);
            crates.move_from_bottom(instruction);
        });

    Answer::new(
        crates.last_state(),
        time.elapsed().unwrap().as_millis() as u32,
    )
}

struct Crates {
    // n_columns: usize,
    columns: Vec<Vec<char>>,
}

struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from_str(s: String) -> Self {
        let split: Vec<&str> = s.split_whitespace().collect();

        Instruction {
            quantity: split[1].parse().unwrap(),
            from: split[3].parse().unwrap(),
            to: split[5].parse().unwrap(),
        }
    }
}

impl Crates {
    fn new_empty(n_columns: usize) -> Self {
        Crates {
            // n_columns,
            columns: vec![Vec::new(); n_columns],
        }
    }

    fn last_state(&self) -> String {
        let mut s = String::new();

        self.columns.iter().for_each(|c| {
            s.push(c.last().unwrap().to_owned());
        });

        s
    }

    fn move_from_top(&mut self, instruction: Instruction) {
        (0..instruction.quantity).for_each(|_| {
            if let Some(v) = self.columns[instruction.from - 1].pop() {
                self.columns[instruction.to - 1].push(v);
            };
        });
    }

    fn move_from_bottom(&mut self, instruction: Instruction) {
        let to_add = self.columns[instruction.from - 1]
            .iter()
            .rev()
            .take(instruction.quantity)
            .map(|c| c.to_owned())
            .collect::<Vec<char>>();

        to_add.iter().rev().for_each(|c| {
            self.columns[instruction.from - 1].pop().unwrap();
            self.columns[instruction.to - 1].push(*c);
        })
    }

    /// Reads the first lines of the puzzle to parse the initial state of the crates
    fn parse(filename: &str) -> Self {
        // first lines of the input file corresponding to the crates
        let mut crate_lines = Vec::new();

        // store the lines to then read in reverse order
        read_lines(filename)
            .take_while(|line| line.chars().nth(1).expect("could not add line to vector") != '1')
            .for_each(|line| {
                crate_lines.push(line);
            });

        // read first line to know how many columns there are and create the vectors
        let n_columns = crate_lines
            .get(0)
            .expect("could not read line length")
            .len()
            / 4
            + 1;

        // create vector of `n_columns` empty vectors, one for each crate column
        let mut crates = Crates::new_empty(n_columns);

        // populate the columns
        crate_lines
            .iter()
            .rev()
            // .map(|line| line.chars().collect::<Vec<char>>())
            .for_each(|line| {
                line.chars()
                    .collect::<Vec<char>>()
                    .iter()
                    .skip(1)
                    .step_by(4)
                    .enumerate()
                    .for_each(|(i, c)| {
                        if *c != ' ' {
                            crates.add_crate(i, *c);
                        }
                    })
            });

        crates
    }

    fn add_crate(&mut self, column_index: usize, value: char) {
        self.columns[column_index].push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::{pt1, pt2, Crates};

    const FILENAME: &str = "./data/examples/example05.txt";

    #[test]
    fn pt01() {
        let answer = pt1(FILENAME);
        assert_eq!(String::from("CMZ"), answer.value());
    }

    #[test]
    fn pt02() {
        let answer = pt2(FILENAME);
        assert_eq!(String::from("MCD"), answer.value());
    }

    #[test]
    fn test_parse_crates() {
        let parsed = Crates::parse(FILENAME).columns;
        assert_eq!(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']], parsed);
    }
}
