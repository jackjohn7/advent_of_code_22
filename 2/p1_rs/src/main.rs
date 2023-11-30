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

fn main() {
    let input = include_str!("../../input.txt");
    let result = input
        .lines()
        .map(|line| {
            let choices: Vec<Choice> = line.split(" ").map(|choice_raw| Choice::parse(choice_raw).unwrap()).collect();
            (choices[0].clone(), choices[1].clone())
        }).fold(0, |acc, (opp, me)| acc + me.play(&opp).unwrap() + me.val());
    println!("Score: {}", result);
}
