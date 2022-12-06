use std::{io::{BufRead, BufReader}, fs::File};


fn is_overlapped(min_a:u32, max_a:u32, min_b:u32, max_b:u32)->bool{
    (min_a>=min_b && max_a<=max_b) || (min_b>=min_a && max_b<=max_a)
}


pub fn count_overlapped_pairs(filepath: &str) -> u32{
    let file = File::open(filepath).expect(&("Failed to open file: ".to_owned() + filepath));
    let reader = BufReader::new(file);

    let mut total_overlap= 0;
    for line in reader.lines() {
        
        
        let pairs = line
            .expect("error reading line").
            split(",")
            .map(String::from)
            .collect::<Vec<_>>();
        
        let a = &pairs[0].split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let b = &pairs[1].split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

       
        let min_a = a[0];
        let max_a = a[1];
        let min_b = b[0];
        let max_b = b[1];

        if is_overlapped(min_a, max_a, min_b, max_b) {
            total_overlap+=1;
        }

    }
    total_overlap
}


fn is_overlapped_2(min_a:u32, max_a:u32, min_b:u32, max_b:u32)->bool{
    (min_a>=min_b && min_a<=max_b) || (min_b>=min_a && min_b<=max_a)
}


pub fn count_overlapped_pairs_2(filepath: &str) -> u32{
    let file = File::open(filepath).expect(&("Failed to open file: ".to_owned() + filepath));
    let reader = BufReader::new(file);

    let mut total_overlap= 0;
    for line in reader.lines() {
        
        
        let pairs = line
            .expect("error reading line").
            split(",")
            .map(String::from)
            .collect::<Vec<_>>();
        
        let a = &pairs[0].split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let b = &pairs[1].split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

       
        let min_a = a[0];
        let max_a = a[1];
        let min_b = b[0];
        let max_b = b[1];

        if is_overlapped_2(min_a, max_a, min_b, max_b) {
            total_overlap+=1;
        }

    }
    total_overlap
}

#[cfg(test)]
mod tests {
    use crate::day_4::{count_overlapped_pairs, count_overlapped_pairs_2};

    #[test]
    fn test_count_overlapped_pairs() {
        assert_eq!(
            count_overlapped_pairs("src/inputFiles/4_example.txt"),
            2
        );
    }

    #[test]
    fn test_count_overlapped_pairs_2() {
        assert_eq!(
            count_overlapped_pairs_2("src/inputFiles/4_example.txt"),
            4
        );
    }

}
