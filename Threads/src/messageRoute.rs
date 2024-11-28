use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("Rust world"),
        ];

        for v in vals {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });

    thread::spawn(move || {
        let vals = vec![
            String::from("More"),
            String::from("messages"),
            String::from("from the python world"),
        ];

        for v in vals {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });



    for r in rx {
        println!("Got :{r}");
    }
}
