use std::thread;
use std::time::Duration;

struct Poet {
  name: String,
}

impl Poet {

  fn new(name: &str) -> Poet {
    Poet {
      name: name.to_string(),
    }
  }

  fn eat(&self){
    println!("{} is eating.", self.name);

    thread::sleep(Duration::from_millis(1000));

    println!("{} is done eating.", self.name);
  }
}

fn main() {

  let poets = vec! [
    Poet::new("Mary Oliver"),
    Poet::new("Anne Sexton"),
    Poet::new("Richard Wilbur"),
    Poet::new("WH Auden"),
    Poet::new("Philip Larkin"),
  ];

  for p in &poets {
    p.eat();
  }

}

