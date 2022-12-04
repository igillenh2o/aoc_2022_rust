
mod day_1;

fn main() {
    println!("Day 1 Max elf calories: {}", day_1::max_elf_calories("src/inputFiles/aoc_1_1_input.txt"));
    println!("Day 1 Top 3 elf calories sum: {}", day_1::top_n_elf_calories(3, "src/inputFiles/aoc_1_1_input.txt"));
}
