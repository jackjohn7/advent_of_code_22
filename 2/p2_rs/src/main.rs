#[derive(Copy, Clone)]
enum Choice {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Choice {
    pub fn parse(raw: &str) -> Result<Self, &str> {
        match raw {
            "A" | "X" => Ok(Choice::ROCK),
            "B" | "Y" => Ok(Choice::PAPER),
            "C" | "Z" => Ok(Choice::SCISSORS),
            _ => Err("Invalid choice provided")
        }
    }
    pub fn val(&self) -> i64 {
        match &self {
            Choice::ROCK => 1,
            Choice::PAPER => 2,
            Choice::SCISSORS => 3,
        }
    }
    pub fn play(&self, other: &Self) -> Result<i64, &str> {
        match &self.val() - other.val() {
            -1 | 2 => Ok(0),
            0 => Ok(3),
            -2 | 1 => Ok(6),
            _ => Err("Failed to play")
        }
    }
}

enum Instruction {
    TIE,
    WIN,
    LOSE,
}

impl Instruction {
    pub fn parse(raw: &str) -> Result<Self, &str> {
        match raw {
            "X" => Ok(Instruction::LOSE),
            "Y" => Ok(Instruction::TIE),
            "Z" => Ok(Instruction::WIN),
            _ => Err("Invalid instruction")
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let result = input
        .lines()
        .map(|line| line.split(" "))
        .map(|[raw_opp, raw_res]| (Choice::parse(raw_opp)))
        .fold(0, |acc, (opp, me)| acc + me.play(&opp).unwrap() + me.val());
    println!("Score: {}", result);
}
