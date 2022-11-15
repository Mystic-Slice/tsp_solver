use std::fmt::Display;
use crate::utility::map::Map;

#[derive(Clone, Debug)]
pub struct Agent {
    pub path: Vec<i32>,
    pub distance: f64,
}

impl Agent {
    pub fn new(path: Vec<i32>, distance: f64) -> Self {
        Self { path: path, distance: distance }
    }

    pub fn new_random(map: &Map) -> Self {
        let (path, distance) = map.random_path();
        Self { path: path, distance: distance }
    }

    pub fn fitness_score(&self) -> f64 {
        1.0/self.distance
    }
}

impl Display for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path distance: {}, Path: {:?}", self.distance, self.path)
    }
}