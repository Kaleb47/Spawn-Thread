use std::thread;
use std::time::Duration;


fn main() {
    thread::spawn(|| {
     for i in 1..10 { 
       println!("hi number {} from the spawned thread", i); 
     }
    });

  for i in 1..5 {
    println!("hi number {} from the spawned thread", i); 
    thread::sleep(Duration::from_millis(1));
  }
}