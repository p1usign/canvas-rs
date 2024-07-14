use std::ops::Mul;

#[derive(Clone, Copy)]
pub struct Matrix {
  pub a: f64,
  pub b: f64,
  pub c: f64,
  pub d: f64,
  pub e: f64,
  pub f: f64,
}

impl Matrix {
  pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Matrix {
    Matrix { a, b, c, d, e, f }
  }

  pub fn identity() -> Matrix {
    Matrix::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
  }

  pub fn translate(&self, tx: f64, ty: f64) -> Matrix {
    let a = self.a;
    let b = self.b;
    let c = self.c;
    let d = self.d;
    let e = self.e + tx * a + ty * c;
    let f = self.f + tx * b + ty * d;
    Matrix::new(a, b, c, d, e, f)
  }

  pub fn scale(&self, sx: f64, sy: f64) -> Matrix {
    let a = self.a * sx;
    let b = self.b * sx;
    let c = self.c * sy;
    let d = self.d * sy;
    let e = self.e;
    let f = self.f;
    Matrix::new(a, b, c, d, e, f)
  }

  pub fn rotate(&self, angle: f64) -> Matrix {
    let cos = angle.cos();
    let sin = angle.sin();
    let a = self.a * cos + self.c * sin;
    let b = self.b * cos + self.d * sin;
    let c = self.c * cos - self.a * sin;
    let d = self.d * cos - self.b * sin;
    let e = self.e;
    let f = self.f;
    Matrix::new(a, b, c, d, e, f)
  }

  pub fn transform_point(&self, x: f64, y: f64) -> (f64, f64) {
    let new_x = self.a * x + self.c * y + self.e;
    let new_y = self.b * x + self.d * y + self.f;
    (new_x, new_y)
  }
}

impl Mul for Matrix {
  type Output = Matrix;

  fn mul(self, other: Matrix) -> Matrix {
    let a = self.a * other.a + self.c * other.b;
    let b = self.b * other.a + self.d * other.b;
    let c = self.a * other.c + self.c * other.d;
    let d = self.b * other.c + self.d * other.d;
    let e = self.a * other.e + self.c * other.f + self.e;
    let f = self.b * other.e + self.d * other.f + self.f;
    Matrix::new(a, b, c, d, e, f)
  }
}