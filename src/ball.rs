/*
 * ball.rs
 * Author: Samuel Vargas
 * Date: 03/27/2017
 */

extern crate sdl2;  
extern crate rand;

//use rand::Rng;

use world::*;
use sdl2::pixels::Color::*;
use sdl2::rect::*;
use paddle::*;


pub enum Direction {
    Left,
    Right
}

pub enum OutOfBoundsFault {
    Player,
    Ai,
}

pub struct Ball {
    pub dimen: Rect,
    pub direction:  Direction,
    pub speed: u32,
    pub angle: i32,
}

impl Ball {

    pub fn new(x: i32, y: i32, w: u32, h: u32, speed: u32) -> Ball {
        return Ball { 
            dimen: Rect::new(x, y, w, h),
            speed: speed,
            direction: Direction::Right,
            angle: 0,
        };
    }

    pub fn move_it(&mut self) {
        match self.direction {
            Direction::Left =>
                {self.dimen.x -= self.speed as i32;}
            Direction::Right =>
                {self.dimen.x += self.speed as i32;}
        }

        self.dimen.y += self.angle;
    }

    pub fn reset(&mut self) {
        self.dimen.x = BALL_X;
        self.dimen.y = BALL_Y;
    }

    pub fn out_of_bounds(&mut self) -> Option<OutOfBoundsFault> {
        if self.dimen.x > SCREEN_X as i32 {
            return Some(OutOfBoundsFault::Ai);
        }
        if self.dimen.x < 0 {
            return Some(OutOfBoundsFault::Player);
        }

        return None
    }

    pub fn render(&self, renderer: &mut sdl2::render::Renderer) {
        renderer.set_draw_color(RGB(255,255,255));
        let _ = renderer.fill_rect(self.dimen);
        renderer.set_draw_color(RGB(0, 0, 0));
    }

    pub fn check_collsion(&mut self, paddle: &Paddle) -> bool {

        let a_t = self.dimen.y;
        let a_b = self.dimen.y + self.dimen.h;
        let a_l = self.dimen.x;
        let a_r = self.dimen.x + self.dimen.w;

        let b_t = paddle.dimen.y;
        let b_b = paddle.dimen.y + paddle.dimen.y;
        let b_l = paddle.dimen.x;
        let b_r = paddle.dimen.x + paddle.dimen.w;

        let result = a_l < b_r && a_r > b_l &&
                     a_t < b_b && a_b > b_t;

        //let mut rng = rand::thread_rng();
          if result {
              match self.direction {
                  Direction::Left => {
                      //self.angle = rand::thread_rng().gen_range(0, 5);
                      self.angle = -1;
                  },

                  Direction::Right => {
                      //self.angle = -rand::thread_rng().gen_range(0, 5);
                      self.angle = -1;
                  },

              }
          }

        return result;
    }
}
