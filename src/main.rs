use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut circle_x = screen_width() / 2.0;
    let mut circle_y = 0.0;
    let speed = 2.0;
    let radius = 18.0;

    loop {
        clear_background(RED);

        draw_line(
            0.0,
            screen_height() - radius,
            screen_width(),
            screen_height() - radius,
            3.0,
            BLACK,
        );
        let ground_y = screen_height() - radius;

        if circle_y + radius >= ground_y {
            circle_y = ground_y - radius;
        } else {
            circle_y += speed;
        }
        draw_circle(circle_x, circle_y, radius, YELLOW);
        if is_key_down(KeyCode::D) {
            circle_x += speed;
        } else if is_key_down(KeyCode::A) {
            circle_x -= speed;
        } else if is_key_down(KeyCode::W) {
            circle_y -= speed * 2.0;
        }
        next_frame().await;
    }
}
