/*
 * paddle.rs
 * Author: Samuel Vargas
 * Date: 03/27/2017
 */

extern crate sdl2;
extern crate rand;

use sdl2::rect::Rect;
use sdl2::pixels::*;
use world::*;

pub struct Paddle {
    pub dimen: Rect,
    pub speed: u32,
}

impl Paddle {
    pub fn new(x: i32, y: i32, w: i32, h: i32, speed: u32) -> Paddle {
        return Paddle {
            dimen: Rect::new(x, y, w as u32, h as u32),
            speed: speed,
        }
    }

    pub fn up(&mut self) {
        if self.dimen.y > 10 {
            self.dimen.y -= self.speed as i32;
        }
    }

    pub fn down(&mut self) {
        if self.dimen.y + self.dimen.h + 10 < SCREEN_Y as i32 - 10 {
            self.dimen.y += self.speed as i32;
        }
    }

    pub fn render(&self, renderer: &mut sdl2::render::Renderer) {
        renderer.set_draw_color(Color::RGB(255,255,255));
        let _ = renderer.fill_rect(self.dimen);
        renderer.set_draw_color(Color::RGB(0, 0, 0));
    }
}
