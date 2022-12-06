use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashSet,
};
use itertools::Itertools;


fn get_priority(letter: char) -> u32{
    let value = letter as u32;
   if value >= 98 {
     value - 96
   } else{
     value -  38
   }
}

pub fn priority_sum(filepath: &str) -> u32 {
    let file = File::open(filepath).expect("Could not read file");
    let reader = BufReader::new(file);
    // a more brute force sort of approach
    let mut score = 0;
    for line in reader.lines() {
        let rucksack = line.expect("issue reading line of file");
        let (pocket1, pocket2) = rucksack.split_at(rucksack.chars().count()/2);
        for (_size,letter) in pocket1.chars().enumerate(){
            if pocket2.contains(letter){
                let priority = get_priority(letter);
                score += priority;
                // println!("unique letter: {:?}, {}", letter, priority);
                break;
            }
        }
    }
    score
}


pub fn priority_sum2(filepath: &str) -> u32 {
    let file = File::open(filepath).expect("Could not read file");
    let reader = BufReader::new(file);

    let mut score = 0;
    //an approach using sets
    for (a, b,c) in  reader.lines().tuples(){
        let set_a = a.expect("error reading first line").chars().collect::<HashSet<char>>();
        let set_b = b.expect("error reading second line").chars().collect::<HashSet<char>>();
        
        for (_size, letter) in c.expect("error reading third line").chars().enumerate(){
            if set_a.contains(&letter) && set_b.contains(&letter){
                score += get_priority(letter);
                break
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use crate::day_3::{priority_sum, priority_sum2};

    #[test]
    fn test_priority_sum() {
        assert_eq!(
            priority_sum("src/inputFiles/3_example.txt"),
            157
        );
    }

    #[test]
    fn test_priority_sum2() {
        assert_eq!(
            priority_sum2("src/inputFiles/3_example.txt"),
            70
        );
    }

}
