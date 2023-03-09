use std::thread;
use std::time::Duration;
use std::sync::mpsc;

/* This section is on fearless Concurrency */

/* Programs are run in process and a process can have multiple threads
to run operation independently */

/* Using mesage passing to transfer data between threads */
/* Do not communicate by sharing memory; share memory by communicating */


fn main() {

    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let val = String::from("hello");
        tx2.send(val).unwrap();
    });


    let mut threadHandles = Vec::new();
    for _ in 1..10 {
        let tx1 = tx.clone();
        let handle = thread::spawn(move || {
            
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }

            let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        threadHandles.push(handle);
    }
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("The length of the handles are {}", threadHandles.len());
    for received in rx {
        println!("Got: {}", received);
    }

    for handle in threadHandles {
        println!("Currently waiting for handles");
        handle.join();
    }
}
