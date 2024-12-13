
use std::thread;
use std::time::Duration;


fn main() {

    let process = thread::spawn(move || {
        for i in 100..10000 {
            println!("{}", i + i);
            thread::sleep(Duration::from_millis(1));
        }
    });


    for i in 1..100 {
        println!("The product is {}", i * i );
        thread::sleep(Duration::from_millis(0));
    }

    for c in 200..100 {
        println!("Letter {}", c);
        thread::sleep(Duration::from_millis(10));
    }

    process.join().unwrap();

    let proc2 = thread::spawn(|| {
        let numbers =  vec![678, 432, 678, 990, 213, 76];

        for i in  &numbers {
            println!("{numbers:?}");
            thread::sleep(Duration::from_millis(12));
        }
    });

    proc2.join().unwrap();
}
