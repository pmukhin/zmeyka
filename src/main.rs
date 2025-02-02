mod snake;

use rand::Rng;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use snake::{Direction, Pt, Snake};
use std::process::exit;
use std::thread;
use std::thread::Thread;
use std::time::Instant;
use rand::rngs::ThreadRng;

const TOP_MARGIN: i32 = 50;
const CELL_SIZE: i32 = 30;
const WIDTH_CELLS: i32 = 32;
const HEIGHT_CELLS: i32 = 18;
const COLOR_LIGHT_LIGHT_GRAY: Color = Color::new(0, 0, 0, 16);

struct Game<'a> {
    width: i32,
    height: i32,
    snake: &'a mut Snake,
    rng: ThreadRng,
    food_pt: Pt,
    counter: u64,
    score: usize,
    paused: bool,
    game_over: bool,
    pop_exit: bool,
    start_time: Instant,
}

impl<'a> Game<'a> {
    fn new(
        cell_size: i32,
        width_cells: i32,
        height_cells: i32,
        snake: &'a mut Snake,
    ) -> Game<'a> {
        let width = cell_size * width_cells;
        let height = cell_size * height_cells + TOP_MARGIN;

        Game {
            width,
            height,
            snake,
            rng: Default::default(),
            food_pt: Default::default(),
            counter: 0,
            score: 0,
            paused: false,
            game_over: false,
            pop_exit: false,
            start_time: Instant::now(),
        }
    }
}

