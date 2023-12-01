use std::{fs, println};

fn main() {
    let file = fs::read_to_string("../../input.txt").unwrap();
    let mut total = 0;
    let _ = file.lines().for_each(|x| {
        let v: Vec<char> = x.chars().filter(|c| c.is_digit(10)).collect();
        let mut s: String = "".to_string();
        s.push(*v.first().unwrap());
        s.push(*v.last().unwrap());
        total += s.parse::<i32>().unwrap();
    });
    println!("{}", total);
}
