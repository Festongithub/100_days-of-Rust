use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Hi Number {i} of spawned  thread");
            thread::sleep(Duration::from_millis(2));
        }
    });
    
    handle.join().unwrap();

    for i in 1..10 {
        println!("Number {i} of  the main thread ");
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![100, 300, 4500, 123, 435, 675, 908, 990];

    let i = thread::spawn(move || {
    println!("Here are the numbers {v:?}");
    });

    i.join().unwrap();

    let i = 1..100;
    let check = thread::spawn(move || { 
        for j in i {
        println!("Number {j:?}");
        thread::sleep(Duration::from_millis(1));
        }
    });

    check.join().unwrap();


}