impl Game<'_> {
    fn on_game_over(
        &mut self,
        d: &mut RaylibDrawHandle,
    ) {
        let text =
            format!("Game over! Your score is {}.\n\n\n Start again? Press Enter!", self.score);
        d.draw_text(&text, 260, 239, 32, Color::BLACK);

        if d.is_key_pressed(KEY_ENTER) {
            self.snake.reset();
            self.score = 0;
            self.start_time = Instant::now();
            self.game_over = false;
        }
    }

    fn draw_piggy(&self, d: &mut impl RaylibDraw, texture: &Texture2D) {
        let source_rect = Rectangle::new(
            0.0,
            0.0,
            texture.width() as f32,
            texture.height() as f32,
        );

        let dest_rect = Rectangle::new(
            self.food_pt.0 as f32 * CELL_SIZE as f32,
            self.food_pt.1 as f32 * CELL_SIZE as f32 + TOP_MARGIN as f32,
            CELL_SIZE as f32,
            CELL_SIZE as f32,
        );

        for i in 0..HEIGHT_CELLS {
            d.draw_rectangle(
                self.food_pt.0 * CELL_SIZE,
                i * CELL_SIZE + TOP_MARGIN,
                CELL_SIZE,
                CELL_SIZE,
                COLOR_LIGHT_LIGHT_GRAY,
            );
        }
        for j in 0..WIDTH_CELLS {
            d.draw_rectangle(
                j * CELL_SIZE,
                self.food_pt.1 * CELL_SIZE + TOP_MARGIN,
                CELL_SIZE,
                CELL_SIZE,
                COLOR_LIGHT_LIGHT_GRAY,
            );
        }

        let origin = Vector2::new(0.0, 0.0); // No rotation offset
        d.draw_texture_pro(
            &texture, source_rect, dest_rect, origin, 0.0,
            Color::WHITE,
        );
    }

    fn draw_score(
        &self,
        d: &mut impl RaylibDraw,
    ) {
        let text = format!("Score: {}, length of the snake: {}", self.score, self.snake.len());
        d.draw_text(&text, 10, 18, 20, Color::BLACK);
        d.draw_text(curr_duration_formatted(&self.start_time).as_str(),
                    self.width - 60, 18, 20, Color::BLACK);
    }

    fn draw_snake(&self, d: &mut impl RaylibDraw) {
        self.snake.draw(|x, y| {
            d.draw_rectangle(
                x * CELL_SIZE,
                y * CELL_SIZE + TOP_MARGIN,
                CELL_SIZE,
                CELL_SIZE,
                Color::BLACK,
            )
        });
    }

    fn draw_lines(&self, d: &mut impl RaylibDraw) {
        let mut y = TOP_MARGIN;

        while y < self.height {
            d.draw_line(0, y, self.width, y, Color::GRAY);
            y += CELL_SIZE;
        }

        let mut x = CELL_SIZE;
        while x < self.width {
            d.draw_line(x, TOP_MARGIN, x, self.height, Color::GRAY);
            x += CELL_SIZE
        }
    }

    fn pop_up(&self, d: &mut impl RaylibDraw, text: &str) {
        let len_text = text.len() as f32 * 18.0; // font_size
        let total_width = len_text + 10.0 + 6.0; // + 5px padding at every side + 3px line x 2
        let total_margin = self.width as f32 - total_width;
        let half_margin = total_margin / 2.0;

        let popup_rect = Rectangle::new(
            half_margin + 3.0,
            200.0,
            total_width - 6.0,
            80.0,
        );
        let text_starts = (popup_rect.width - len_text) as i32 / 2;

        d.draw_rectangle_rec(popup_rect, Color::LIGHTGRAY);
        d.draw_rectangle_lines_ex(popup_rect, 3.0, Color::BLACK);

        d.draw_text(text, half_margin as i32 + text_starts + 83, 230, 20, Color::BLACK);
    }

    fn draw_pop_ups(&mut self, d: &mut impl RaylibDraw) {
        if self.pop_exit {
            self.pop_up(d, "To exit, press Enter, to resume, press Space");
        }

        if self.paused {
            self.pop_up(d, "Press Enter to continue!");
        }
    }

    fn inc_counter(&mut self) {
        self.counter += 1;
    }

    fn should_make_move(&self) -> bool {
        self.counter % 5 == 0 && !self.paused && !self.pop_exit
    }

    fn step(&mut self) {
        if self.should_make_move() {
            let mut growing = false;

            if self.snake.head() == self.food_pt {
                self.food_pt = Pt(
                    self.rng.random_range(0..WIDTH_CELLS),
                    self.rng.random_range(0..HEIGHT_CELLS),
                );
                self.score += 1;
                growing = true;
            }

            if self.snake.collapsed_into_self() {
                self.game_over = true;
            }
            if !self.game_over {
                self.snake.make_move(growing);
            }
        }
    }

    fn run(&mut self, rl: &mut RaylibHandle, texture: &Texture2D, thread: &RaylibThread) {
        while !rl.window_should_close() {
            if rl.is_key_pressed(KEY_LEFT) {
                self.snake.set_direction(Direction::Left);
            } else if rl.is_key_pressed(KEY_RIGHT) {
                self.snake.set_direction(Direction::Right);
            } else if rl.is_key_pressed(KEY_UP) {
                self.snake.set_direction(Direction::Up);
            } else if rl.is_key_pressed(KEY_DOWN) {
                self.snake.set_direction(Direction::Down);
            } else if rl.is_key_pressed(KEY_SPACE) && !self.game_over && !self.pop_exit {
                self.paused = true;
            } else if rl.is_key_pressed(KEY_ENTER) && self.paused {
                self.paused = false;
            } else if rl.is_key_down(KEY_LEFT_CONTROL) && rl.is_key_pressed(KEY_C) {
                self.pop_exit = true;
            } else if rl.is_key_pressed(KEY_ENTER) && self.pop_exit {
                exit(0);
            } else if rl.is_key_pressed(KEY_SPACE) && self.pop_exit {
                self.pop_exit = false;
            }

            self.step();

            let mut d = rl.begin_drawing(thread);

            d.clear_background(Color::WHITE);

            if !self.game_over {
                self.draw_piggy(&mut d, &texture);
                self.draw_snake(&mut d);
                self.draw_lines(&mut d);
                self.draw_score(&mut d);
            } else {
                self.on_game_over(&mut d);
            }

            self.draw_pop_ups(&mut d);
            self.inc_counter();
        }
    }
}

fn main() {
    let mut snake = Snake::new(WIDTH_CELLS, HEIGHT_CELLS);
    let mut game = Game::new(CELL_SIZE, WIDTH_CELLS, HEIGHT_CELLS, &mut snake);

    let (mut rl, thread) = raylib::init()
        .size(game.width, game.height)
        .title("Snake 0.0.1")
        .build();

    rl.set_target_fps(60);

    let texture = rl
        .load_texture(&thread, "image_50x50.png")
        .expect("failed to load image");

    game.run(&mut rl, &texture, &thread);
}

fn curr_duration_formatted(start_time: &Instant) -> String {
    let duration = Instant::now() - *start_time;
    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
