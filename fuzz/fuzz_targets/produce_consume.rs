#![no_main]
use libfuzzer_sys::fuzz_target;


fuzz_target!(|data: &[u8]| {
    let my_vec = data.to_vec();
    // Initialize a queue with capacity of 500 values
    let (p, c) = bounded_spsc_queue::make(500);


    for _ in 0..100 {
        p.push(my_vec.clone());
    }

    for _ in 0..100 {
        c.pop();
    }
});
