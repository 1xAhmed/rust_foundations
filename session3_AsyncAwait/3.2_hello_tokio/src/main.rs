// use tokio::runtime;

async fn hello() {
    println!("Hello Tokio");
}

// Hardway to do

// fn main() {
//     let rt = runtime::Builder::new_multi_thread()
//         .enable_all()
//         .worker_threads(4)
//         .build()
//         .unwrap();

//     rt.block_on(hello());
// }

// Easy way to do
#[tokio::main]
async fn main() {
    hello().await;
}
