mod game;

pub use game::{Car, CarPosition, Direction, Game};

use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program::set_return_data, pubkey::Pubkey,
};

entrypoint!(process_instruction);

#[derive(BorshDeserialize, BorshSerialize)]
struct Payload {
    board: Vec<Vec<u8>>,
}

#[derive(BorshSerialize, Debug)]
struct Data {
    result: Vec<(u8, Direction)>,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    msg!("Initializing rush solver");

    let mut payload = Payload::try_from_slice(data).unwrap();
    payload.board.shrink_to_fit();

    let (game, initial_state) = Game::load(payload.board);

    if let Some(result) = game.solve(initial_state) {
        let mut data = Vec::new();
        msg!("Result {:?}", result);
        let _ = Data { result }.serialize(&mut data);
        set_return_data(&data.to_vec());
    } else {
        msg!("No solution found");
    }

    Ok(())
}
