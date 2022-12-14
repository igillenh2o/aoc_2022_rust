mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    println!("Day 1 Max elf calories: {}", day_1::max_elf_calories("src/inputFiles/1_input.txt"));
    println!("Day 1 Top 3 elf calories sum: {}", day_1::top_n_elf_calories(3, "src/inputFiles/1_input.txt"));

    println!("Day 2 Rock, paper, scissors score: {}", day_2::rock_paper_scissors_score("src/inputFiles/2_input.txt"));
    println!("Day 2 Rock, paper, scissors score 2: {}", day_2::rock_paper_scissors_score2("src/inputFiles/2_input.txt"));

    println!("Day 3 Priority sum: {}", day_3::priority_sum("src/inputFiles/3_input.txt"));
    println!("Day 3 Priority sum2: {}", day_3::priority_sum2("src/inputFiles/3_input.txt"));

    println!("Day 4 Count overlap: {}", day_4::count_overlapped_pairs("src/inputFiles/4_input.txt"));
    println!("Day 4 Count overlap2: {}", day_4::count_overlapped_pairs_2("src/inputFiles/4_input.txt"));
   // println!("Day 3 Priority sum2: {}", day_3::priority_sum2("src/inputFiles/3_input.txt"));
}
