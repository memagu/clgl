use crate::canvas::Canvas;

/// Sets all pixels of a Canvas object to the chosen brightness value.
pub fn fill(canvas: &mut Canvas, brightness: f64) {
    canvas.map_pixels(|_| brightness);
}

/// Draws a rectangle with specified top-left corner, width and height on a Canvas object.
pub fn rectangle(
    canvas: &mut Canvas,
    corner_x: f64,
    corner_y: f64,
    width: f64,
    height: f64,
    brightness: f64,
) {
    for dx in 0..width as usize {
        for dy in 0..height as usize {
            let x: f64 = corner_x + dx as f64;
            let y: f64 = corner_y + dy as f64;
            if canvas.in_canvas(x, y) {
                canvas.set_pixel(x, y, brightness);
            }
        }
    }
}

/// Draws a circle with its center at (center_x, center_y) and radius r on a Canvas object.
pub fn circle(canvas: &mut Canvas, center_x: f64, center_y: f64, r: f64, brightness: f64) {
    let bb_x_min = (center_x - r).max(0.0) as usize;
    let bb_x_max = (center_x + r).min(canvas.width() as f64) as usize;
    let bb_y_min = (center_y - r).max(0.0) as usize;
    let bb_y_max = (center_y + r).min(canvas.height() as f64) as usize;

    for x in bb_x_min..bb_x_max {
        for y in bb_y_min..bb_y_max {
            let x: f64 = x as f64;
            let y: f64 = y as f64;
            let dx: f64 = x - center_x;
            let dy: f64 = y - center_y;
            if dx.powi(2) + dy.powi(2) <= r.powi(2) {
                canvas.set_pixel(x, y, brightness)
            }
        }
    }
}

/// Draws a line segment from (x1, y1) to (x2, y2) on a Canvas object.
pub fn line_segment(canvas: &mut Canvas, x1: f64, y1: f64, x2: f64, y2: f64, brightness: f64) {
    let dx: f64 = x2 - x1;
    let dy: f64 = y2 - y1;

    let steps: usize = dx.abs().max(dy.abs()) as usize;

    for i in 0..steps {
        let scale_factor: f64 = i as f64 / steps as f64;
        let x: f64 = x1 + scale_factor * dx;
        let y: f64 = y1 + scale_factor * dy;

        if canvas.in_canvas(x, y) {
            canvas.set_pixel(x, y, brightness);
        }
    }
}

/// Draws a line through (x1, y1) and (x2, y2) on a Canvas object.
pub fn line(canvas: &mut Canvas, x1: f64, y1: f64, x2: f64, y2: f64, brightness: f64) {
    let dx: f64 = x2 - x1;
    let dy: f64 = y2 - y1;

    if dx == 0.0f64 {
        if !canvas.in_canvas(x1, 0.0f64) {
            return;
        }

        for y in 0..canvas.height() {
            canvas.set_pixel(x1, y as f64, brightness);
        }
    }

    if dy == 0.0f64 {
        if !canvas.in_canvas(0.0f64, y1) {
            return;
        }

        for x in 0..canvas.width() {
            canvas.set_pixel(x as f64, y1, brightness);
        }
    }

    let k: f64 = dy / dx;
    let m: f64 = y1 - k * x1;

    if k.abs() <= 1.0f64 {
        for x in 0..canvas.width() {
            let x: f64 = x as f64;
            let y: f64 = k * x + m;

            if canvas.in_canvas(x, y) {
                canvas.set_pixel(x, y, brightness);
            }
        }
        return;
    }

    for y in 0..canvas.height() {
        let y: f64 = y as f64;
        let x: f64 = (y - m) / k;

        if canvas.in_canvas(x, y) {
            canvas.set_pixel(x, y, brightness);
        }
    }
}
