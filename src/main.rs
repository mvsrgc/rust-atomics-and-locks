use std::thread;
use std::time::Duration;
mod channels;
use channels::ex3_channels_call;

fn main() {
    // Uncomment to execute examples 1 and 2
    // ex1();
    // ex2();
    // ex2_main_thread_busy();

    // Uncomment to execute example 3
    // ex3_channels();
}

fn ex3_channels() {
    let t = thread::spawn(|| {
        ex3_channels_call();
    });

    t.join().unwrap();

}

// Example 3: Demonstrate main thread being busy doing other work
fn ex3_main_thread_busy() {
    // Spawn new thread to calculate the average
    let t = thread::spawn(|| {
        let average = ex3();
        println!("The average is: {}", average);
    });

    // Main thread busy doing other tasks
    for i in 1..=5 {
        println!("Main thread: Doing something else...");
        thread::sleep(Duration::from_millis(200));
    }
}

// Example 3: Calculate the average of numbers in a range
fn ex3() -> usize {
    let numbers = Vec::from_iter(0..=100000000);

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });

    t.join().unwrap()
}

// Example 2: Using a spawned thread to print numbers
fn ex2() {
    let numbers = vec![1, 2, 3];

    // When using &numbers in spawn, numbers has to be moved because
    // spawn has a lifetime bound of 'static which means the closure
    // might outlive ex2(), and in that case &numbers in the closure
    // would refer to invalid memory because numbers will have been freed.
    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();
}

// Example 1: Basic thread creation and usage
fn ex1() {
    let t1 = thread::spawn(ex1_f);
    let t2 = thread::spawn(ex1_f);

    println!("Hello from the main thread.");

    t1.join().unwrap();
    t2.join().unwrap();
}

fn ex1_f() {
    println!("Hello from another thread!");
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
