use macroquad::prelude::*;

#[macroquad::main("Physics")]
async fn main() {
    loop {
        clear_background(RED);

        draw_line(
            0.0,
            screen_height() - 20.0,
            screen_width(),
            screen_height() - 20.0,
            15.0,
            BLUE,
        );

        next_frame().await
    }
}
