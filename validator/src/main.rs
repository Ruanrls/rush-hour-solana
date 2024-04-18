use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn read_from_file() -> Result<String, std::io::Error> {
    let mut file = File::open("/Users/ruanrls/Desktop/my-solution/validator/src/test.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_values(data: &str) -> Vec<Step> {
    let regex = Regex::new(r"(?<steps>\d+), (?<direction>\w+)").unwrap();
    let steps: Vec<Step> = regex
        .captures_iter(&data)
        .map(|cap| {
            let steps = cap["steps"].parse::<u8>().unwrap();
            let direction = match &cap["direction"] {
                "Up" => Direction::Up,
                "Down" => Direction::Down,
                "Left" => Direction::Left,
                "Right" => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            (steps, direction)
        })
        .collect();

    steps
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    Horizontal,
    Vertical,
}

type Step = (u8, Direction);
type Board = [[u8; 6]; 6];

#[derive(Clone, Debug)]
struct Car {
    id: u8,
    orientation: Orientation,
    positions: Vec<(usize, usize)>,
}

fn map_cars(board: &Board) -> HashMap<u8, Car> {
    let mut cars: HashMap<u8, Car> = HashMap::new();

    for i in 0..6 {
        for j in 0..6 {
            let car = board[i][j];
            if car == 0 {
                continue;
            }

            cars.entry(car)
                .and_modify(|entry| {
                    entry.positions.push((j, i));

                    if entry.positions[0].1 == i {
                        entry.orientation = Orientation::Horizontal;
                    } else {
                        entry.orientation = Orientation::Vertical;
                    }
                })
                .or_insert(Car {
                    id: car,
                    orientation: Orientation::Horizontal,
                    positions: Vec::from([(j, i)]),
                });
        }
    }

    cars
}

fn move_car(board: &mut Board, cars: &mut HashMap<u8, Car>, step: Step, play: u32) {
    let (car_id, direction) = step;

    cars.entry(car_id)
        .and_modify(|car| match (car.orientation, direction) {
            (Orientation::Horizontal, Direction::Left) => {
                let (x, y) = car.positions[0];

                if x == 0 {
                    panic!("Invalid move detected, step: {:?} - play: {play}", step)
                }

                if board[y][x - 1] != 0 {
                    panic!("Invalid move detected, step: {:?} - play: {play}", step)
                }

                let (x, y) = car.positions.last().unwrap();
                board[*y][*x] = 0;

                car.positions.iter_mut().for_each(|(x, y)| {
                    board[*y][*x - 1] = car.id;
                    *x -= 1;
                });
            }
            (Orientation::Horizontal, Direction::Right) => {
                let (x, y) = car.positions.last().unwrap();

                if *x == 5 {
                    panic!("Invalid move detected, step: {:?}  - play: {play}", step)
                }

                if board[*y][x + 1] != 0 {
                    panic!("Invalid move detected, step: {:?}  - play: {play}", step)
                }

                let (x, y) = car.positions[0];
                board[y][x] = 0;

                car.positions.iter_mut().for_each(|(x, y)| {
                    board[*y][*x + 1] = car.id;
                    *x += 1;
                });
            }
            (Orientation::Vertical, Direction::Up) => {
                let (x, y) = car.positions[0];

                if y == 0 {
                    panic!(
                        "from up Invalid move detected, step: {:?}  - play: {play}",
                        step
                    )
                }

                if board[y - 1][x] != 0 {
                    panic!("ok Invalid move detected, step: {:?}  - play: {play}", step)
                }

                let (x, y) = car.positions.last().unwrap();
                board[*y][*x] = 0;

                car.positions.iter_mut().for_each(|(x, y)| {
                    board[*y - 1][*x] = car.id;
                    *y -= 1;
                });
            }
            (Orientation::Vertical, Direction::Down) => {
                let (x, y) = car.positions.last().unwrap();

                if *y == 5 {
                    panic!("Invalid move detected, step: {:?}  - play: {play}", step)
                }

                if board[y + 1][*x] != 0 {
                    panic!("Invalid move detected, step: {:?}  - play: {play}", step)
                }

                let (x, y) = car.positions[0];
                board[y][x] = 0;

                car.positions.iter_mut().for_each(|(x, y)| {
                    board[*y + 1][*x] = car.id;
                    *y += 1;
                });
            }
            _ => panic!("Invalid move detected, step: {:?}  - play: {play}", step),
        });
}

fn print_board(board: &Board) {
    for i in 0..6 {
        for j in 0..6 {
            print!("{:?} ", board[i][j]);
        }
        println!();
    }
}

fn main() {
    let data = read_from_file().unwrap();
    let steps = parse_values(&data);

    let mut board: Board = [
        [0, 0, 4, 2, 2, 2],
        [0, 0, 4, 0, 0, 0],
        [1, 1, 4, 0, 0, 0],
        [5, 0, 0, 6, 6, 3],
        [5, 0, 0, 0, 7, 3],
        [8, 8, 8, 0, 7, 3],
    ];

    //validate board

    let mut play: u32 = 0;
    let mut cars = map_cars(&board);
    for step in steps {
        play += 1;
        move_car(&mut board, &mut cars, step, play);
        println!("-----");
        print_board(&board);
    }
}
