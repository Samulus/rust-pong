/*
 * ai.rs
 * Author: Samuel Vargas
 * Date: 03/27/2017
 */

use ball::*;
use paddle::*;

pub fn follow_paddle(paddle: &mut Paddle, ball: &Ball) {

    let dt = ball.dimen.y - (paddle.dimen.y + (paddle.dimen.h / 2));

    /* avoid doing little micromovements when the paddle is close enough */
    if dt.abs() < 5 {
        return;
    }

    if paddle.dimen.y < ball.dimen.y - paddle.dimen.h / 2 {
        paddle.dimen.y += 4;
    }

    else if paddle.dimen.y > ball.dimen.y  - paddle.dimen.h / 2{
        paddle.dimen.y -= 4;
    }
}

