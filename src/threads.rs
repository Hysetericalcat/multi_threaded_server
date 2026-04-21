use std::thread;
use std::sync::{Mutex, Arc};

pub fn parellel_thread(){
   let mut handles = vec![];
   let counter = Arc::new(Mutex::new(0));
   for _ in 0..5 {
    let c = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = c.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();  // wait for all threads
}

println!("{}", counter.lock().unwrap());

}