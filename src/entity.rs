/*
 * entity.rs
 * Author: Samuel Vargas
 * 
 * Everything is hardcoded to 800x600 because
 * toy project.
 */

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use util::abs;

const PADDLE_HEIGHT: i32 = 80;
const PADDLE_WIDTH: i32 = 15;
const PADDLE_SPEED: i32 = 16;
const BALL_HEIGHT: i32 = 10;
const BALL_WIDTH: i32 = 10;

pub enum BoxType {
    Paddle,
    Ball
}

pub struct Box {
    pub x: i32,
    pub y: i32, 
    pub w: i32,
    pub h: i32,
    pub going_left: bool,
}

impl Box {

    pub fn new(x: i32, y: i32, box_type: BoxType) -> Box {
        let (w, h) = match box_type {
            BoxType::Paddle => (PADDLE_WIDTH, PADDLE_HEIGHT),
            BoxType::Ball   => (BALL_WIDTH, BALL_HEIGHT),
        };
        return Box { x: x, y: y, w: w, h: h, going_left: false};
    }

    pub fn input(&mut self, key: Keycode) {
        if key == Keycode::J || key == Keycode::Down && self.y + PADDLE_HEIGHT < 550 {
            self.y += PADDLE_SPEED;
        }

        if key == Keycode::K || key == Keycode::Up && self.y - PADDLE_SPEED > 0 {
            self.y -= PADDLE_SPEED;
        }
    }

    pub fn render(&mut self, renderer: &mut sdl2::render::Renderer) {
        renderer.set_draw_color(Color::RGB(255,255,255));
        let _ = renderer.fill_rect(Rect::new(self.x, self.y, self.w as u32, self.h as u32));
        renderer.set_draw_color(Color::RGB(0, 0, 0));
    }

    // http://gamedev.stackexchange.com/a/587
    pub fn check_collsion(&mut self, ball : &mut Box) -> bool {
          return abs(self.x - ball.x) * 2 < (self.w + ball.w) &&
                 abs(self.y - ball.y) * 2 < (self.h + ball.h);
    }
}
