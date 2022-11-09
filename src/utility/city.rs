use std::{fmt::Display, ops::Sub};

#[derive(Clone, Copy, Debug)]
pub struct City {
    pub id: i32,
    pub x: f64,
    pub y: f64,
}

impl City {

    pub fn new(id: i32, x: f64, y: f64) -> Self {
        Self { id, x, y }
    }
    
    pub fn distance(self, city: City) -> f64 {
        let x_dis = (self.x - city.x).abs();
        let y_dis = (self.y - city.y).abs();
        let distance = (x_dis * x_dis + y_dis * y_dis).sqrt();
        distance
    }
}

impl Sub for City {
    type Output = f64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.distance(rhs)
    }
}

impl Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "City({}) at x: {}, y: {}", self.id, self.x, self.y)
    }
}
