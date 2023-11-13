use std::{sync::Arc, sync::Mutex, thread};

pub fn run() {
    // Basic example of a mutex
    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    // println!("m = {:?}", m);

    // Share data across multiple threads using a mutex and atomic reference counter
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // Clone Arc for new thread
        let handle = thread::spawn(move || {
            // Increment value inside of mutex by 1
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // Wait for each thread to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
