 fn main() {
    let n: i32 = 5;

    println!("{n}");
    let n: () = {
        let n = 76;
    };

    println!("{n:?}");
}
