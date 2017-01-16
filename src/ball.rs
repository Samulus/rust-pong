/*
 * ball.rs
 * Author: Samuel Vargas
 *
 * Logic for moving the ball
 */

use entity::Box;

const BALL_SPEED: i32 = 5;

pub fn move_ball(ball : &mut Box) {

    /* if the ball is going to collide
     * with the ceiling or the floor
     * then bounce it appropriately
     */

    ball.x += BALL_SPEED;

    if ball.x > 800 {
        ball.x = 400;
    }
    

}
