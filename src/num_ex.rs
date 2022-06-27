use std::thread;
use crate::permut::Permutation;

pub fn numerical_expected_value<F: Fn(&Permutation) -> f32 + Send + Copy + 'static>(perm_length : usize, sample_size : u32, thread_count : usize, random_variable : F) -> f32 {
    let mut thread_pool = Vec::new();
    thread_pool.reserve(thread_count);
    
    let local_sample_size = sample_size/thread_count as u32;

    for _ in 0..thread_count {
        let new_thread = thread::spawn(move || {
            let mut permutation = Permutation::identity(perm_length);
            let mut sum = 0f32;
            for _ in 0..local_sample_size {
                permutation.permute_in_place();
                sum = sum + random_variable(&permutation);
            }
        
            let average = sum / local_sample_size as f32;
        
            average
        });
        
        thread_pool.push(new_thread);
    }
    
    let mut sum = 0f32;
    for thr in thread_pool {
        sum = sum + thr.join().unwrap();
    }

    let average = sum / thread_count as f32;

    average
}