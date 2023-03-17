mod channels;
mod simple_threads;
mod scoped_threads;

fn main() {
    simple_threads::simple_threads_main();

    channels::channels_main();
    
    scoped_threads::scoped_threads_main();
}
