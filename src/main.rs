use macroquad::prelude::*;

#[macroquad::main("Basic 3D Ball")]
async fn main() {
    let mut ball_pos = vec3(0.0, 0.0, 0.0);
    let radius = 1.0;

    loop {
        clear_background(WHITE);

        set_camera(&Camera3D {
            position: vec3(5.0, 10.0, 10.0),
            up: vec3(0.0, 1.0, 0.0),
            target: ball_pos,
            ..Default::default()
        });

        draw_sphere(ball_pos, radius, None, RED);

        draw_grid(20, 1.0, BLACK, GRAY);

        set_default_camera();

        draw_text("Basic 3D Ball", 20.0, 40.0, 30.0, BLACK);

        next_frame().await;
    }
}
