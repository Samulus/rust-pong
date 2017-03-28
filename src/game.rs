/*
 * game.rs
 * Author: Samuel Vargas
 * Date: 03/27/2017
 */

extern crate sdl2;  
extern crate sdl2_sys;

use ai::*;
use ball::*;
use paddle::*;
use world::*;

use sdl2::keyboard::Keycode;
use std::collections::HashMap;

pub struct State {
    player: Paddle,
    ai: Paddle,
    ball: Ball,
    keyboard: HashMap<Keycode, bool>,
}

impl State {

    pub fn new() -> State {
        return State {

            player: Paddle::new(PADDLE_P1_X, PADDLE_P1_Y, 
                                PADDLE_WIDTH, PADDLE_HEIGHT, 
                                PADDLE_SPEED),

                ai: Paddle::new(PADDLE_AI_X, PADDLE_AI_Y, 
                                PADDLE_WIDTH, PADDLE_HEIGHT, 
                                PADDLE_SPEED),

              ball: Ball::new(BALL_X, BALL_Y, 
                             BALL_WIDTH, BALL_HEIGHT,
                             BALL_SPEED),

            keyboard: HashMap::new(),
        };
    }

    pub fn input(&mut self, key: Keycode, press: bool) {
        let mut missing_key = false;
        match self.keyboard.get_mut(&key) {
            Some(v) => *v = press,
            None    => missing_key = true,
        }

        if missing_key {
            self.keyboard.insert(key, press);
        }
    }

    pub fn update(&mut self) {

        self.ball.move_it();

        match self.ball.out_of_bounds() {
            Some(OutOfBoundsFault::Player) => {
                self.ball.reset();
            },

            Some(OutOfBoundsFault::Ai) => {
                self.ball.reset();
            },

            _ => {}
        }

        /* get the ai paddle to follow the ball */
        follow_paddle(&mut self.ai, &self.ball);

        /* move player */
        match self.keyboard.get(&Keycode::Down) {
            Some(p) => if *p { self.player.down(); },
            None => {},
        }

        match self.keyboard.get(&Keycode::Up) {
            Some(p) => if *p { self.player.up(); },
            None => {},
        }

        if self.ball.check_collsion(&self.player) {
            self.ball.direction = Direction::Right;
        }

        if self.ball.check_collsion(&self.ai) {
            self.ball.direction = Direction::Left;
        }
    }

    pub fn render(&mut self, renderer : &mut sdl2::render::Renderer) {
        self.player.render(renderer);
        self.ai.render(renderer);
        self.ball.render(renderer);
    }
}
