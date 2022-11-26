use crate::{algorithms::{basic_genetic::{ BasicGenetic, BasicAgent }, two_opt::two_opt}, utility::{population::Population, agent::Agent}};
use utility::map::Map;
use std::{collections::HashMap, env, fs};

pub mod algorithms;
pub mod utility;
fn main() {
    const MAX_UNCHANGED: i32 = 10;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run <dataset-name>");
        return;
    }
    let file_name = args[1].clone();

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
    let mut population = Population::<BasicAgent>::new(population_size, &map);
    let mut genetic_algo = BasicGenetic::new(3, &mut population, 0.65, 0.1);

    let mut generation: HashMap<i32, BasicAgent> = HashMap::new();

    let mut best_fitness = 0f64;
    let mut unchanged_for_iterations = 0;
    let mut best_agent = BasicAgent::new_random(&map);

    for i in 0..num_generations {
        println!("------Generation {} ------", i+1);
        println!("Selection");
        genetic_algo.selection(&map);
        println!("Crossover");
        genetic_algo.crossover(&map);
        println!("Mutation");
        genetic_algo.mutation(&map);
        println!("Replacement");
        generation.insert(i+1, genetic_algo.replacement(&map));

        let curr_fitness = generation[&(i+1)].fitness_score();
        if curr_fitness > best_fitness {
            best_fitness = curr_fitness;
            best_agent = generation[&(i+1)].clone();
            unchanged_for_iterations = 0;
        } else {
            unchanged_for_iterations += 1;
        }
        if unchanged_for_iterations >= MAX_UNCHANGED {
            println!("No better fitness score for a long time. Terminating genetic algorithm.");
            break;
        }
    }
    println!("Best path found: {:?}", best_agent.path);
    println!("Distance: {}", best_agent.distance);
    fs::write("genetic.txt", format!("{:?}", best_agent.path)).expect("Unable to write to file");

    let two_opt_path = two_opt(&best_agent.path, 100000, &map);
    let two_opt_distance = map.path_distance(&two_opt_path);
    println!("After two opt search: {:?}", two_opt_path);
    println!("Distance: {}", two_opt_distance);
    fs::write("two_opt.txt", format!("{:?}", two_opt_path)).expect("Unable to write to file");

}
