use std::sync::{Arc, Mutex};
use std::thread;

fn some_func(lock: Arc<Mutex<u64>>) {
  loop {
    let mut num = lock.lock().unwrap();
    *num += 1;
    println!("{}", *num);
  }
}

fn main() {
  let lock0 = Arc::new(Mutex::new(0));

  let lock1 = lock0.clone();

  let th0 = thread::spawn(move || {
    some_func(lock0);
  });

  let th1 = thread::spawn(move || {
    some_func(lock1);
  });

  th0.join().unwrap();
  th1.join().unwrap();
}