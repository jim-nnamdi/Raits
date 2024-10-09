
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn _concurrent_main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn( move || {
        let vals = vec![String::from("a"),
        String::from("b"),
        String::from("c")];
        for val in vals.into_iter() {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx.into_iter() {
        println!("{}", received);
    }
}