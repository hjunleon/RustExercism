use core::num;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
// use std::time::Duration;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let num_texts = input.len();
    let mut handles = vec![];
    let mut freq_map: Mutex<HashMap<char, usize>> = Mutex::new(HashMap::new());
    for text_idx in 0..num_texts {
        // unsafe {
            let cur_text = input.get(text_idx).cloned().unwrap_or("a");
            let input_len = cur_text.chars().count();
            for worker_id in 0..worker_count {
                let handle = thread::spawn(move || {
                    for i in (worker_id..input_len).step_by(worker_count) {
                        let mut my_map = freq_map.lock().unwrap();
                        *my_map.entry(cur_text.chars().nth(i).unwrap()).or_insert(0) += 1;
                    }
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        // }
    }
    return *freq_map.lock().unwrap().clone();
    // unimplemented!(
    //     "Count the frequency of letters in the given input '{:?}'. Ensure that you are using {} to process the input.",
    //     input,
    //     match worker_count {
    //         1 => "1 worker".to_string(),
    //         _ => format!("{} workers", worker_count),
    //     }
    // );
}
