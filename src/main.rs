use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod framebuffer;
mod bmp;
mod conway;

use framebuffer::Framebuffer;
use conway::{WIDTH, HEIGHT, initialize_pulsar, initialize_oscillator, initialize_pinwheel, initialize_angel, initialize_pattern1, initialize_pattern2, initialize_pattern3, initialize_pattern4, initialize_beehive, initialize_loaf, initialize_boat, initialize_blinker, initialize_toad, initialize_beacon, render, update_board};

fn main() {
    let window_width = WIDTH * 4;
    let window_height = HEIGHT * 4;

    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Irving Acosta 22781 - Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // Inicializar el tablero y los patrones iniciales
    let mut board = vec![vec![false; HEIGHT]; WIDTH];
    initialize_pulsar(&mut board);
    initialize_oscillator(&mut board);
    initialize_pinwheel(&mut board);
    initialize_angel(&mut board);
    initialize_pattern1(&mut board);
    initialize_pattern2(&mut board);
    initialize_pattern3(&mut board);
    initialize_pattern4(&mut board);
    initialize_beehive(&mut board);
    initialize_loaf(&mut board);
    initialize_boat(&mut board);
    initialize_blinker(&mut board);
    initialize_toad(&mut board);
    initialize_beacon(&mut board);

    while window.is_open() {
        // Escuchar entradas del teclado
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Actualizar el tablero según las reglas de Conway
        board = update_board(&board);

        // Llamar a la función render para pintar el framebuffer
        render(&mut framebuffer, &board);

        // Actualizar la ventana con el contenido del framebuffer
        window
            .update_with_buffer(&framebuffer.buffer.iter().map(|color| {
                ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
            }).collect::<Vec<u32>>(), WIDTH, HEIGHT)
            .unwrap();

        // Esperar un poco para mantener una tasa de frames constante
        std::thread::sleep(frame_delay);
    }
}
