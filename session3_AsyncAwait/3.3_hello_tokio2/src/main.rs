async fn hello() -> u32 {
    println!("Hello");
    2
}

async fn hello2() -> u32 {
    println!("Hello Tokio2");
    4
}

async fn ticker() {
    for i in 0..10 {
        println!("tick {i}");
        tokio::task::yield_now().await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    hello().await;

    tokio::spawn(ticker());

    hello().await;
    let results = tokio::join!(hello(), hello2());
    println!("{results:?}");

    let _ = tokio::join!(
        tokio::spawn(hello()),
        tokio::spawn(ticker()),
        tokio::spawn(ticker()),
    );

    println!("Finished");
}
