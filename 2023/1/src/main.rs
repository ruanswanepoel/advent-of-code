use std::io::prelude::*;

fn main() {
    let file_path = "calibration.txt";
    let file = std::fs::File::open(file_path).expect("Could not open file");
    let reader = std::io::BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let val = get_line_value(l);
        sum += val;
    }

    println!("{}", sum);
}

fn get_line_value(line: String) -> i32 {
    let chars = line.chars();

    let first_occurance = get_first_number(chars.clone());
    let last_occurance = get_first_number(chars.rev());

    let final_number = format!("{}{}", first_occurance, last_occurance);
    final_number.parse::<i32>().unwrap()
}

fn get_first_number<'a, I>(mut vals: I) -> char
where
    I: Iterator<Item = char>,
{
    let numbers = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    let q = vals.find(|c| numbers.contains(&c)).unwrap();
    q
}
