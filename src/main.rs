use nalgebra::Vector2;

fn main() {
    let origin = Vector2::new(0.0f32, 0.0);
    let direction = Vector2::new(1.0, 0.5).normalize();

    let step = 0.5;
    let steps = 20;

    let mut position = origin;

    for i in 0..steps {
        println!("step {:02}: ({:.2}, {:.2})", i, position.x, position.y);

        position += direction * step;
    }
}
