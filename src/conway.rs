pub const WIDTH: usize = 100;
pub const HEIGHT: usize = 100;

// Crear el patrón Pulsar (period 3)
pub fn initialize_pulsar(board: &mut Vec<Vec<bool>>) {
    let offsets = [
        (2, 4), (2, 5), (2, 6), (2, 10), (2, 11), (2, 12),
        (7, 4), (7, 5), (7, 6), (7, 10), (7, 11), (7, 12),
        (9, 4), (9, 5), (9, 6), (9, 10), (9, 11), (9, 12),
        (14, 4), (14, 5), (14, 6), (14, 10), (14, 11), (14, 12),
        (4, 2), (5, 2), (6, 2), (10, 2), (11, 2), (12, 2),
        (4, 7), (5, 7), (6, 7), (10, 7), (11, 7), (12, 7),
        (4, 9), (5, 9), (6, 9), (10, 9), (11, 9), (12, 9),
        (4, 14), (5, 14), (6, 14), (10, 14), (11, 14), (12, 14),
    ];

    let positions = [(10, 10), (50, 10), (10, 50), (50, 50), (30, 30), (70, 30)];

    for &(offset_x, offset_y) in &positions {
        for &(dx, dy) in offsets.iter() {
            board[offset_x + dx][offset_y + dy] = true;
            board[offset_x + dy][offset_y + dx] = true;
        }
    }
}

// Crear el patrón Oscillator
pub fn initialize_oscillator(board: &mut Vec<Vec<bool>>) {
    let offsets = [
        (0, 1), (0, 2), (0, 3), (1, 2), (2, 2), (3, 2), (4, 1), (4, 2), (4, 3),
    ];

    let positions = [(60, 10), (20, 30), (30, 60), (70, 40), (40, 50), (80, 20)];

    for &(offset_x, offset_y) in &positions {
        for &(dx, dy) in offsets.iter() {
            board[offset_x + dx][offset_y + dy] = true;
        }
    }
}

// Crear el patrón Pinwheel
pub fn initialize_pinwheel(board: &mut Vec<Vec<bool>>) {
    let offsets = [
        (0, 1), (0, 2), (0, 3), (1, 0), (1, 4), (2, 0), (2, 4), (3, 0), (3, 4), (4, 1), (4, 2), (4, 3),
        (6, 1), (6, 2), (6, 3), (7, 0), (7, 4), (8, 0), (8, 4), (9, 0), (9, 4), (10, 1), (10, 2), (10, 3),
        (12, 1), (12, 2), (12, 3), (13, 0), (13, 4), (14, 0), (14, 4), (15, 0), (15, 4), (16, 1), (16, 2), (16, 3),
    ];

    let positions = [(30, 30), (70, 70), (10, 70), (50, 30), (20, 20), (60, 60)];

    for &(offset_x, offset_y) in &positions {
        for &(dx, dy) in offsets.iter() {
            board[offset_x + dx][offset_y + dy] = true;
        }
    }
}

// Crear el patrón Angel
pub fn initialize_angel(board: &mut Vec<Vec<bool>>) {
    let offsets = [
        (1, 0), (2, 0), (3, 0), (0, 1), (2, 1), (4, 1), (1, 2), (3, 2), (2, 3), (2, 4),
    ];

    let positions = [(50, 50), (10, 70), (30, 10), (70, 30), (40, 40), (80, 80)];

    for &(offset_x, offset_y) in &positions {
        for &(dx, dy) in offsets.iter() {
            board[offset_x + dx][offset_y + dy] = true;
        }
    }
}

// Crear el patrón 1
pub fn initialize_pattern1(board: &mut Vec<Vec<bool>>) {
    let offsets = [
        (1, 0), (2, 1), (0, 2), (1, 2), (2, 2), (3, 2), (2, 3), (4, 4), (5, 5),
    ];

    let positions = [(70, 70), (20, 20), (10, 60), (60, 10), (30, 30), (80, 80)];

    for &(offset_x, offset_y) in &positions {
        for &(dx, dy) in offsets.iter() {
            board[offset_x + dx][offset_y + dy] = true;
        }
    }
}

// Crear el patrón 2
pub fn initialize_pattern2(board: &mut Vec<Vec<bool>>) {
    let offsets = [
        (1, 0), (2, 0), (3, 0), (4, 1), (5, 2), (6, 2), (5, 3), (4, 4), (3, 4), (2, 4), (1, 4), (0, 3), (0, 2), (0, 1),
        (1, 5), (2, 6), (3, 6), (4, 5), (5, 4), (6, 3), (7, 2), (6, 1), (5, 0), (4, 0), (3, 0), (2, 0), (1, 0),
    ];

    let positions = [(20, 20), (80, 80), (30, 60), (70, 30), (50, 50), (90, 90)];

    for &(offset_x, offset_y) in &positions {
        for &(dx, dy) in offsets.iter() {
            board[offset_x + dx][offset_y + dy] = true;
        }
    }
}

// Crear el patrón 3
pub fn initialize_pattern3(board: &mut Vec<Vec<bool>>) {
    let offsets = [
        (1, 0), (2, 0), (3, 0), (4, 1), (5, 2), (6, 2), (5, 3), (4, 4), (3, 4), (2, 4), (1, 4), (0, 3), (0, 2), (0, 1),
        (2, 6), (3, 6), (4, 5), (5, 4), (6, 3), (7, 2), (6, 1), (5, 0), (4, 0), (3, 0), (2, 0), (1, 0),
    ];

    let positions = [(40, 40), (10, 10), (70, 10), (10, 70), (50, 50), (90, 90)];

    for &(offset_x, offset_y) in &positions {
        for &(dx, dy) in offsets.iter() {
            board[offset_x + dx][offset_y + dy] = true;
        }
    }
}

