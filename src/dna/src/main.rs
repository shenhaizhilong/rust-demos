use crate::population::Population;

mod mating_pool;
mod dna;
mod population;

/**
 * https://medium.com/@mohammam.mz/implement-genetic-algorithm-using-rust-56b9622fdb94
 * https://www.youtube.com/watch?v=nrKjSeoc7fc
 **/
fn main() {
    let target = String::from("to be or not to be that is");

    let mut population = Population::new(&target);

    let mut g_count = 0;

    loop {
        population.calculate_all_fitness(&target);
        let total_fitness = population.total_fitness();

        let best = population.find_best();

        println!("Generation {}", g_count);
        println!("Total Fitness: {}", total_fitness);
        println!("Best DNA: {}", best.genes);
        println!("Fitness: {}", best.fitness);
        print!("\n");

        if best.genes == target || best.fitness > 0.90 {
            break;
        }
        // get mating pool
        let pool = population.get_matting_pool();
        // cross over and get new population
        population = Population::cross_over(pool);
        // increment generation
        g_count += 1;
    }
}
