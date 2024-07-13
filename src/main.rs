use minifb::{Window, WindowOptions};
use std::time::Duration;
mod framebuffer;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let close_delay = Duration::from_secs(10);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

    // Dibujar algo en el framebuffer
    framebuffer.set_background_color(0x333355);
    framebuffer.clear();

    // Ejemplo de dibujo: dibujar una línea diagonal
    framebuffer.set_foreground_color(0xFFDDDD);
    for i in 0..framebuffer_width.min(framebuffer_height) {
        framebuffer.point(i as isize, i as isize);
    }

    let mut window = Window::new(
        "Rust Graphics - Framebuffer Example",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // Actualizar la ventana con el contenido del framebuffer
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window
            .update_with_buffer(&framebuffer.buffer.iter().map(|color| {
                ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
            }).collect::<Vec<u32>>(), framebuffer_width, framebuffer_height)
            .unwrap();
        
        // Esperar un poco antes de la siguiente actualización
        std::thread::sleep(Duration::from_millis(16));
    }
}
