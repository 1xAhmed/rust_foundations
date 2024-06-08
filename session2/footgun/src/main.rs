// static mut COUNTER: i32 = 0;

use std::sync::atomic::AtomicI32;

static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut handles = Vec::new();
    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1_100 {
                COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("{}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}
 


 /* Unsafe Version, bugs: Race condition



 static mut COUNTER: i32 = 0;

fn main() {
    let mut handles = Vec::new();
    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1_100 {
                unsafe {
                    COUNTER += 1;
                }
            }
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    unsafe {
        println!("{COUNTER}");
    }
}

 
 
  */