mod not_thread_safe {
    use std::thread;
    
    // #region not-thread-safe
    static mut COUNTER: i32 = 0;

    const THREADS: usize = 4;
    const ITERATIONS: usize = 100_000;

    pub unsafe fn main() { // keyword `unsafe` is required here
        let mut handles = Vec::new();



        for _ in 0..THREADS {
            handles.push(thread::spawn(|| {
                for _ in 0..ITERATIONS {


                    COUNTER += 1;



                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Expected: {}", THREADS * ITERATIONS);
        println!("Actual: {}", COUNTER);
    }
    // #endregion not-thread-safe
}

mod thread_safe {
    use std::sync::{Arc, Mutex};
    use std::thread;

    // #region thread-safe
    static COUNTER: Mutex<i32> = Mutex::new(0);

    const THREADS: usize = 4;
    const ITERATIONS: usize = 100_000;

    pub fn main() {
        let mut handles = Vec::new();


        
        for _ in 0..THREADS {
            handles.push(thread::spawn(move || {
                for _ in 0..ITERATIONS {
                    let mut value = COUNTER.lock().unwrap();

                    *value += 1;


                    
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Expected: {}", THREADS * ITERATIONS);
        println!("Actual: {}", COUNTER.lock().unwrap());
    }
    // #endregion thread-safe
}

pub fn main() {
    println!("First run is not thread-safe");
    unsafe {
        not_thread_safe::main();
    }
    
    println!("Second run is thread-safe");
    thread_safe::main();
}
