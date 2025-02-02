mod snake;
mod game;

use game::Game;
use rand::Rng;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use snake::Snake;

const TOP_MARGIN: i32 = 50;
const CELL_SIZE: i32 = 30;
const WIDTH_CELLS: i32 = 32;
const HEIGHT_CELLS: i32 = 18;

fn main() {
    let mut snake = Snake::new(WIDTH_CELLS, HEIGHT_CELLS);
    let mut game = Game::new(
        CELL_SIZE,
        WIDTH_CELLS,
        HEIGHT_CELLS,
        TOP_MARGIN,
        &mut snake,
    );

    let (mut rl, thread) = raylib::init()
        .size(game.width(), game.height())
        .title("Snake 0.0.1")
        .build();

    rl.set_target_fps(60);

    let texture = rl
        .load_texture(&thread, "image_50x50.png")
        .expect("failed to load image");

    game.run(&mut rl, &texture, &thread);
}

