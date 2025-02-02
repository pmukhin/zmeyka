mod snake;
mod game;
use game::Game;
use snake::Snake;
use serde::Deserialize;
use std::fs;
use toml::de::Error;

#[derive(Deserialize)]
struct Config {
    top_margin: i32,
    cell_size: i32,
    width_cells: i32,
    height_cells: i32,
}

fn load_config() -> Result<Config, Error> {
    let config_str = fs::read_to_string("config.toml")
        .expect("can't open config.toml");

    toml::from_str(&config_str)
}

fn main() {
    let config = load_config().unwrap();

    let mut snake = Snake::new(config.width_cells, config.width_cells);
    let mut game = Game::new(
        config.cell_size,
        config.width_cells,
        config.height_cells,
        config.top_margin,
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

