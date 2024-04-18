use get_size::GetSize;
use std::collections::{HashMap, HashSet, VecDeque};

pub const MAX_CARS: usize = 8; // 4 or 8 bytes (depends on the target architecture)
pub const BOARD_SIZE: usize = 6; // 4 or 8 bytes (depends on the target architecture)
pub const DIRECTIONS: [Direction; 4] = [
    Direction::Right,
    Direction::Up,
    Direction::Down,
    Direction::Left,
];

pub type Board = [[u8; BOARD_SIZE]; BOARD_SIZE]; // 6x6 board -> bytes: 6 * 6 * 1 = 36 bytes

pub type Point = (u8, u8); // (x, y) -> bytes: 2 * 1 = 2 bytes

pub type Step = (u8, Direction);

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, GetSize)]
pub enum Orientation {
    Horizontal,
    Vertical,
} // 1 byte

#[derive(Debug, GetSize)]
pub struct Car {
    pub id: u8,                   // 1 byte
    pub length: u8,               // 1 byte
    pub orientation: Orientation, // 1 byte
} // 3 bytes

impl Car {
    pub fn get_orientation_from_board(board: &Board, from: &CarPosition) -> Orientation {
        if from.position.0 < 5
            && board[from.position.1 as usize][from.position.0 as usize + 1] == from.id
        {
            Orientation::Horizontal
        } else {
            Orientation::Vertical
        }
    }

    pub fn get_size_from_board(board: &Board, from: &CarPosition) -> u8 {
        if Car::get_orientation_from_board(board, from) == Orientation::Horizontal {
            if from.position.0 < 4
                && board[from.position.1 as usize][from.position.0 as usize + 2] == from.id
            {
                return 3;
            }
        } else {
            if from.position.1 < 4
                && board[from.position.1 as usize + 2][from.position.0 as usize] == from.id
            {
                return 3;
            }
        }

        2
    }

    pub fn winner(state: &State) -> bool {
        for car in state {
            if car.id == 1 && car.position == (4, 2) {
                return true;
            }
        }

        false
    }

    pub fn get_end(car: &Car, position: &CarPosition) -> Point {
        match car.orientation {
            Orientation::Horizontal => (position.position.0 + car.length - 1, position.position.1),
            Orientation::Vertical => (position.position.0, position.position.1 + car.length - 1),
        }
    }

