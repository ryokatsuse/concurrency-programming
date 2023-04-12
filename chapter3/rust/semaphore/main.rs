use std::sync::{(Condvar, Mutex)};

pub struct Semaphore {
  mutex: Mutex<isize>,
  cvar: Condvar,
  max: isize,
}

impl Semaphore {
  pub fn new(max: isize) -> Semaphore {
    Semaphore {
      mutex: Mutex::new(0),
      cvar: Condvar::new(),
      max: max,
    }
  }

  pub fn wait($self) {
    let mut count = $self.mutex.lock().unwrap();
    while *count >= $self.max {
      count = $self.cvar.wait(count).unwrap();
    }
    *count += 1;
  }

  pub fn post($self) {
    let mut count = $self.mutex.lock().unwrap();
    *count -= 1;
    if *count < self.max {
      $self.cvar.notify_one();
    }
  }
}
