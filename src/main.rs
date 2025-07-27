use macroquad::math::{Circle, Rect};
use macroquad::prelude::*;
#[macroquad::main("MyGame")]
async fn main() {
    let lbound = 100.0;
    let rbound = 600.0;
    let top = 100.0;
    let bottom = 400.0;
    let mut lrect = Rect::new(lbound, top, 1.0, 100.0);
    let mut rrect = Rect::new(rbound, top, 1.0, 100.0);
    let mut ball = Circle::new((lbound + rbound) / 2.0, top, 12.0);
    let mut direction = (1.0, 1.0);
    let mut ball_speed = 1.0;
    let out = loop {
        clear_background(BLACK);
        ball.x -= direction.0 * ball_speed;
        ball.y -= direction.1 * ball_speed;
        ball_speed += 0.002;
        if ball.y < top {
            direction.1 = -1.0;
        }
        if ball.y > bottom {
            direction.1 = 1.0;
        }
        if ball.x < lbound {
            break "right wins";
        }
        if ball.x > rbound {
            break "left wins";
        }
        if is_key_down(KeyCode::W) && lrect.y > top {
            lrect.y -= 4.0;
        }
        if is_key_down(KeyCode::S) && lrect.y < bottom -lrect.h {
            lrect.y += 4.0;
        }
        if is_key_down(KeyCode::Up) && rrect.y > top {
            rrect.y -= 4.0;
        }
        if is_key_down(KeyCode::Down) && rrect.y < bottom -lrect.h{
            rrect.y += 4.0;
        }
        if ball.overlaps_rect(&lrect) {
            direction.0 = -1.0;
        }
        if ball.overlaps_rect(&rrect) {
            direction.0 = 1.0;
        }
        draw_line(lbound, lrect.y, lbound, lrect.y + 100.0, 5.0, WHITE);
        draw_line(rbound, rrect.y, rbound, rrect.y + 100.0, 5.0, WHITE);
        draw_circle(ball.x, ball.y, ball.r, WHITE);
        draw_text("Hello, Github!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    };
    println!("{out}");
}
