use macroquad::math::{Circle, Rect};
use macroquad::{prelude::*};
use macroquad::ui::{root_ui,Skin};

async fn main_menu(){
    
    loop{
        clear_background(BLUE);
        let height = screen_height();
        let width = screen_width();
        let middle = Vec2::new(width/2.0,height/2.0);
        let button_1p = Vec2::new(middle.x-10.0,middle.y-10.0);
        let button_2p = Vec2::new(middle.x-10.0,middle.y+10.0);
        let exit = Vec2::new(width-100.0,height-100.0);
        if root_ui().button(button_1p, "One Player Mode") {
            one_player().await;
            break;
        }
        if root_ui().button(button_2p, "Two Player Mode") {
            two_player().await;
            break;
        }
        if root_ui().button(exit, "exit") {
            break;
        }
        next_frame().await
    }
}

async fn play_on(score:(u32,u32)) -> bool{
    let mut cont = true;

    let label_style = root_ui()
        .style_builder()
        .text_color(WHITE)
        .font_size(100)
        .build();
    let ui_skin = Skin{
        label_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);
    loop{
        let height = screen_height();
        let width = screen_width();
        let middle = Vec2::new(width/2.0,height/2.0);

        let score_pos = Vec2::new(middle.x-100.0,middle.y-200.0);
        let button_1p = Vec2::new(middle.x,middle.y-10.0);
        let button_2p = Vec2::new(middle.x,middle.y+30.0);

        root_ui().label(score_pos, &format!("{} vs {}",score.0,score.1));

        if root_ui().button(button_1p, "Continue") {
            break;
        }
        if root_ui().button(button_2p, "End") {
            cont = false;
            break;
        }
        next_frame().await;
    }
    cont
}
async fn one_player(){
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
            let cont  = play_on(score).await;
            if !cont{
                break
            } 
        }
        if ball.x > rbound {
            ball.move_to(Vec2 { x: (lbound+rbound)/2.0, y: top });
            score.0 +=1;
            ball_speed = 1.0;
            let cont  = play_on(score).await;
            if !cont{
                break
            } 
        }
        if is_key_down(KeyCode::W) && lrect.y > top {
            lrect.y -= 4.0;
        }
        if is_key_down(KeyCode::S) && lrect.y < bottom -lrect.h {
            lrect.y += 4.0;
        }
        if ball.y<rrect.y+50.0 && rrect.y > top {
            rrect.y -= 4.0;
        }
        if ball.y>rrect.y+50.0 && rrect.y < bottom -lrect.h{
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
async fn two_player(){
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
            let cont  = play_on(score).await;
            if !cont{
                break
            } 
        }
        if ball.x > rbound {
            ball.move_to(Vec2 { x: (lbound+rbound)/2.0, y: top });
            score.0 +=1;
            ball_speed = 1.0;
            let cont  = play_on(score).await;
            if !cont{
                break
            } 
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
#[macroquad::main("MyGame")]
async fn main() {
    main_menu().await;
}
