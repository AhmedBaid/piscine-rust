use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(raduis: f64, x: f64, y: f64) -> Self {
        Self {
            center: Point(x, y),
            radius: raduis,
        }
    }
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }
    pub fn area(&self) -> f32 {
        PI * (self.radius as f32).powi(2)
    }
    pub fn intersect(&self, circle: Circle) -> bool {
        let distance_centers = self.center.distance(circle.center);
        distance_centers < (self.radius + circle.radius)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, point: Point) -> f64 {
        (((self.0 - point.0).powi(2)) + ((self.1 - point.1).powi(2))).sqrt()
    }
}
