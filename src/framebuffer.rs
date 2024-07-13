#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: i32, g: i32, b: i32) -> Color {
        Color {
            r: Color::clamp(r),
            g: Color::clamp(g),
            b: Color::clamp(b),
        }
    }

    fn clamp(value: i32) -> u8 {
        if value < 0 {
            0
        } else if value > 255 {
            255
        } else {
            value as u8
        }
    }
}

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
    background_color: Color,
    foreground_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let background_color = Color::new(0, 0, 0);
        let buffer = vec![background_color; width * height];
        Framebuffer {
            width,
            height,
            buffer,
            background_color,
            foreground_color: Color::new(255, 255, 255),
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (y as usize) * self.width + (x as usize);
            self.buffer[index] = self.foreground_color;
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = Color::new(
            ((color >> 16) & 0xFF) as i32,
            ((color >> 8) & 0xFF) as i32,
            (color & 0xFF) as i32,
        );
    }

    pub fn set_foreground_color(&mut self, color: u32) {
        self.foreground_color = Color::new(
            ((color >> 16) & 0xFF) as i32,
            ((color >> 8) & 0xFF) as i32,
            (color & 0xFF) as i32,
        );
    }
}
