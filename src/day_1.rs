use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn max_elf_calories(filepath: &str) -> u32 {
    let file = File::open(filepath).expect("Could not read file");
    let reader = BufReader::new(file);

    let mut max = 0;
    let mut total_for_elf = 0;
    for line in reader.lines() {
        let calories = line
            .expect("Error reading line of file")
            .trim()
            .parse::<u32>();

        match calories {
            Ok(n) => total_for_elf += n,
            Err(_error) => {
                if total_for_elf > max {
                    max = total_for_elf
                }
                total_for_elf = 0;
            }
        }
    }
    if total_for_elf > max {
        //handle last elf
        max = total_for_elf
    }

    max
}

pub fn top_n_elf_calories(num_elves: u32, filepath: &str) -> u32 {
    let file = File::open(filepath).expect("Could not read file");
    let reader = BufReader::new(file);

    let mut sum_vec: Vec<u32> = vec![];
    let mut total_for_elf = 0;
    for line in reader.lines() {
        let calories = line
            .expect("Error reading line of file")
            .trim()
            .parse::<u32>();

        match calories {
            Ok(n) => total_for_elf += n,
            Err(_error) => {
                sum_vec.push(total_for_elf);
                total_for_elf = 0;
            }
        }
    }
    sum_vec.push(total_for_elf); // handles the last elf not having an empty row after them

    sum_vec.sort();
    let mut total = 0;
    for _i in 0..num_elves {
        total += sum_vec
            .pop()
            .expect("list shorter than requested number of elves");
    }

    //println!("{}", calories);
    total
}

#[cfg(test)]
mod tests {

    use crate::day_1::{max_elf_calories, top_n_elf_calories};

    #[test]
    fn test_max_elf_calories() {
        assert_eq!(
            max_elf_calories("src/inputFiles/aoc_1_1_example.txt"),
            24000
        );
    }

    #[test]
    fn test_top_n_elf_calories() {
        assert_eq!(
            top_n_elf_calories(3, "src/inputFiles/aoc_1_1_example.txt"),
            45000
        );
    }
}
