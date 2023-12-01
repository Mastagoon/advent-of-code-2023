use std::{fs, println};

fn main() {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let file = fs::read_to_string("../../input.txt").unwrap();
    let mut total = 0;
    file.lines().for_each(|x| {
        let v: Vec<char> = x.chars().filter(|c| c.is_digit(10)).collect();
        let mut first = v.first().unwrap().clone();
        let mut last = v.last().unwrap().clone();
        let mut first_pos = x.find(first).unwrap();
        let mut last_pos = x.rfind(last).unwrap();

        for (i, n) in nums.iter().enumerate() {
            if let Some(pos) = x.find(n) {
                if pos < first_pos {
                    first = (i + 1).to_string().chars().next().unwrap();
                    first_pos = pos;
                }
            }
            if let Some(pos) = x.rfind(n) {
                if pos > last_pos {
                    last = (i + 1).to_string().chars().next().unwrap();
                    last_pos = pos;
                }
            }
        }
        let s: String = [first, last].iter().collect();
        total += s.parse::<i32>().unwrap();
    });
    println!("{}", total);
}
