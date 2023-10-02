use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread::{self},
};
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // let val: String = String::from("hi");
    let (sender, reciever) = mpsc::channel();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let spawn = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            thread_method(*num, sender);
        });

        handles.push(spawn);
    }

    // let _ = spawn.join();
    for handle in handles {
        handle.join();
    }

    let rec = reciever.recv().unwrap();
    println!("recieved {}", rec);
}

fn thread_method(val: i32, sender: Sender<i32>) {
    println!("value {}", val);
    let _ = sender.send(val);
}
