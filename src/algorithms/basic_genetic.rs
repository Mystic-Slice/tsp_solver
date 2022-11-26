use rand::{thread_rng, seq::IteratorRandom, Rng};
use std::fmt::Display;

use crate::utility::{population::Population, map::Map, agent::Agent};

#[derive(Clone, Debug)]
pub struct BasicAgent {
    pub path: Vec<i32>,
    pub distance: f64,
}

impl BasicAgent {
    pub fn new(path: Vec<i32>, distance: f64) -> Self {
        Self { path: path, distance: distance }
    }
}

impl Agent for BasicAgent {
    fn fitness_score(&self) -> f64 {
        1.0/self.distance
    }

    fn path(&self) -> Vec<i32> {
        self.path.clone()
    }

    fn distance(&self) -> f64 {
        self.distance
    }

    fn new_random(map: &Map) -> Self {
        let (path, distance) = map.random_path();
        Self { path: path, distance: distance }
    }
}

impl Display for BasicAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path distance: {}, Path: {:?}", self.distance, self.path)
    }
}

pub struct BasicGenetic<'a> {
    pub tournament_size: i32,
    pub population: &'a mut Population<BasicAgent>,
    pub cross_prob: f64,
    pub mut_prob: f64,
    pub parents: Vec<BasicAgent>, // Temp variable -> find a better solution
    pub offspring: Vec<BasicAgent>, // Temp variable
    pub fittest: Option<BasicAgent>, // Temp variable
}

impl<'a> BasicGenetic<'a> {
    pub fn new(tournament_size: i32, population: &'a mut Population<BasicAgent>, cross_prob: f64, mut_prob: f64) -> Self {
        Self {
            tournament_size: tournament_size,
            population: population,
            cross_prob: cross_prob,
            mut_prob: mut_prob,
            parents: vec![],
            offspring: vec![],
            fittest: None,
        }
    }

    pub fn selection(&mut self, map: &Map) {

        let mut rng = thread_rng();

        self.parents.clear();

        while (self.parents.len() as i32) != self.population.population_size {
            let participants = self.population.agents.iter().choose_multiple(&mut rng, self.tournament_size as usize);

            let best_agent = participants.iter().max_by(|a, b| a.fitness_score().partial_cmp(&b.fitness_score()).unwrap());

            let best_path = best_agent.unwrap().path.clone();
            let best_distance = map.path_distance(&best_path);
            self.parents.push(BasicAgent::new(best_path, best_distance));
        }
    }

    pub fn crossover(&mut self, map: &Map) {
        let mut rng = thread_rng();

        self.offspring.clear();

        while self.parents.len() != 0 {
            let index = rng.gen_range(0..=self.parents.len()-1);
            let a_path = self.parents[0].path.clone();
            let b_path = self.parents[index].path.clone();

            let pc = rng.gen::<f64>();

            if pc <= self.cross_prob {
                let crossover_index = rng.gen_range(0..=map.cities.len()-3);

                let window_a = &a_path[crossover_index..crossover_index+3];
                let window_b = &b_path[crossover_index..crossover_index+3];

                let mut c_path: Vec<i32> = vec![];
                let mut d_path: Vec<i32> = vec![];

                let mut i = 0;
                let mut j = 0;

                while c_path.len() != crossover_index {
                    if !window_a.iter().any(|&x| x == b_path[i]) {
                        c_path.push(b_path[i]);
                    }
                    i += 1;
                }

                while d_path.len() != crossover_index {
                    if !window_b.iter().any(|&x| x == a_path[j]) {
                        d_path.push(a_path[j])
                    }
                    j += 1;
                }

                c_path.append(&mut window_a.clone().to_vec());
                d_path.append(&mut window_b.clone().to_vec());

                while c_path.len() != map.cities.len() {
                    if !window_a.iter().any(|&x| x == b_path[i]) {
                        c_path.push(b_path[i])
                    }
                    i += 1
                }

                while d_path.len() != map.cities.len() {
                    if !window_b.iter().any(|&x| x == a_path[j]) {
                        d_path.push(a_path[j])
                    }
                    j += 1
                }
                let c_distance = map.path_distance(&c_path);
                self.offspring.push(BasicAgent::new(c_path, c_distance));

                let d_distance = map.path_distance(&d_path);
                self.offspring.push(BasicAgent::new(d_path, d_distance));                
            } else {
                let a_distance = map.path_distance(&a_path);
                self.offspring.push(BasicAgent::new(a_path, a_distance));

                let b_distance = map.path_distance(&b_path);
                self.offspring.push(BasicAgent::new(b_path, b_distance));    
            }
            self.parents.remove(index);
            self.parents.remove(0);
        }
    }

    pub fn mutation(&mut self, map: &Map) {
        let mut rng = thread_rng();

        for x in 0..self.offspring.len() {
            let pm = rng.gen::<f64>();

            if pm <= self.mut_prob {
                let indices = vec![rng.gen_range(0..=map.cities.len()-1); 2];

                let mut route = self.offspring[x].path.clone();

                let city = route[indices[0]];
                route[indices[0]] = route[indices[1]];
                route[indices[1]] = city;

                self.offspring[x].path = route
            }
        }
    }

    pub fn replacement(&mut self, map: &Map) -> BasicAgent {
        self.population.agents = self.offspring.clone();
        
        for agent in &self.population.agents {
            if let Some(fittest_agent) = self.fittest.clone() {
                if fittest_agent.fitness_score() < agent.fitness_score() {
                    self.fittest = Some(agent.clone());
                }
            } else {
                self.fittest = Some(agent.clone());
            }
        }

        for agent in &self.offspring {
            if let Some(fittest_agent) = self.fittest.clone() {
                if fittest_agent.fitness_score() < agent.fitness_score() {
                    self.fittest = Some(agent.clone());
                }
            } else {
                self.fittest = Some(agent.clone());
            }
        }
        
        let fittest_agent = self.fittest.clone().unwrap();
        println!("Fittest individual: {}", fittest_agent.fitness_score());
        println!("Path length: {}", fittest_agent.distance);

        let city_path = fittest_agent.clone().path;
        println!("Fittest route: {:?}", city_path);

        fittest_agent
    }

}
