use std::fmt::Display;
use crate::utility::{map::Map, agent::Agent};

#[derive(Clone, Debug)]
pub struct Population {
    pub population_size: i32,
    pub agents: Vec<Agent>,
}

impl Population {

    pub fn new(population_size: i32, map: &Map) -> Self {
        let mut pop = Self { 
                population_size: population_size, 
                agents: vec![]
            };
        pop.initialize_agents(map);
        pop
    }

    pub fn initialize_agents(&mut self, map: &Map) {
        while (self.agents.len() as i32) < self.population_size {
            self.agents.push(Agent::new_random(map))
        }
    }
}

impl Display for Population {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Population size: {}\n", self.population_size)?;

        for agent in &self.agents {
            write!(f, "{}\n", agent)?;
        }

        Ok(())
    }
}