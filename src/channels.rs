use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

fn ex3_channels(tx: Sender<f64>) {
    let len = 10_000_000;
    let mut sum = 0.0;

    for (i, n) in (1..=10_000_000).into_iter().enumerate() {
        sum += n as f64;
        let _ = tx.send(sum / (i + 1) as f64);
    }

    let _ = tx.send(sum / len as f64);
}

#[inline(never)]
pub fn ex3_channels_call() {
    let (tx, rx) = mpsc::channel();

    let t = thread::spawn(move || {
        ex3_channels(tx);
    });

    let mut last_print_time = Instant::now();
    let mut last_received = 0.0;

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
            Err(RecvTimeoutError::Timeout) => {
                continue;
            }
            Err(RecvTimeoutError::Disconnected) => {
                println!("The child thread has finished.");
                break;
            }
        }
    }

    println!("Final average: {}", last_received);

    t.join().expect("The child thread has panicked");
}
