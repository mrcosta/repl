use std::{thread, time};

static NTHREADS: i32 = 2;

pub fn thread_that_dont_wait_and_finish_after_main() {
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            thread::sleep(time::Duration::from_millis(10));
            println!("this is thread number {}", i);
        }));
    }

    // This one is not going to print because we are NOT joining them. It means that we don't wait for them to finish
    // before finish the method main
}

pub fn thread_that_waits_and_finish_before_main() {
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            thread::sleep(time::Duration::from_millis(10));
            println!("this is thread number {}", i);
        }));
    }

    // This one is going to print because we are joining them. It means that we wait for them to finish
    // before finishing the method main

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
