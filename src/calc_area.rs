struct Triangle {
  high: f64,
  bottom: f64,
}

struct Square {
  length: f64,
}

struct Round {
  radius: f64,
}

trait CalculateArea {
  fn calculate_area(&self) -> f64;
}

impl CalculateArea for Square {
  fn calculate_area(&self) -> f64 {
    (*self).length * (*self).length
  }
}

impl CalculateArea for Round {
  fn calculate_area(&self) -> f64 {
    (*self).radius * (*self).radius * std::f64::consts::PI
  }
}

impl CalculateArea for Triangle {
  fn calculate_area(&self) -> f64 {
    (*self).bottom * (*self).high / 2.0f64
  }
}

fn main() {
  let square = Square { length: 5.0 };
  print!("area: {}", square.calculate_area());
}