use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::time::Duration;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;

    let window = video
        .window(
            "SDL2 + pixels-style sim (RGBA)",
            WIDTH as u32,
            HEIGHT as u32,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, WIDTH as u32, HEIGHT as u32)
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl.event_pump()?;

    let mut buffer = vec![0u8; WIDTH * HEIGHT * 4];

    let mut x: i32 = 100;
    let mut y: i32 = 100;
    let mut vx: i32 = 2;
    let mut vy: i32 = 2;

    println!("Running. Press Escape or close the window to quit.");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        x += vx;
        y += vy;

        if x <= 0 || x >= WIDTH as i32 - 1 {
            vx = -vx;
        }
        if y <= 0 || y >= HEIGHT as i32 - 1 {
            vy = -vy;
        }

        for chunk in buffer.chunks_exact_mut(4) {
            chunk[0] = 0; // R
            chunk[1] = 0; // G
            chunk[2] = 0; // B
            chunk[3] = 255; // A
        }

        if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
            let idx = (y as usize * WIDTH + x as usize) * 4;
            buffer[idx + 0] = 255; // R
            buffer[idx + 1] = 255; // G
            buffer[idx + 2] = 255; // B
            buffer[idx + 3] = 255; // A
        }

        texture.with_lock(None, |tex_buf: &mut [u8], pitch: usize| {
            for row in 0..HEIGHT {
                let src_start = row * WIDTH * 4;
                let dst_start = row * pitch;
                let src_slice = &buffer[src_start..src_start + WIDTH * 4];
                let dst_slice = &mut tex_buf[dst_start..dst_start + WIDTH * 4];
                dst_slice.copy_from_slice(src_slice);
            }
        })?;

        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
