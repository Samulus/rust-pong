/*
 * ai.rs
 * Author: Samuel Vargas
 */

use entity::Box;


pub fn follow_paddle(paddle : &mut Box, ball : &mut Box) {
    if paddle.y < ball.y {
        paddle.y += 4;
    }

    else if paddle.y > ball.y {
        paddle.y -= 4;
    }
}

