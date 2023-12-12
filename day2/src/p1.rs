use std::fs;

struct Game {
    red: i32,
    blue: i32,
    green: i32,
}

pub fn run() -> usize {
    let file = fs::read_to_string("./input.txt").unwrap();
    let mut sum = 0;
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    file.lines().enumerate().for_each(|(idx, line)| {
        let game_id = idx + 1;
        let plays = line.split(":").nth(1).unwrap().split(";").map(|play| {
            let mut game = Game {
                red: 0,
                blue: 0,
                green: 0,
            };
            // 10 red, 7 green, 3 blue
            play.split(",").for_each(|p| {
                let color = p.trim().split(" ").nth(1).unwrap();
                let count = p.trim().split(" ").nth(0).unwrap();
                match color.to_lowercase().as_str() {
                    "red" => game.red += count.parse::<i32>().unwrap_or(0),
                    "green" => game.green += count.parse::<i32>().unwrap_or(0),
                    "blue" => game.blue += count.parse::<i32>().unwrap_or(0),
                    _ => println!("Invalid color"),
                }
            });
            if game.green <= max_green && game.red <= max_red && game.blue <= max_blue {
                return true;
            } else {
                return false;
            }
        });
        if plays.filter(|play| !*play).count() == 0 {
            sum += game_id;
        }
    });
    return sum;
}
