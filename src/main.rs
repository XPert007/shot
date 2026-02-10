use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut circle_x = screen_width() - 30.0;
    let circle_y = screen_height() - 30.0;
    let speed = -2.0;

    loop {
        clear_background(RED);

        circle_x += speed;

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(circle_x, circle_y, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 100.0, 100.0, DARKGRAY);

        next_frame().await;
    }
}

