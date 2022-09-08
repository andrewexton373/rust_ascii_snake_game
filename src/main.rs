use std::collections::VecDeque;

use rand::Rng;
use ruscii::app::{App, State};
use ruscii::terminal::{Window};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::spatial::{Vec2};
use ruscii::gui::{FPSCounter};

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct PlayerState {
    pub snake: VecDeque<Vec2>,
    pub direction: Direction
}

struct GameState {
    pub dimension: Vec2,
    pub player: PlayerState,
    pub food: Vec2,
    pub has_lost: bool
}

impl GameState {
        pub fn new(dim: Vec2) -> Self {
            Self {
                dimension: dim,
                player: PlayerState { snake: VecDeque::from(vec![Vec2::xy(dim.x/2, dim.y/2)]), direction: Direction::Right },
                food: Self::rand_food_position(dim),
                has_lost: false
            }
        }

        pub fn update(&mut self) {
            let front  = *self.player.snake.front().unwrap();

            let updated_front = match self.player.direction {
                Direction::Up => { front + Vec2::xy(0,-1) },
                Direction::Down => { front + Vec2::xy(0,1) },
                Direction::Left => { front + Vec2::xy(-1,0) },
                Direction::Right => { front + Vec2::xy(1,0) }
            };

            // If the snake ran over itself, trigger loss
            if self.player.snake.contains(&updated_front) {
                self.has_lost = true;
            }

            // If the snake hit the boundary, trigger loss
            if (
                updated_front.x < 0 ||
                updated_front.y < 0 ||
                updated_front.x >= self.dimension.x ||
                updated_front.y >= self.dimension.y
             ) {
                self.has_lost = true;
            }

            
            self.player.snake.push_front(updated_front);    

            if updated_front == self.food {
                self.food = Self::rand_food_position(self.dimension);
            } else {
                self.player.snake.pop_back();
            }
      
        }

        fn rand_food_position(dim: Vec2) -> Vec2 {
            let mut rng = rand::thread_rng();
            Vec2::xy(rng.gen_range(1..dim.x - 1), rng.gen_range(1..dim.y - 1))
        }
}

fn main() {
    let mut fps_counter = FPSCounter::new();
    let mut app = App::new();
    let win_size = app.window().size();
    let mut state = GameState::new((win_size * 4) / 5);

    app.run(|app_state: &mut State, window: &mut Window| {

        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                KeyEvent::Pressed(Key::R) => { state = GameState::new((win_size * 4) / 5) },
                _ => (),
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::W => state.player.direction = Direction::Up,
                Key::S => state.player.direction = Direction::Down,
                Key::A => state.player.direction = Direction::Left,
                Key::D => state.player.direction = Direction::Right,
                _ => (),
            }
        }

        fps_counter.update();

        if app_state.step() % 2 == 0 {
            state.update();
        }

        let mut pencil = Pencil::new(window.canvas_mut());

        if state.has_lost {
            let you_lose_msg = &format!("You Lose! Score: {} | Press \"R\" to Restart", state.player.snake.len());
            
            // Draw You Lose Message
            pencil
                .set_origin(Vec2::xy((win_size.x - you_lose_msg.len() as i32) / 2, (win_size.y - state.dimension.y) / 2 - 1))
                .draw_text(you_lose_msg, Vec2::xy(0, 0));
            return;
        }

        let score_msg = &format!("Score: {}", state.player.snake.len());

        // Draw FPS and Score
        pencil
        .draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(0, 0))
        .set_origin(Vec2::xy((win_size.x - score_msg.len() as i32) / 2, (win_size.y - state.dimension.y) / 2 - 1))
        .draw_text(score_msg, Vec2::xy(0, 0));

        // Draw Boundary
        pencil
            .set_origin((win_size - state.dimension) / 2)
            .draw_rect(&RectCharset::simple_round_lines(), Vec2::zero(), state.dimension);

        // Draw food
        pencil
            .set_origin((win_size - state.dimension) / 2)
            .draw_char('o', state.food);


        // Draw Snake
        for snake_link in state.player.snake.iter() {
            pencil.draw_char('▒', *snake_link);
        }
    });
}
