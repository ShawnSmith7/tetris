use super::Game;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GameManager;

impl GameManager {
    pub fn run(&self) {
        let mut game = Game::default();
        game.run();
    }
}