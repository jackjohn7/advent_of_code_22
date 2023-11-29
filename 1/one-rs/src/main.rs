fn main() {
    // this macro actually bakes the content of a file into a string at compile time
    let contents = include_str!("../../input.txt");

    // after having looked at someone else's solution briefly, I learned about .lines, .sum, and .max which greatly simplify what I was doing.
    //  after learning about those, I rewrote the function using them and it's basically identical to what the guy online wrote.
    let best_result = contents
        .split("\n\n")
        .map(|sub_str| {
            sub_str
                .lines()
                .map(|x| x.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    println!("The highest calorie count is: {}", best_result);

    let result = contents
        .split("\n\n")
        .map(|sub_str| {
            sub_str
                .lines()
                .fold(0 as u32, |acc, x| acc + x.parse::<u32>().unwrap())
        })
        .enumerate()
        .fold(
            None,
            |acc_max: Option<(usize, u32)>, (idx, cals)| match &acc_max {
                None => Some((idx + 1, cals)),
                Some(current_max) => match current_max.1.cmp(&cals) {
                    std::cmp::Ordering::Less => Some((idx + 1, cals)),
                    std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => acc_max,
                },
            },
        )
        .unwrap();

    println!(
        "The elf with the most food is elf {} with {} cals",
        result.0, result.1
    );
}
