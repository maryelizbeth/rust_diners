use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Table {
  forks: Vec<Mutex<()>>,
}

struct Poet {
  name: String,
  left: usize,
  right: usize,
}

impl Poet {

  fn new(name: &str, left: usize, right: usize) -> Poet {
    Poet {
      name: name.to_string(),
      left: left,
      right: right,
    }
  }

  fn eat(&self, table: &Table){

    let _left = table.forks[self.left].lock().unwrap();
    thread::sleep(Duration::from_millis(150));
    let _right = table.forks[self.right].lock().unwrap();

    println!("{} is eating.", self.name);

    thread::sleep(Duration::from_millis(1000));

    println!("{} is done eating.", self.name);
  }
}

fn main() {

  let table = Arc::new(Table { forks: vec![
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
  ]});

  let poets = vec! [
    Poet::new("Mary Oliver", 0, 1),
    Poet::new("Anne Sexton", 1, 2),
    Poet::new("Richard Wilbur", 2, 3),
    Poet::new("WH Auden", 3, 4),
    Poet::new("Philip Larkin", 0, 4),
  ];

  let handles: Vec<_> = poets.into_iter().map(|p| {
    let table = table.clone();

    thread::spawn(move || {
      p.eat(&table);
    })
  }).collect();

  for h in handles {
    h.join().unwrap();
  }
}

