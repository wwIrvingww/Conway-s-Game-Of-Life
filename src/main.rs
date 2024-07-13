use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::time::SystemTime;
mod framebuffer;
mod bmp;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Rust Graphics - Render Loop Example",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut x = 1;
    let mut speed = 1;

    while window.is_open() {
        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        // prepare variables for rendering
        if x as usize == framebuffer_width {
            speed = -1;
        }
        if x == 0 {
            speed = 1;
        }
        x += speed;

        // Clear the framebuffer
        framebuffer.set_background_color(0x333355);
        framebuffer.clear();

        // Draw some points
        framebuffer.set_foreground_color(0xFFDDDD);
        framebuffer.point(x as isize, 40);

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer.iter().map(|color| {
                ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
            }).collect::<Vec<u32>>(), framebuffer_width, framebuffer_height)
            .unwrap();

        // Check for screenshot key press
        if window.is_key_down(Key::S) {
            let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
            let filename = format!("screenshot_{}.bmp", timestamp);
            bmp::write_bmp_file(&filename, &framebuffer.buffer, framebuffer_width, framebuffer_height).unwrap();
        }

        // Wait a bit to maintain consistent frame rate
        std::thread::sleep(frame_delay);
    }
}
