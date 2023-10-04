use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let (sender, reciever) = mpsc::channel();
    let mut spawns = vec![];

    for _i in 1..10 {
        let sender = sender.clone();
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            let _ = sender.send(*num);
            *num += 1;
            func1(*num);
        });
        spawns.push(handle);
    }

    for thread in spawns {
        _ = thread.join();
        let recieved = reciever.recv().unwrap();
        println!("recieved {}", recieved);
    }
}

fn func1(num: u32) {
    // num = num + 1;
    println!("thread {}", num);
    thread::sleep(Duration::from_millis(1000));
}

