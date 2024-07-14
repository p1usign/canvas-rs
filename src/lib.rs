pub trait Renderer {
  fn size(&self) -> (f64, f64);
  fn render_path(&self, path: &str);
  fn render_image(&self, path: &str);
  fn render_text(&self, text: &str);
}

pub mod canvas;
pub mod matrix;