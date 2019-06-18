use amethyst::{
    core::{transform::Transform, Float},
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::pong::{Ball, Paddle, Side, ARENA_HEIGHT};

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, paddles, transforms): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            if ball_y <= ball.radius.into() && ball.velocity[1] < 0.0 {
                ball.velocity[1] = -ball.velocity[1];
            } else if ball_y >= (ARENA_HEIGHT - ball.radius).into() && ball.velocity[1] > 0.0 {
                ball.velocity[1] = -ball.velocity[1];
            }

            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x - (paddle.width * 0.5).into();
                let paddle_y = paddle_transform.translation().y - (paddle.height * 0.5).into();

                if point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - ball.radius.into(),
                    paddle_y - ball.radius.into(),
                    paddle_x + (paddle.width + ball.radius).into(),
                    paddle_y + (paddle.height + ball.radius).into(),
                ) {
                    if paddle.side == Side::Left && ball.velocity[0] < 0.0 {
                        ball.velocity[0] = -ball.velocity[0];
                    } else if paddle.side == Side::Right && ball.velocity[1] > 0.0 {
                        ball.velocity[0] = -ball.velocity[0];
                    }
                }
            }
        }
    }
}

fn point_in_rect(x: Float, y: Float, left: Float, bottom: Float, right: Float, top: Float) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
