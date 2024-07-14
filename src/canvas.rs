use std::collections::HashMap;
use crate::Renderer;
use crate::matrix::Matrix;

pub struct Canvas {
  width: f64,
  height: f64,
  z_index: i32,
  layers: HashMap<i32, Layer>
}

impl Canvas {
  pub fn new(width: f64, height: f64) -> Canvas {
    Canvas {
      width,
      height,
      z_index: 0,
      layers: HashMap::new()
    }
  }

  pub fn get_width(&self) -> f64 {
    self.width
  }

  pub fn get_height(&self) -> f64 {
    self.height
  }

  pub fn get_context(&self) -> Context {
    Context {
      canvas: self,
      state: ContextState {
        transform: Matrix::identity(),
      },
      stack: vec![],
    }
  }

  pub fn empty(&self) -> bool {
    self.layers.is_empty()
  }

  pub fn reset(&mut self) {
    self.layers.clear();
  }

  pub fn set_z_index(&mut self, z_index: i32) {
    self.z_index = z_index;
  }

  pub fn render_to(&self, r: &dyn Renderer) {
    let mut keys = self.layers.keys().collect::<Vec<_>>();
    keys.sort();
    for key in keys {
      if let Some(layer) = self.layers.get(key) {
        for path in &layer.paths {
          r.render_path(path);
        }
        for image in &layer.images {
          r.render_image(image);
        }
        for text in &layer.texts {
          r.render_text(text);
        }
      }
    }
  }
}

impl Renderer for Canvas {
  fn size(&self) -> (f64, f64) {
    (self.width, self.height)
  }

  fn render_path(&self, path: &str) {
    println!("render_path({})", path);
  }

  fn render_image(&self, path: &str) {
    println!("render_image({})", path);
  }

  fn render_text(&self, text: &str) {
    println!("render_text({})", text);
  }
}

pub struct Context<'a> {
  canvas: &'a Canvas,
  state: ContextState,
  stack: Vec<ContextState>
}

#[derive(Clone, Copy)]
pub struct ContextState {
  transform: Matrix
}

impl<'a> Context<'a> {
  pub fn size(&self) -> (f64, f64) {
    self.canvas.size()
  }
  pub fn save(&mut self) {
    self.stack.push(self.state);
    self.state = ContextState {
      transform: Matrix::identity(),
    };
  }
  pub fn restore(&mut self) {
    if let Some(s) = self.stack.pop() {
      self.state = s;
    }
  }
}

struct Layer {
  paths: Vec<String>,
  images: Vec<String>,
  texts: Vec<String>
}