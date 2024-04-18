use game::Game;

mod game;

fn main() {
    let initial_board = [
        [0, 0, 4, 2, 2, 2],
        [0, 0, 4, 0, 0, 0],
        [1, 1, 4, 0, 0, 0],
        [5, 0, 0, 6, 6, 3],
        [5, 0, 0, 0, 7, 3],
        [8, 8, 8, 0, 7, 3],
    ];

    // let initial_board = [
    //     [0, 0, 0, 2, 2, 2],
    //     [0, 0, 0, 0, 0, 0],
    //     [1, 1, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0],
    // ];

    let (game, initial_state) = Game::load(initial_board);
    if let Some(result) = game.solve(initial_state) {
        println!("Solution found:");

        println!("{:?}", result.len());
        // for step in result {
        // print!("{:?} ", res);
        // }
    } else {
        println!("No solution found");
    }

    ()
}
