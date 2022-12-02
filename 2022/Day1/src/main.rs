use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    result.to_string();
    println!("{}", result);
    }
