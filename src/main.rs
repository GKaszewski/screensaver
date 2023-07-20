#![windows_subsystem = "windows"]
use rand::Rng;
use raylib::prelude::*;
use std::time::{ Duration, Instant };

const CELL_SIZE: i32 = 10;

struct GameOfLife {
    width: i32,
    height: i32,
    cells: Vec<bool>,
    next_cells: Vec<bool>,
}

#[allow(dead_code)]
impl GameOfLife {
    fn new(width: i32, height: i32) -> Self {
        let cells = vec![false; (width * height) as usize];
        let next_cells = cells.clone();
        Self {
            width,
            height,
            cells,
            next_cells,
        }
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    fn get_cell(&self, x: i32, y: i32) -> bool {
        self.cells[self.get_index(x, y)]
    }

    fn set_cell(&mut self, x: i32, y: i32, state: bool) {
        let index = self.get_index(x, y);
        self.cells[index] = state;
    }

    fn live_neighbours(&self, x: i32, y: i32) -> i32 {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let x = x + i;
                let y = y + j;
                if x < 0 || x >= self.width || y < 0 || y >= self.height {
                    continue;
                }
                if self.get_cell(x, y) {
                    count += 1;
                }
            }
        }
        count
    }

    fn update(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) as usize;
                let live_neighbours = self.live_neighbours(x, y);
                let cell = self.cells[idx];
                let next_cell = match (cell, live_neighbours) {
                    (true, 2) | (true, 3) => true,
                    (true, _) => false,
                    (false, 3) => true,
                    (false, _) => false,
                };
                self.next_cells[idx] = next_cell;
            }
        }
        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    fn place_lwss(&mut self, x: i32, y: i32) {
        let pattern = [
            (1, 0),
            (2, 0),
            (0, 1),
            (3, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 3),
            (2, 4),
        ];

        for (dx, dy) in pattern.iter() {
            self.set_cell(x + dx, y + dy, true);
        }
    }

    fn randomize(&mut self, percent: f64) {
        let mut rng = rand::thread_rng();
        for cell in self.cells.iter_mut() {
            *cell = rng.gen_bool(percent);
        }
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get_cell(x, y);
                if cell {
                    d.draw_rectangle(
                        x * CELL_SIZE,
                        y * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        Color::WHITE
                    );
                }
            }
        }
    }
}

fn run_game(rl: &mut RaylibHandle, thread: &RaylibThread) {
    rl.set_target_fps(30);

    let mut game = GameOfLife::new(800, 600);
    let mut rng = rand::thread_rng();
    game.randomize(rng.gen());

    let frame_time = Duration::from_secs_f32(1.0 / 30.0);
    let mut last_update = Instant::now();

    let mouse_pos = rl.get_mouse_position();

    while !rl.window_should_close() {
        let new_mouse_pos = rl.get_mouse_position();

        if new_mouse_pos != mouse_pos {
            break;
        }

        if
            rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) ||
            rl.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) ||
            rl.is_mouse_button_pressed(MouseButton::MOUSE_MIDDLE_BUTTON) ||
            rl.get_mouse_wheel_move() != 0.0
        {
            break;
        }

        if last_update.elapsed() >= frame_time {
            game.update();
            last_update = Instant::now();
        }

        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::BLACK);
        game.draw(&mut d);
    }
}

#[cfg(target_os = "windows")]
fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("/s") | None => {
            let (mut rl, thread) = raylib
                ::init()
                .size(1920, 1080)
                .title("Screensaver")
                .undecorated()
                .fullscreen()
                .build();
            run_game(&mut rl, &thread);
        }
        Some("/p") => {
            let (mut rl, thread) = raylib::init().size(1280, 720).title("Screensaver").build();
            run_game(&mut rl, &thread);
        }
        _ => {
        }
    }
}