// Crear el patrón 4
pub fn initialize_pattern4(board: &mut Vec<Vec<bool>>) {
    let offsets = [
        (2, 1), (2, 2), (2, 3), (3, 2), (4, 1), (4, 2), (4, 3), (5, 4), (5, 5), (5, 6), (6, 5), (7, 4), (7, 5), (7, 6),
        (8, 7), (8, 8), (8, 9), (9, 8), (10, 7), (10, 8), (10, 9), (11, 10), (11, 11), (11, 12), (12, 11), (13, 10),
        (13, 11), (13, 12), (14, 13), (14, 14), (14, 15), (15, 14), (16, 13), (16, 14), (16, 15), (17, 16), (17, 17),
        (17, 18), (18, 17), (19, 16), (19, 17), (19, 18),
    ];

    let positions = [(10, 60), (60, 10), (10, 10), (60, 60), (30, 30), (80, 80)];

    for &(offset_x, offset_y) in &positions {
        for &(dx, dy) in offsets.iter() {
            board[offset_x + dx][offset_y + dy] = true;
        }
    }
}

// Crear el patrón Beehive
pub fn initialize_beehive(board: &mut Vec<Vec<bool>>) {
    let offsets = [
        (1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2),
    ];

    let positions = [(5, 5), (45, 45), (25, 25), (65, 65), (15, 15), (55, 55)];

    for &(offset_x, offset_y) in &positions {
        for &(dx, dy) in offsets.iter() {
            board[offset_x + dx][offset_y + dy] = true;
        }
    }
}

// Crear el patrón Loaf
pub fn initialize_loaf(board: &mut Vec<Vec<bool>>) {
    let offsets = [
        (1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3),
    ];

    let positions = [(10, 10), (50, 50), (70, 70), (30, 30), (20, 20), (60, 60)];

    for &(offset_x, offset_y) in &positions {
        for &(dx, dy) in offsets.iter() {
            board[offset_x + dx][offset_y + dy] = true;
        }
    }
}

// Crear el patrón Boat
pub fn initialize_boat(board: &mut Vec<Vec<bool>>) {
    let offsets = [
        (0, 0), (1, 0), (0, 1), (2, 1), (1, 2),
    ];

    let positions = [(15, 15), (55, 55), (35, 35), (75, 75), (25, 25), (65, 65)];

    for &(offset_x, offset_y) in &positions {
        for &(dx, dy) in offsets.iter() {
            board[offset_x + dx][offset_y + dy] = true;
        }
    }
}

// Crear el patrón Blinker
pub fn initialize_blinker(board: &mut Vec<Vec<bool>>) {
    let offsets = [
        (0, 1), (1, 1), (2, 1),
    ];

    let positions = [(20, 20), (60, 60), (80, 80), (40, 40), (10, 10), (70, 70)];

    for &(offset_x, offset_y) in &positions {
        for &(dx, dy) in offsets.iter() {
            board[offset_x + dx][offset_y + dy] = true;
        }
    }
}

// Crear el patrón Toad
pub fn initialize_toad(board: &mut Vec<Vec<bool>>) {
    let offsets = [
        (1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1),
    ];

    let positions = [(25, 25), (65, 65), (35, 35), (75, 75), (45, 45), (85, 85)];

    for &(offset_x, offset_y) in &positions {
        for &(dx, dy) in offsets.iter() {
            board[offset_x + dx][offset_y + dy] = true;
        }
    }
}

// Crear el patrón Beacon
pub fn initialize_beacon(board: &mut Vec<Vec<bool>>) {
    let offsets = [
        (0, 0), (1, 0), (0, 1), (2, 3), (3, 2), (3, 3),
    ];

    let positions = [(30, 30), (70, 70), (40, 40), (80, 80), (20, 20), (60, 60)];

    for &(offset_x, offset_y) in &positions {
        for &(dx, dy) in offsets.iter() {
            board[offset_x + dx][offset_y + dy] = true;
        }
    }
}

// Función para contar vecinos vivos
pub fn count_alive_neighbors(board: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let nx = (x as isize + i).rem_euclid(WIDTH as isize) as usize;
            let ny = (y as isize + j).rem_euclid(HEIGHT as isize) as usize;
            if board[nx][ny] {
                count += 1;
            }
        }
    }
    count
}

// Función render dedicada
pub fn render(framebuffer: &mut crate::framebuffer::Framebuffer, board: &Vec<Vec<bool>>) {
    // No limpiar el framebuffer
    framebuffer.set_background_color(0x5A639C);
    framebuffer.clear();

    // Dibujar las células vivas
    framebuffer.set_foreground_color(0xE2BBE9);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if board[x][y] {
                framebuffer.point(x as isize, y as isize);
            }
        }
    }
}

// Función para actualizar el tablero
pub fn update_board(board: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_board = board.clone();

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let alive_neighbors = count_alive_neighbors(board, x, y);
            if board[x][y] {
                if alive_neighbors < 2 || alive_neighbors > 3 {
                    new_board[x][y] = false;
                }
            } else {
                if alive_neighbors == 3 {
                    new_board[x][y] = true;
                }
            }
        }
    }

    new_board
}
