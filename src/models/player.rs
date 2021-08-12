use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    /// X and Y
    pub position: Vec<i8>,
}

// TODO
enum Movement {}

impl Player {
    pub fn move_player(&mut self, input: &str) {
        match input {
            "w" => self.position[1] = self.position[1] + 1,
            "s" => self.position[1] = self.position[1] - 1,
            "a" => self.position[0] = self.position[0] - 1,
            "d" => self.position[0] = self.position[0] + 1,
            _ => panic!("Invalid movement")
        };
        print!("{:?}", self.position);
    }
}
