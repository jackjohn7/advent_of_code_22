fn main() {
    let contents = include_str!("../../input.txt");

    // this is my first attempt at this. I compared with someone online's solution and the only
    //  difference between mine and theirs is that my solution doesn't require the reversal
    //  of the vector afterward because I compared differently in line 11. Happy with this one.

    let mut result = contents
        .split("\n\n")
        .map(|elf_raw| {
            elf_raw
                .lines()
                .map(|val| val.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    result.sort_by(|a, b| b.cmp(a));

    println!("result: {}", result.into_iter().take(3).sum::<u32>());
}
