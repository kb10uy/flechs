/*
mod bounce;
mod move_balls;
mod paddle;
mod winner;

pub use self::bounce::BounceSystem;
pub use self::move_balls::MoveBallsSystem;
pub use self::paddle::PaddleSystem;
pub use self::winner::WinnerSystem;
*/

use crate::pong::{Ball, Paddle, WALL_BOTTOM, WALL_TOP, PaddleSide};

use bevy::prelude::*;

pub const FIXED_TIME_STEP: f32 = 0.01;
pub const PADDLE_SPEED: f32 = 400.0;

pub fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    for (paddle, mut transform) in &mut query {
        let (up_key, down_key) = paddle.move_keycodes();
        let move_amount = if keyboard_input.pressed(up_key) {
            PADDLE_SPEED * FIXED_TIME_STEP
        } else if keyboard_input.pressed(down_key) {
            -PADDLE_SPEED * FIXED_TIME_STEP
        } else {
            0.0
        };
        transform.translation.y = (transform.translation.y + move_amount)
            .max(WALL_BOTTOM + Paddle::HEIGHT / 2.0)
            .min(WALL_TOP - Paddle::HEIGHT / 2.0);
    }
}

pub fn move_ball(mut ball_query: Query<(&Ball, &mut Transform)>) {
    for (ball, mut transform) in &mut ball_query {
        transform.translation.x += ball.velocity.x * FIXED_TIME_STEP;
        transform.translation.y += ball.velocity.y * FIXED_TIME_STEP;
    }
}

pub fn bounce_ball(
    mut ball_query: Query<(&mut Ball, &Transform)>,
    paddle_query: Query<(&Paddle, &Transform)>,
) {
    for (mut ball, ball_transform) in &mut ball_query {
        if ball.velocity.y > 0.0 && ball_transform.translation.y >= WALL_TOP - 16.0 {
            ball.velocity.y = -ball.velocity.y;
        }
        if ball.velocity.y < 0.0 && ball_transform.translation.y <= WALL_BOTTOM + 16.0 {
            ball.velocity.y = -ball.velocity.y;
        }

        for (paddle, transform) in &paddle_query {
            if point_in_rect(
                ball_transform.translation.x,
                ball_transform.translation.y,
                transform.translation.x - 16.0,
                transform.translation.y - 64.0,
                transform.translation.x + 16.0,
                transform.translation.y + 64.0,
            ) {
                if (paddle.0 == PaddleSide::Left && ball.velocity.x < 0.0)
                    || (paddle.0 == PaddleSide::Right && ball.velocity.x > 0.0)
                {
                    ball.velocity.x = -ball.velocity.x;
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
