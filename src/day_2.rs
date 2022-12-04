use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn score_line(line: &str)-> u32{
    match line {
        "A X" => 4,  //Rock draws
        "A Y" => 8, // Paper wins
        "A Z" => 3, // Scissors loses
        "B X" => 1, // Rock loses
        "B Y" => 5, // Paper draws
        "B Z" => 9, // Scissors wins
        "C X" => 7, // Rock wins
        "C Y" => 2, // Paper loses
        "C Z" => 6, // Scissors draw
        _=>panic!()
    }
}

pub fn rock_paper_scissors_score(filepath: &str) -> u32 {
    let file = File::open(filepath).expect("Could not read file");
    let reader = BufReader::new(file);

    let mut score = 0;
    for line in reader.lines() {
        let round = line.expect("issue reading line of file");
        score += score_line(&round);
        }
    score
}

fn score_line_part_2(line: &str)-> u32{
    // X lose, Y draw, Z win
    match line {
        "A X" => 3, // Scissors loses
        "A Y" => 4, // Rock draws
        "A Z" => 8, // Paper wins
        "B X" => 1, // Rock loses
        "B Y" => 5, // Paper draws
        "B Z" => 9, // Scissors wins
        "C X" => 2, // Paper loses
        "C Y" => 6, // Scissors draws
        "C Z" => 7, // Rock wins
        _=>panic!()
    }
}

pub fn rock_paper_scissors_score2(filepath: &str) -> u32 {
    let file = File::open(filepath).expect("Could not read file");
    let reader = BufReader::new(file);

    let mut score = 0;
    for line in reader.lines() {
        let round = line.expect("issue reading line of file");
        score += score_line_part_2(&round);
        }
    score
}


#[cfg(test)]
mod tests {
    use crate::day_2::{rock_paper_scissors_score, rock_paper_scissors_score2};


    #[test]
    fn test_rock_paper_scissors_score() {
        assert_eq!(
            rock_paper_scissors_score("src/inputFiles/2_example.txt"),
            15
        );
    }

    #[test]
    fn test_rock_paper_scissors_score2() {
        assert_eq!(
            rock_paper_scissors_score2("src/inputFiles/2_example.txt"),
            12
        );
    }

}
