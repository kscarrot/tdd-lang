use std::f64::consts::PI;

trait HasArea {
    fn area(&self) -> f64;
}

struct Rectangle {
    height: f64,
    weight: f64,
}

struct Circle {
    radius: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.weight
    }
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        PI * &self.radius * &self.radius
    }
}

impl HasArea for Triangle {
    fn area(&self) -> f64 {
        0.5 * &self.height * self.base
    }
}

fn get_area<T: HasArea>(shape: T) -> f64 {
    return shape.area();
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::shapes::{Circle, get_area, Rectangle, Triangle};

    #[test]
    fn test_get_area() {
        assert_eq!(get_area(Rectangle { height: 12.0, weight: 6.0 }), 72.0);
        assert_eq!(get_area(Circle { radius: 10.0 }), PI * 100.0);
        assert_eq!(get_area(Triangle { base: 12.0, height: 6.0 }), 36.0);
    }
}