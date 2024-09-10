use std::fs::File;
use std::io::{BufRead, BufReader};
use std::u32;

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Cube>,
}

#[derive(Debug)]
struct Cube {
    blue: u32,
    red: u32,
    green: u32,
}

fn main() {
    let file = BufReader::new(File::open("content/input.txt").expect("Cannot open input.txt"));
    let puzzle_input = puzzle_input(file);
    if let Some(successful_games) = puzzle_calculation(puzzle_input) {
        let games_sum: u32 = successful_games.iter().sum();
        println!("Successful Games Sum: {:?}", games_sum);
    }
}

fn puzzle_input(file: BufReader<File>) -> Vec<Game> {
    let mut cube_values: Vec<Game> = Vec::new();

    for line in file.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(':').collect();
            let game_fetch: Vec<&str> = parts.first().expect("No id").split(' ').collect();
            let game_id: u32 = game_fetch
                .last()
                .unwrap()
                .parse::<u32>()
                .expect("no number");

            let rounds_data: Vec<&str> = parts[1].split(';').collect();

            let mut rounds: Vec<Cube> = Vec::new();

            for round in rounds_data {
                let cube_data: Vec<(&str, &str)> = round
                    .split(',')
                    .map(|s| s.trim_start().split_once(' ').unwrap())
                    .collect();

                let mut cube = Cube {
                    blue: 0,
                    red: 0,
                    green: 0,
                };

                for (cube_amount, cube_color) in cube_data {
                    let cube_amount = cube_amount.parse::<u32>().expect("number not valid");

                    match cube_color {
                        "blue" => cube.blue += cube_amount,
                        "red" => cube.red += cube_amount,
                        "green" => cube.green += cube_amount,
                        _ => panic!("invalid color"),
                    }
                }

                rounds.push(cube);
            }

            let game = Game {
                id: game_id,
                rounds,
            };
            cube_values.push(game);
        }
    }

    cube_values
}

fn puzzle_calculation(games: Vec<Game>) -> Option<Vec<u32>> {
    let mut successful_games: Vec<u32> = Vec::new();
    let my_cubes: [Cube; 1] = [Cube {
        blue: 14,
        red: 12,
        green: 13,
    }];

    for game in games {
        let mut game_successful = true;

        for round in &game.rounds {
            let my_cube = &my_cubes[0];

            if my_cube.blue < round.blue || my_cube.red < round.red || my_cube.green < round.green {
                game_successful = false;
                break;
            }
        }

        if game_successful {
            successful_games.push(game.id);
        }
    }

    Some(successful_games)
}
