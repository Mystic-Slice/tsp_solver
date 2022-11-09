use std::{fs::File, io::{BufReader, BufRead}, fmt::Display};
use rand::{thread_rng, seq::SliceRandom};
use crate::utility::city::City;

#[derive(Clone, Debug)]
pub struct Map {
    pub cities: Vec<City>,
}

impl Map {

    pub fn new() -> Map {
        Self { cities: vec![] }
    }

    pub fn load_map(&mut self, file_name: String) -> Result<(), std::io::Error> {
        let file = File::open(file_name.clone())
            .expect(&format!("File not found: {}", file_name));
        
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;

            if line.len() == 0 {
                break
            }

            if !line.as_bytes()[0].is_ascii_digit() {
                continue
            }

            let mut parts = line.split_whitespace().map(|s| s.parse::<f64>());
            match (parts.next(), parts.next(), parts.next()) {
                (Some(Ok(city_id)), Some(Ok(x)), Some(Ok(y))) => {
                    self.cities.push(
                        City { 
                            id: city_id as i32,
                            x: x,
                            y: y,
                    })
                },
                _ => break,
            }
        }
        Ok(())
    }

    pub fn path_distance(&self, path: &Vec<i32>) -> f64 {
        let mut distance: f64 = 0.0;
        let n = path.len();
        for i in 0..n {
            let to_city = *self.cities.iter().find(|x| x.id == path[i]).expect("City with id not found");
            let from_city = *self.cities.iter().find(|x| x.id == path[(i+1)%n]).expect("City with id not found");

            distance += to_city - from_city;
        }
        distance
    }

    pub fn random_path(&self) -> (Vec<i32>, f64) {
        let mut path = self.cities.iter().map(|x| x.id).collect::<Vec<i32>>();

        let mut rng = thread_rng();
        path.shuffle(&mut rng);

        let distance = self.path_distance(&path);

        return (path, distance)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Map with {} cities.\n", self.cities.len())?;

        for city in &self.cities {
            write!(f, "{}\n", city)?;

        }
        
        Ok(())
    }
}