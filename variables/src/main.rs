fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    let n = double(25);
    println!("{}", double(4));
    println!("{n}");
    let n: () = {
        let n = 76;
    };

    println!("{n:?}");
}
