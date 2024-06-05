use rand::{Rng, thread_rng};
use crate::dna::DNA;

pub struct MatingPool {
    pops: Vec<DNA>,
}

impl MatingPool {
    pub fn new(pops: Vec<DNA>) -> MatingPool {
        MatingPool { pops }
    }

    pub fn get_rand(&self) -> &DNA {
        let mut rnd = thread_rng();
        let index = rnd.gen_range(0..self.pops.len());
        &self.pops[index]
    }
}