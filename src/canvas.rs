use std::io::{self, Write};

/// Canvas struct.
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<f64>,
    charset: &'static [u8],
}

impl Canvas {
    /// Creates a new Canvas instance with specified width, height and charset.
    pub fn new(width: usize, height: usize, charset: &'static [u8]) -> Self {
        Self {
            width,
            height,
            pixels: vec![0.0f64; (width * height) as usize],
            charset,
        }
    }

    /// Gets width of Canvas.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Gets height of Canvas.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Gets brightness value of pixel at (x, y).
    pub fn get_pixel(&self, x: f64, y: f64) -> f64 {
        self.pixels[self.coordinate_to_pixel_index(x, y)]
    }

    /// Sets brightness value of pixel at (x, y). The brightness value is clamped between 0 and 1.
    pub fn set_pixel(&mut self, x: f64, y: f64, brightness: f64) {
        let pixel_index: usize = self.coordinate_to_pixel_index(x, y);
        self.pixels[pixel_index] = brightness.clamp(0.0f64, 1.0f64);
    }

    /// Applies a function to all pixels.
    pub fn map_pixels(&mut self, func: impl Fn(f64) -> f64) {
        for pixel_value in self.pixels.iter_mut() {
            *pixel_value = func(*pixel_value);
        }
    }

    /// Resizes canvas.
    pub fn resize(mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.pixels.resize(width * height, 0.0f64);
    }

    /// Renders the canvas to stdout.
    pub fn render(&self) {
        let mut buffer: Vec<u8> = Vec::with_capacity((self.width + 1) * self.height);
        let mut writer: io::Stdout = io::stdout();

        for (i, pixel_value) in self.pixels.iter().enumerate() {
            buffer.push(self.brightness_to_char_as_byte(*pixel_value));
            if (i + 1) % self.width == 0 {
                buffer.push(b'\n');
            }
        }
        writer.write_all(&buffer).unwrap();
        writer.flush().unwrap();
    }

    /// Checks if a coordinate (x, y) is within Canvas bounds.
    pub fn in_canvas(&self, x: f64, y: f64) -> bool {
        0.0f64 <= x && x < self.width as f64 && 0.0f64 <= y && y < self.height as f64
    }

    /// Returns the character corresponding to a specified brightness value as bytes.
    fn brightness_to_char_as_byte(&self, brightness: f64) -> u8 {
        let char_index: usize = (brightness * ((self.charset.len() - 1) as f64)) as usize;
        self.charset[char_index]
    }

    /// Flattens a coordinate (x, y) to a index.
    fn coordinate_to_pixel_index(&self, x: f64, y: f64) -> usize {
        y as usize * self.width + x as usize
    }
}