    pub fn get_positions(car: &Car, position: &CarPosition) -> Vec<Point> {
        match car.orientation {
            Orientation::Horizontal => {
                let mut positions = Vec::with_capacity(car.length as usize);
                for i in 0..car.length {
                    let x = position.position.0 + i;
                    let y = position.position.1;
                    positions.push((x, y));
                }
                positions
            }
            Orientation::Vertical => {
                let mut positions = Vec::with_capacity(car.length as usize);
                for i in 0..car.length {
                    let x = position.position.0;
                    let y = position.position.1 + i;
                    positions.push((x, y));
                }
                positions
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CarPosition {
    pub id: u8,          // 1 byte
    pub position: Point, // 2 bytes
} // 3 bytes

pub type State = Vec<CarPosition>; // Max 8 cars -> bytes: 8 * 3 = 24 bytes

#[derive(Debug)]
pub struct Game {
    pub cars: HashMap<u8, Car>,
}

impl Game {
    pub fn load(board: Board) -> (Game, State) {
        let mut cars: HashMap<u8, Car> = HashMap::with_capacity(MAX_CARS);
        let mut state: State = vec![
            CarPosition {
                id: 0,
                position: (0, 0),
            };
            MAX_CARS
        ];

        for col in 0..BOARD_SIZE {
            for line in 0..BOARD_SIZE {
                let id = board[col][line];
                if id == 0 {
                    continue;
                }

                if !cars.contains_key(&id) {
                    let current_position = CarPosition {
                        id,
                        position: (line as u8, col as u8),
                    };

                    let temp_car = Car {
                        id,
                        length: Car::get_size_from_board(&board, &current_position),
                        orientation: Car::get_orientation_from_board(&board, &current_position),
                    };

                    cars.insert(id, temp_car);
                    state[(id - 1) as usize] = current_position;
                }
            }
        }

        cars.remove(&0);
        state.drain(cars.len()..MAX_CARS); // Remove the empty cars (id = 0)
        state.reserve_exact(MAX_CARS - state.len());
        let game = Game { cars };

        (game, state)
    }

    pub fn print(&self, state: &State) {
        let mut board: Board = [[0; BOARD_SIZE]; BOARD_SIZE];

        for car_position in state {
            let car = self.cars.get(&car_position.id).unwrap();
            let positions = Car::get_positions(car, car_position);

            for position in positions {
                board[position.1 as usize][position.0 as usize] = car.id;
            }
        }

        for line in board.iter() {
            println!("{:?}", line);
        }
    }

    pub fn will_intersect(&self, other_car: CarPosition, dest: Point) -> bool {
        let car = self.cars.get(&other_car.id).unwrap();
        let car_end = Car::get_end(car, &other_car);

        return (dest.0 >= other_car.position.0 && dest.0 <= car_end.0)
            && (dest.1 >= other_car.position.1 && dest.1 <= car_end.1);

        // match car.orientation {
        //     Orientation::Horizontal => {
        //         (dest.position.0 <= car_end.0 && dest_end.0 >= other_car.position.0)
        //             && (dest.position.1 <= car_end.1 && dest_end.1 >= other_car.position.1)
        //     }
        //     Orientation::Vertical => {
        //         (dest.position.0 <= car_end.0 && dest_end.0 >= other_car.position.0)
        //             && (dest.position.1 <= car_end.1 && dest_end.1 >= other_car.position.1)
        //     }
        // }

        // false

        // match car.orientation {
        //     Orientation::Horizontal => {

        //         // car_end.0 == dest_end.0
        //         //     && (other_car.position.1 <= dest_end.1 && car_end.1 >= dest_end.1)

        //         //The overlap should be calculated if any of the points of the cars
        //         //will cross at any point
        //     }
        //     Orientation::Vertical => {
        //         car_end.1 == dest_end.1
        //             && (other_car.position.0 <= dest_end.0 && car_end.0 >= dest_end.0)
        //     }
        // }
    }

    pub fn get_moves(&self, state: &State) -> Vec<(State, Step)> {
        let mut moves: Vec<(State, Step)> = Vec::new();

        for vehicle in state {
            let car = self.cars.get(&vehicle.id).unwrap();
            'direction: for direction in DIRECTIONS {
                let dest = match (&car.orientation, &direction) {
                    (Orientation::Vertical, Direction::Up) => {
                        if vehicle.position.1 == 0 {
                            continue;
                        }

                        [
                            (vehicle.position.0, vehicle.position.1 - 1),
                            (vehicle.position.0, vehicle.position.1 - 1),
                        ]
                    }
                    (Orientation::Vertical, Direction::Down) => {
                        if vehicle.position.1 + car.length == BOARD_SIZE as u8 {
                            continue;
                        }

                        [
                            (vehicle.position.0, vehicle.position.1 + 1),
                            (vehicle.position.0, vehicle.position.1 + car.length),
                        ]
                    }
                    (Orientation::Horizontal, Direction::Left) => {
                        if vehicle.position.0 == 0 {
                            continue;
                        }

                        [
                            (vehicle.position.0 - 1, vehicle.position.1),
                            (vehicle.position.0 - 1, vehicle.position.1),
                        ]
                    }
                    (Orientation::Horizontal, Direction::Right) => {
                        if vehicle.position.0 + car.length == BOARD_SIZE as u8 {
                            continue;
                        }

                        [
                            (vehicle.position.0 + 1, vehicle.position.1),
                            (vehicle.position.0 + car.length, vehicle.position.1),
                        ]
                    }
                    _ => continue,
                };

                for vehicle in state {
                    if self.will_intersect(*vehicle, dest[1]) {
                        continue 'direction;
                    }
                }

                let mut new_state = state.clone();
                new_state[(car.id - 1) as usize].position = dest[0];
                moves.push((new_state, (car.id, direction)));
            }
        }

        //Try move the target car fisrt
        moves.sort_by(|(_, a), (_, b)| b.0.cmp(&a.0));
        moves
    }

    pub fn solve(&self, initial_state: State) -> Option<Vec<Step>> {
        let mut previous_states: HashSet<State> = HashSet::new();
        let mut next_states: Vec<(State, Vec<Step>)> = Vec::from([(initial_state, Vec::new())]);

        while let Some((state, history)) = next_states.pop() {
            if Car::winner(&state) {
                // self.print(&state);
                // println!("\n{:?}", history);
                return Some(history);
            }

            let moves = self.get_moves(&state);
            for (move_state, step) in moves {
                if previous_states.contains(&move_state) {
                    continue;
                }

                // println!("\n");
                // self.print(&state);
                previous_states.insert(state.clone());

                let mut new_history = history.clone();
                new_history.push(step);
                next_states.push((move_state, new_history));
            }
        }

        None
    }
}
