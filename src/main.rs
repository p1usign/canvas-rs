use canvas_rs::canvas::Canvas;

fn main() {
    let c1 = Canvas::new(640.0, 480.0);
    let c2 = Canvas::new(800.0, 600.0);
    c1.render_to(&c2)
}
