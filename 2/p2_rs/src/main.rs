enum Choice {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Choice {
    pub fn parse(raw: &str) -> Result<Self, &str> {
        match raw {
            "A" => Ok(Choice::ROCK),
            "B" => Ok(Choice::PAPER),
            "C" => Ok(Choice::SCISSORS),
            _ => Err("Invalid choice provided")
        }
    }
    pub fn from_val(val: i64) -> Option<Choice> {
        match val {
            1 => Some(Choice::ROCK),
            2 => Some(Choice::PAPER),
            3 => Some(Choice::SCISSORS),
            _ => None
        }
    }
    pub fn val(&self) -> i64 {
        match &self {
            Choice::ROCK => 1,
            Choice::PAPER => 2,
            Choice::SCISSORS => 3,
        }
    }
    pub fn appropriate_move(&self, instr: &Instruction) -> Self {
        Choice::from_val(match self.val() + instr.adjustment() {
            4 => 1,
            0 => 3,
            x => x,
        }).unwrap()
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
    pub fn adjustment(&self) -> i64 {
        match &self {
            Instruction::LOSE => -1,
            Instruction::TIE => 0,
            Instruction::WIN => 1,
        }
    }
    pub fn val(&self) -> i64 {
        match &self {
            Instruction::LOSE => 0,
            Instruction::TIE => 3,
            Instruction::WIN => 6,
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let result = input
        .lines()
        .map(|line| {let splits: Vec<&str> = line.split(" ").collect(); (splits[0], splits[1])})
        .map(|(raw_opp, raw_res)| (Choice::parse(raw_opp).unwrap(), Instruction::parse(raw_res).unwrap()))
        .fold(0, |acc, (opp_move, instr)| acc + opp_move.appropriate_move(&instr).val() + instr.val());
    println!("Score: {}", result);
}
