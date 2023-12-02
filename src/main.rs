use regex::Regex;

fn main() {
    let input = include_str!("input/2.txt");
    let mut total: i32 = 0;
    let line_parser = Regex::new(r"(?m)^Game (\d+):\s*(.*)\s*$").unwrap();
    let color_parser = Regex::new(r"^\s*(\d+)\s+(\w+)\s*$").unwrap();
    for (_, [game_id_str, game_data]) in line_parser.captures_iter(input).map(|c| c.extract()) {
        let game_id: i32 = game_id_str.parse().unwrap();
        let mut is_possible: bool = true;
        for pull in game_data.split(";") {
            for colorset in pull.split(",") {
                let (_, [count_str, color]) = color_parser.captures(colorset).unwrap().extract::<2>();
                let count: i32 = count_str.parse().unwrap();

                match color {
                    "red" => {
                        if count > 12 { is_possible = false; }
                    },
                    "green" => {
                        if count > 13 { is_possible = false; }
                    },
                    "blue" => {
                        if count > 14 { is_possible = false; }
                    },
                    _ => {}
                }
            }
        }
        if is_possible {
            total += game_id;
        }
    }
    println!("Total: {}", total);
}
