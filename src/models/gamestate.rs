use crate::models::player::Player;
// use rand::prelude::*;
use rand::{Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub board: Vec<Vec<i8>>,
    pub players: Vec<Player>,
    pub current_player: Option<usize>,
    pub turns: i8
}

/// GameState
impl GameState {
    pub fn print_board(&self) -> () {
        for x in &self.board {
            for y in x {
                print!("{} ", y);
            }
            println!("");
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    /// win validation
    pub fn winner(&self) -> bool {
        // TODO: move to server and
        // send TCP message to client if server detects a winner
        false
    }

    /// Get the Current Player from the list
    pub fn get_current_player(&self) -> &Player {
        let player = &self.players[self.current_player.unwrap()];
        return player;
    }


    /// Get User Start
    pub fn coinflip(&mut self) {
        let num = Some(rand::thread_rng().gen_range(0..2));
        println!("Player {} will start", num.unwrap());
        self.current_player = num
    }

    pub fn move_player(&mut self, input: &str) -> () {
        self.players[self.current_player.unwrap()].move_player(input);
    }
}
