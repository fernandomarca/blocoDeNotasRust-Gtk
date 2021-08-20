use lazy_static::lazy_static;
use once_cell::sync::Lazy; // 1.3.1
use std::sync::Mutex; // 1.4.0

static ARRAY: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![]));

fn do_a_call() {
  ARRAY.lock().unwrap().push(1);
}

// fn main() {
//   do_a_call();
//   do_a_call();
//   do_a_call();

//   println!("called {}", ARRAY.lock().unwrap().len());
// }
//
lazy_static! {
  static ref ARRAY2: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

fn do_a_call() {
  ARRAY2.lock().unwrap().push(1);
}

// fn main() {
//   do_a_call();
//   do_a_call();
//   do_a_call();

//   println!("called {}", ARRAY2.lock().unwrap().len());
// }
