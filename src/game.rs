extern crate sdl2;  

use entity::Box;
use entity::BoxType;
use ball::move_ball;
use ai::follow_paddle;

pub struct State {
    player: Box,
    ai: Box,
    ball: Box,
}

impl State {

    pub fn new() -> State {
        return State {
            player: Box::new(50,10, BoxType::Paddle),
            ai: Box::new(750, 10, BoxType::Paddle),
            ball: Box::new(400,300, BoxType::Ball),
        };
    }

    pub fn input(&mut self, key: sdl2::keyboard::Keycode) {
        self.player.input(key);
    }

    pub fn update(&mut self) {
        move_ball(&mut self.ball);
        follow_paddle(&mut self.ai, &mut self.ball);

        if self.ai.check_collsion(&mut self.ball) {
        }

    }

    pub fn render(&mut self, renderer : &mut sdl2::render::Renderer) {
        self.player.render(renderer);
        self.ai.render(renderer);
        self.ball.render(renderer);
    }
}
