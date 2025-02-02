use std::process::exit;
use std::time::Instant;
use rand::prelude::ThreadRng;
use rand::Rng;
use raylib::color::Color;
use raylib::consts::KeyboardKey::{KEY_C, KEY_DOWN, KEY_ENTER, KEY_LEFT, KEY_LEFT_CONTROL, KEY_RIGHT, KEY_SPACE, KEY_UP};
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::{RaylibTexture2D, Texture2D};
use raylib::{RaylibHandle, RaylibThread};
use crate::snake::{Direction, Pt, Snake};

const COLOR_LIGHT_LIGHT_GRAY: Color = Color::new(0, 0, 0, 16);

pub struct Game<'a> {
    cell_size: i32,
    width_cells: i32,
    height_cells: i32,
    top_margin: i32,
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
    pub fn new(
        cell_size: i32,
        width_cells: i32,
        height_cells: i32,
        top_margin: i32,
        snake: &'a mut Snake,
    ) -> Game<'a> {
        let width = cell_size * width_cells;
        let height = cell_size * height_cells + top_margin;

        Game {
            cell_size,
            width_cells,
            height_cells,
            top_margin,
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
    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn on_game_over(
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

    pub fn draw_piggy(&self, d: &mut impl RaylibDraw, texture: &Texture2D) {
        let source_rect = Rectangle::new(
            0.0,
            0.0,
            texture.width() as f32,
            texture.height() as f32,
        );

        let dest_rect = Rectangle::new(
            self.food_pt.0 as f32 * self.cell_size as f32,
            self.food_pt.1 as f32 * self.cell_size as f32 + self.top_margin as f32,
            self.cell_size as f32,
            self.cell_size as f32,
        );

        for i in 0..self.height_cells {
            d.draw_rectangle(
                self.food_pt.0 * self.cell_size,
                i * self.cell_size + self.top_margin,
                self.cell_size,
                self.cell_size,
                COLOR_LIGHT_LIGHT_GRAY,
            );
        }
        for j in 0..self.width_cells {
            d.draw_rectangle(
                j * self.cell_size,
                self.food_pt.1 * self.cell_size + self.top_margin,
                self.cell_size,
                self.cell_size,
                COLOR_LIGHT_LIGHT_GRAY,
            );
        }

        let origin = Vector2::new(0.0, 0.0); // No rotation offset
        d.draw_texture_pro(
            &texture, source_rect, dest_rect, origin, 0.0,
            Color::WHITE,
        );
    }

    pub fn draw_score(
        &self,
        d: &mut impl RaylibDraw,
    ) {
        let text = format!("Score: {}, length of the snake: {}", self.score, self.snake.len());
        d.draw_text(&text, 10, 18, 20, Color::BLACK);
        d.draw_text(curr_duration_formatted(&self.start_time).as_str(),
                    self.width - 60, 18, 20, Color::BLACK);
    }

    pub fn draw_snake(&self, d: &mut impl RaylibDraw) {
        self.snake.draw(|x, y| {
            d.draw_rectangle(
                x * self.cell_size,
                y * self.cell_size + self.top_margin,
                self.cell_size,
                self.cell_size,
                Color::BLACK,
            )
        });
    }

    pub fn draw_lines(&self, d: &mut impl RaylibDraw) {
        let mut y = self.top_margin;

        while y < self.height {
            d.draw_line(0, y, self.width, y, Color::GRAY);
            y += self.cell_size;
        }

        let mut x = self.cell_size;
        while x < self.width {
            d.draw_line(x, self.top_margin, x, self.height, Color::GRAY);
            x += self.cell_size
        }
    }

    pub fn pop_up(&self, d: &mut impl RaylibDraw, text: &str) {
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

    pub fn draw_pop_ups(&mut self, d: &mut impl RaylibDraw) {
        if self.pop_exit {
            self.pop_up(d, "To exit, press Enter, to resume, press Space");
        }

        if self.paused {
            self.pop_up(d, "Press Enter to continue!");
        }
    }

    pub fn inc_counter(&mut self) {
        self.counter += 1;
    }

    pub fn should_make_move(&self) -> bool {
        self.counter % 5 == 0 && !self.paused && !self.pop_exit
    }

    pub fn step(&mut self) {
        if self.should_make_move() {
            let mut growing = false;

            if *self.snake.head() == self.food_pt {
                self.food_pt = Pt(
                    self.rng.random_range(0..self.width_cells),
                    self.rng.random_range(0..self.height_cells),
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

    pub fn run(&mut self, rl: &mut RaylibHandle, texture: &Texture2D, thread: &RaylibThread) {
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

fn curr_duration_formatted(start_time: &Instant) -> String {
    let duration = Instant::now() - *start_time;
    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
