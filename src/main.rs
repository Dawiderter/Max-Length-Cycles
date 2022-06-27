use crate::num_ex::*;
use crate::results_writer::*;

pub mod permut;
pub mod num_ex;
pub mod results_writer;

fn main() {
    const N: usize = 100;
    const K: u32 = 100_000;

    let mut number_cycles_writer = ResultsWriter::create("cycles").unwrap();
    number_cycles_writer.add_row(vec!["n", "Oczekiwana liczba cykli"]).unwrap();

    for n in 0..N {
        let result = numerical_expected_value(n, K, 16, 
            |p| {
                p.count_cycles_less_alloc() as f32
        });
        number_cycles_writer.add_row(vec![n as f32, result]).unwrap();
    }

    let mut max_cycles_writer = ResultsWriter::create("maximums").unwrap();
    max_cycles_writer.add_row(vec!["n", "Oczekiwana długość najdłuższego ciągu"]).unwrap();

    for n in 0..N {
        let result = numerical_expected_value(n, K, 16, 
            |p| {
                p.max_length_less_alloc() as f32
        });
        max_cycles_writer.add_row(vec![n as f32, result]).unwrap();
    }
}
