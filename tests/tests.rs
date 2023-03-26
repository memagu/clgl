#[cfg(test)]
mod lib_test {
    use clgl::canvas;
    use clgl::draw;
    use clgl::tools;

    const CHARSET: &[u8] =
        b" .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

    #[test]
    fn test_test() {
        assert_eq!(true, true);
    }

    #[test]
    fn test_canvas_render() {
        let c: canvas::Canvas = canvas::Canvas::new(32, 32, CHARSET);
        c.render();
    }

    #[test]
    fn test_draw_fill() {
        let mut c: canvas::Canvas = canvas::Canvas::new(32, 32, CHARSET);
        c.render();
        draw::fill(&mut c, 1.0f64);
        c.render();
    }

    #[test]
    fn test_draw_rectangle() {
        let mut c: canvas::Canvas = canvas::Canvas::new(32, 32, CHARSET);
        draw::rectangle(&mut c, 10.0f64, 10.0f64, 5.0f64, 2.0f64, 1.0f64);
        c.render();
    }

    #[test]
    fn test_draw_rectangles() {
        let mut c: canvas::Canvas = canvas::Canvas::new(32, 32, CHARSET);
        draw::rectangle(&mut c, 10.0f64, 10.0f64, 5.0f64, 2.0f64, 1.0f64);
        draw::rectangle(&mut c, 5.0f64, 5.0f64, 6.0f64, 6.0f64, 0.5f64);
        draw::rectangle(&mut c, 7.0f64, 8.0f64, 3.0f64, 1.0f64, 0.3f64);
        draw::rectangle(&mut c, 0.0f64, 10.0f64, 5.0f64, 2.0f64, 0.1f64);
        c.render();
    }

    #[test]
    fn test_draw_circle() {
        let mut c: canvas::Canvas = canvas::Canvas::new(32, 32, CHARSET);
        draw::circle(&mut c, 10.0f64, 10.0f64, 5.0f64, 1.0f64);
        c.render();
    }

    #[test]
    fn test_draw_circles() {
        let mut c: canvas::Canvas = canvas::Canvas::new(32, 32, CHARSET);
        draw::circle(&mut c, 5.0f64, 5.0f64, 3.0f64, 0.5f64);
        draw::circle(&mut c, 10.0f64, 10.0f64, 5.0f64, 1.0f64);
        draw::circle(&mut c, 10.0f64, 15.0f64, 7.0f64, 0.3f64);
        draw::circle(&mut c, 0.0f64, 13.0f64, 7.0f64, 0.1f64);
        c.render();
    }

    #[test]
    fn test_draw_line_segment() {
        let mut c: canvas::Canvas = canvas::Canvas::new(32, 32, CHARSET);
        draw::line_segment(&mut c, 1.0f64, 1.0f64, 31.0f64, 31.0f64, 1.0f64);
        c.render();
    }

    #[test]
    fn test_draw_line_segments() {
        let mut c: canvas::Canvas = canvas::Canvas::new(32, 32, CHARSET);
        draw::line_segment(&mut c, 1.0f64, 1.0f64, 31.0f64, 31.0f64, 1.0f64);
        draw::line_segment(&mut c, 15.0f64, 22.0f64, 12.0f64, 5.0f64, 0.5f64);
        draw::line_segment(&mut c, 0.0f64, 16.0f64, 0.0f64, 31.0f64, 0.3f64);
        draw::line_segment(&mut c, -10.0f64, 16.0f64, 10.0f64, 31.0f64, 0.1f64);
        c.render();
    }

    #[test]
    fn test_draw_line() {
        let mut c: canvas::Canvas = canvas::Canvas::new(32, 32, CHARSET);
        draw::line(&mut c, 1.0f64, 1.0f64, 31.0f64, 31.0f64, 1.0f64);
        c.render();
    }

    #[test]
    fn test_draw_lines() {
        let mut c: canvas::Canvas = canvas::Canvas::new(32, 32, CHARSET);
        draw::line(&mut c, 1.0f64, 1.0f64, 31.0f64, 31.0f64, 1.0f64);
        draw::line(&mut c, 15.0f64, 22.0f64, 12.0f64, 5.0f64, 0.5f64);
        draw::line(&mut c, 0.0f64, 16.0f64, 0.0f64, 31.0f64, 0.3f64);
        draw::line(&mut c, -10.0f64, 16.0f64, 10.0f64, 31.0f64, 0.1f64);
        c.render();
    }

    #[test]
    fn test_tools_clear_terminal() {
        tools::clear_terminal();
    }

    #[test]
    fn test_tools_reset_cursor_position() {
        tools::reset_cursor_position();
    }
}
