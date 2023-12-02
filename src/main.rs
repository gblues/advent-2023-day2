use regex::Regex;

fn main() {
    let input = include_str!("input/2.txt");
    let mut total: i32 = 0;
    let line_parser = Regex::new(r"(?m)^Game (\d+):\s*(.*)\s*$").unwrap();
    let color_parser = Regex::new(r"^\s*(\d+)\s+(\w+)\s*$").unwrap();
    for (_, [_, game_data]) in line_parser.captures_iter(input).map(|c| c.extract()) {
        let mut most_red: i32 = 0;
        let mut most_green: i32 = 0;
        let mut most_blue: i32 = 0;

        for pull in game_data.split(";") {
            for colorset in pull.split(",") {
                let (_, [count_str, color]) = color_parser.captures(colorset).unwrap().extract::<2>();
                let count: i32 = count_str.parse().unwrap();

                match color {
                    "red" => {
                        most_red = if count > most_red {count} else {most_red};
                    },
                    "green" => {
                        most_green = if count > most_green {count} else {most_green};
                    },
                    "blue" => {
                        most_blue = if count > most_blue {count} else {most_blue};
                    },
                    _ => {}
                }
            }
        }
        let power: i32 = most_red * most_green * most_blue;
        total += power;
    }
    println!("Total: {}", total);
}
