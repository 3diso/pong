use macroquad::math::{Circle, Rect};
use macroquad::prelude::*;
#[macroquad::main("MyGame")]
async fn main() {
    let lbound = 100.0;
    let rbound = 600.0;
    let top = 100.0;
    let bottom = 400.0;
    let mut score = (0,0);
    let mut lrect = Rect::new(lbound, top, 1.0, 100.0);
    let mut rrect = Rect::new(rbound, top, 1.0, 100.0);
    let mut ball = Circle::new((lbound + rbound) / 2.0, top, 12.0);
    let mut direction = (1.0, 1.0);
    let mut ball_speed = 1.0;
    loop {
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
            ball.move_to(Vec2 { x: (lbound+rbound)/2.0, y: top });
            score.1 +=1;
            ball_speed = 1.0;
        }
        if ball.x > rbound {
            ball.move_to(Vec2 { x: (lbound+rbound)/2.0, y: top });
            score.0 +=1;
            ball_speed = 1.0;
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
        draw_line(lbound, top-12.0, rbound, top-12.0, 5.0, WHITE);
        draw_line(lbound, bottom+12.0, rbound, bottom+12.0, 5.0, WHITE);
        draw_circle(ball.x, ball.y, ball.r, WHITE);
        draw_text(&format!("{}  {}",score.0,score.1), 250.0, 300.0, 100.0, DARKGRAY);

        next_frame().await
    }
}
