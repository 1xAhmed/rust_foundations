use futures::executor::block_on;
use futures::future::join_all;
use futures::join;

fn do_something_async() {
    println!("Not async!");
}

async fn say_hello() {
    println!("Hello");
    join!(second_funciton(), say_goodbye());

    let n = double(4).await;
    println!("{n}");

    let futures = vec![double(1), double(2), double(4)];
    let results = join_all(futures).await;
    println!("{results:?}");

    do_something_async();
}

async fn second_funciton() {
    println!("Hello again")
}

async fn say_goodbye() {
    println!("Goodbye")
}

async fn double(n: u32) -> u32 {
    n * 2
}

fn main() {
    block_on(say_hello());
}
