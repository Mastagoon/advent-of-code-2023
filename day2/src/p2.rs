use std::fs;

struct Game {
    red: i32,
    blue: i32,
    green: i32,
}

pub fn run() -> i32 {
    let file = fs::read_to_string("./input.txt").unwrap();
    let mut sum = 0;
    file.lines().for_each(|line| {
        let mut min_red = 1;
        let mut min_green = 1;
        let mut min_blue = 1;
        line.split(":").nth(1).unwrap().split(";").for_each(|play| {
            let mut game = Game {
                red: 0,
                blue: 0,
                green: 0,
            };
            play.split(",").for_each(|p| {
                let color = p.trim().split(" ").nth(1).unwrap();
                let count = p.trim().split(" ").nth(0).unwrap();
                match color.to_lowercase().as_str() {
                    "red" => {
                        game.red = count.parse::<i32>().unwrap_or(0);
                        if game.red > min_red {
                            min_red = game.red;
                        }
                    }
                    "green" => {
                        game.green = count.parse::<i32>().unwrap_or(0);
                        if game.green > min_green {
                            min_green = game.green;
                        }
                    }
                    "blue" => {
                        game.blue = count.parse::<i32>().unwrap_or(0);
                        if game.blue > min_blue {
                            min_blue = game.blue;
                        }
                    }
                    _ => println!("Invalid color"),
                }
            });
        });
        sum += min_blue * min_green * min_red;
    });
    return sum;
}
