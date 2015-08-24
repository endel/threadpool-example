use std::thread;

extern crate threadpool;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
  let pool = ThreadPool::new(4);
  let (tx, rx) = channel();
  println!("Pool of 4 threads created.");

  loop {
    for i in 0..8 {
      let tx = tx.clone();
      pool.execute(move || {
        println!("Executing... {}", i);
        tx.send(i).unwrap();
      });
    }
    println!("Enqueued 8 tasks");

    for i in 0..8 {
      println!("Receiving data from thread: {:?}", rx.recv().unwrap());
    }

    // wait to enqueue more jobs
    thread::sleep_ms(1000);
  }

}
