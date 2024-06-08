use std::sync::Mutex;

static MY_SHARED: Mutex<u32> = Mutex::new(3);

fn poisoner() {
    let mut lock = MY_SHARED.lock().unwrap();
    *lock += 1;
    panic!("The poisoner strikes")
}

fn main() {
    /// Deadlock Example
    ///
    // ----------------------------
    // let lock = MY_SHARED.lock().unwrap();
    // std::mem::drop(lock);
    // if let Ok(_lock) = MY_SHARED.try_lock() {
    //     println!("Got the lock");
    // } else {
    //     println!("No lock");
    // }
    // ----------------------------


    // Poisoning the Mutex
    let handle = std::thread::spawn(poisoner);
    println!("Trying to from the thread");
    println!("{:?}", handle.join());

    let lock = MY_SHARED.lock();
    println!("{lock:?}");

    let recoverred_data = lock.unwrap_or_else(|poisoned| {
        println!("Mutex is poisoned, reconvering data...");
        poisoned.into_inner()
    });
}
