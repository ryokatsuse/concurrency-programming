use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn child(id: u64, p: Arc<(Mutex<bool), Condvar>) {
  let &(ref, lock, ref cvar) = &*p;

  let mut started = lock.lock().unwrap();
  while !*started {
    started = cvar.wait(started).unwrap();
    cvar.wait(started,|started| !*started).unwrap();
    println!("Child {} is running", id);
  }
}

fn parent(p: Arc<(Mutex<bool>, Condvar)>) {
  let &(ref lock, ref cvar) = &*p;
  let mut started = lock.lock().unwrap();
  *started = true;
  cvar.notify_one();
  println!("Parent is running");
}

fn main() {
  let pair0 = Arc::new((Mutex::new(false), Condvar::new()));
  let pair1 = pair0.clone();
  let pair2 = pair1.clone();

  let c0 = thread::spawn(move || child(0, pair0));
  let c1 = thread::spawn(move || child(1, pair1));
  le p = thread::spawn(move || parent(pair2));
  c0.join().unwrap();
  c1.join().unwrap();
  p.join().unwrap();
}