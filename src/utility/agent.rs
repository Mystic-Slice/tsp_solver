use std::fmt::Display;

use crate::utility::map::Map;

pub trait Agent: Display {
    fn fitness_score(&self) -> f64;
    fn path(&self) -> Vec<i32>;
    fn distance(&self) -> f64;
    fn new_random(map: &Map) -> Self;
}