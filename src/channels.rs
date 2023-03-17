use std::sync::mpsc;
use std::sync::mpsc::{RecvTimeoutError, Sender};
use std::thread;
use std::time::{Duration, Instant};

// Calculate cumulative average of numbers from 1 to 10_000_000
// and sends average to main thread using Sender.
fn ex3_channels(tx: Sender<f64>) {
    let len = 10_000_000;
    let mut sum = 0.0;

    // Calculate cumulative average
    for (i, n) in (1..=len).enumerate() {
        sum += n as f64;
        let _ = tx.send(sum / (i + 1) as f64);
    }

    // Send final average
    let _ = tx.send(sum / len as f64);
}

// Print received average every 100ms
pub fn ex3_channels_call() {
    let (tx, rx) = mpsc::channel();

    let t = thread::spawn(move || {
        ex3_channels(tx);
    });

    let mut last_print_time = Instant::now();
    let mut last_received = 0.0;

    // Main loop to receive average values from the child thread
    loop {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(average) => {
                last_received = average;
                let elapsed = last_print_time.elapsed();

                if elapsed >= Duration::from_millis(100) {
                    println!("Current average: {}", average);
                    last_print_time = Instant::now();
                }
            }
            // No message received during the timeout period
            Err(RecvTimeoutError::Timeout) => {
                continue;
            }
            // Child thread finished, exit loop
            Err(RecvTimeoutError::Disconnected) => {
                println!("The child thread has finished.");
                break;
            }
        }
    }

    println!("Final average: {}", last_received);

    // Wait for the child thread to finish
    t.join().expect("The child thread has panicked");
}
