use macroquad::math::Rect;
use macroquad::prelude::*;
#[macroquad::main("MyGame")]
async fn main() {
    let lbound = 100.0;
    let rbound = 600.0;
    let top = 100.0;
    let bottom = 600.0;
    let mut lrect = Rect::new(lbound, top, 15.0, 100.0);
    let mut rrect = Rect::new(rbound, top, 15.0, 100.0);

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::W) && lrect.h > top {
            lrect.h -= 4.0;
        }
        if is_key_down(KeyCode::S) && lrect.h < bottom {
            lrect.h += 4.0;
        }
        if is_key_down(KeyCode::Up) && rrect.h > top {
            rrect.h -= 4.0;
        }
        if is_key_down(KeyCode::Down) && rrect.h < bottom {
            rrect.h += 4.0;
        }
        draw_line(lbound, lrect.h, lbound, lrect.h + 100.0, 15.0, WHITE);
        draw_line(rbound, rrect.h, rbound, rrect.h + 100.0, 15.0, WHITE);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
