use std::fs::File;
use std::io::{ BufRead, BufReader};

fn main() {
    let file = File::open("inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let digits: Vec<char> = line.chars().filter(|&c|c.is_digit(10)).collect();
        let first_digit = digits.first();
        let last_digit = digits.last();

        if let (Some(first),Some(last)) = (first_digit ,last_digit){ 
            let combined_digits = format!("{}{}", first,last);
            if let Ok(combined_digits) = combined_digits.parse::<i32>(){
               total+= combined_digits; 
            }
        }
    }

    println!("total {} ", total);

}
