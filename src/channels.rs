use std::sync::mpsc::{Sender, Receiver, TryRecvError, RecvTimeoutError};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn ex3_channels(tx: Sender<usize>) {
    let len = 10_000_000;
    let mut sum = 0;

    for (i, n) in (0..10_000_000).into_iter().enumerate() {
        sum += n;
        let _ = tx.send(sum / (i + 1));
    }

    let _ = tx.send(sum / len);
}

#[inline(never)]
pub fn ex3_channels_call() {
    let (tx, rx) = mpsc::channel();

    let t = thread::spawn(move || {
        ex3_channels(tx);
    });

    let start_time = Instant::now();
    let mut last_print_time = Instant::now();

    loop {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(average) => {
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

    t.join().expect("The child thread has panicked");
}
