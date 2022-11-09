use utility::map::Map;

use crate::utility::population::Population;

pub mod utility;
fn main() {
    let file_name = "datasets/Berlin52/Berlin52.tsp".to_string();

    let mut map = Map::new();
    map.load_map(file_name).unwrap();

    println!("Map: {}", map);

    println!("Randomly Generated path: {:?}", map.random_path());

    println!("Sample city in map: {}", map.cities[0]);

    let pop = Population::new(10, &map);
    println!("Randomly generated population: {}", pop);

    println!("Sample agent in population: {}", pop.agents[0]);
}
