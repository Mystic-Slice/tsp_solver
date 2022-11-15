use crate::{algorithms::basic_genetic::BasicGenetic, utility::population::Population, utility::agent::Agent};
use itertools::Itertools;
use utility::map::Map;
use std::collections::HashMap;

pub mod algorithms;
pub mod utility;
fn main() {
    let file_name = "datasets/Berlin52/Berlin52.tsp".to_string();

    let mut map = Map::new();
    match map.load_map(file_name) {
        Ok(()) => (),
        Err(e) => {
            println!("Problem with opening map file: {}", e);
            return;
        }
    }

    let population_size = 1000;
    let num_generations = 1000;
    let mut population = Population::new(population_size, &map);
    let mut genetic_algo = BasicGenetic::new(3, &mut population, 0.65, 0.1);

    let mut generation: HashMap<String, Agent> = HashMap::new();

    for i in 0..num_generations {
        println!("------Generation {} ------", i+1);
        println!("Selection");
        genetic_algo.selection(&map);
        println!("Crossover");
        genetic_algo.crossover(&map);
        println!("Mutation");
        genetic_algo.mutation(&map);
        println!("Replacement");
        generation.insert(format!("Generation {}", i+1), genetic_algo.replacement(&map));
    }

    for (k, v) in generation.iter().sorted_by_key(|x| x.0) {
        println!("{}\n{}\n", k, v);
    }

}
