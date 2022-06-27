use std::fmt::Display;

use rand::Rng;

#[derive(Debug)]
pub struct Permutation {
    sequence: Vec<usize>,
}

impl Permutation 
{
    pub fn identity(n : usize) -> Self {
        let sequence = (0..n).collect();
        Self { sequence }
    }

    pub fn permute_in_place(&mut self) {
        let mut rng = rand::thread_rng();
        
        let n = self.sequence.len();

        for i in 0..n {
            let j = rng.gen_range(i..n);
            self.swap(i, j);
        }
    }

    pub fn find_cycles(&self) -> Vec<Vec<usize>>{
        let n = self.sequence.len();
        let mut cycles : Vec<Vec<usize>> = Vec::new();
        let mut walked_elements = vec![false; n];

        for mut i in 0..n {
            let mut new_cycle : Vec<usize> = Vec::new();
            while !walked_elements[i] {
                new_cycle.push(i);
                walked_elements[i] = true;
                i = self.sequence[i];
            }
            if !new_cycle.is_empty() {
                cycles.push(new_cycle);
            }
        }

        cycles
    }

    pub fn count_cycles(&self) -> usize{
        self.find_cycles().len()
    }

    pub fn count_cycles_less_alloc(&self) -> usize {
        let n = self.sequence.len();
        let mut walked_elements = vec![false; n];

        let mut count = 0;

        for mut i in 0..n {
            if !walked_elements[i] {
                count = count + 1;

                while !walked_elements[i] {
                    walked_elements[i] = true;
                    i = self.sequence[i];
                }
            }
        }

        count
    }

    pub fn max_length_cycle(&self) -> usize{
        self.find_cycles().into_iter().fold(0, |max_length,cycle| {
            max_length.max(cycle.len())
        })
    }

    pub fn max_length_less_alloc(&self) -> usize {
        let n = self.sequence.len();
        let mut walked_elements = vec![false; n];

        let mut max = 0;

        for mut i in 0..n {
            if !walked_elements[i] {
                let mut count = 0;

                while !walked_elements[i] {
                    count = count + 1;
                    walked_elements[i] = true;
                    i = self.sequence[i];
                }

                max = max.max(count);
            }
        }

        max
    }

    fn swap(&mut self, x : usize, y : usize) {
        let temp = self.sequence[x];
        self.sequence[x] = self.sequence[y];
        self.sequence[y] = temp;
    }

}

impl Display for Permutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.sequence)
    }
}
