use std::thread;

pub fn scoped_threads_main() {
    scoped_threads();    
}

fn scoped_threads() {
    let mut numbers = vec![1,2,3];

    // Scoped thread is joined automatically once ::scope
    // scope ends.
    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });

    // Scoped threads have been joined, it's safe to use numbers
    for i in numbers {
        println!("{i}");
    }
}
