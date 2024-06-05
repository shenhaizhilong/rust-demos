use rand::{Rng, thread_rng};
use crate::dna::DNA;
use crate::mating_pool::MatingPool;

const POP_SIZE: i32 = 100;

pub struct Population {
    pops: Vec<DNA>,
}

impl Population {
    pub fn new(target: &String) -> Population {
        let mut pops = vec![];
        for i in 0..POP_SIZE {
            pops.push(DNA::new(target));
        }

        Population {
            pops,
        }
    }

    pub fn calculate_all_fitness(&mut self, target: &String) {
        self.pops.iter_mut().for_each(|pop| {
            pop.calculate_fitness(target);
        })
    }

    pub fn find_best(&self) -> &DNA {
        let best_opt = self.pops
            .iter()
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).expect("expect a dna"));
        match best_opt {
            None => { panic!("didn't find a DNA") }
            Some(best) => {
                best
            }
        }
    }

    pub fn total_fitness(&self) -> f32 {
        let mut total = 0.0;
        self.pops.iter().for_each(|dna| {
            total += dna.fitness;
        });
        total = total / self.pops.len() as f32;
        total
    }

    /// gets a list of a randomly selected population
    /// but based on there `fitness` they will get more chance to be in the pool
    pub fn get_matting_pool(&self) -> MatingPool {
        let mut pool = vec![];
        let mut rng = thread_rng();
        while pool.len() < 10 {
            for dna in &self.pops {
                let chance = rng.gen_range(0.0..=1.0);
                if dna.fitness >= chance {
                    pool.push(dna.clone());
                }
            }
        }

        return MatingPool::new(pool);
    }


    /// ## cross_over
    /// receive a `MatingPool` that can be acquired from `get_mating_pool` method
    /// and returns the next generation of population
    pub fn cross_over(mating_pool: MatingPool) -> Population {
        let mut pops = vec![];
        for i in 0..POP_SIZE {
            // 随机获取双亲
            let p_a = mating_pool.get_rand();
            let p_b = mating_pool.get_rand();
            // 交叉操作，基因混合
            let mut child = p_a.cross_over(p_b);
            // 孩子基因突变
            child.mutate();
            pops.push(child);
        }

        Population { pops }
    }
}