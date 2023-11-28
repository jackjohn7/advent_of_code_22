use std::fs::File;
use std::io::Read;

fn main() {

    let mut file = File::open("input.txt").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let results: Vec<u32> = contents
        .trim()
        .split("\n\n")
        .map(|sub_str| sub_str.split("\n")
        .fold(0 as u32, |acc, x| acc + x.parse::<u32>().unwrap()))
        .collect();

    let mut elf_with_most_food = 0;
    for (idx, res) in results.iter().enumerate() {
        if res > &results[elf_with_most_food] {
            elf_with_most_food = idx;
        }
    }

    println!("The elf with the most food is elf {} with {} cals", elf_with_most_food+1, results[elf_with_most_food]);
}
