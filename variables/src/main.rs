fn double(n: i32) -> i32 {
    n * 2
}

fn double_or_nothing(n: i32) -> i32 {
    if n > 0 {
        return n * 2;
    }
    0
}

fn greet(s: String) -> String {
    print!("Hellp {s}");
    s
}

fn main() {
    let n = double(2);
    println!("{}", double(4));
    let p = double_or_nothing(n);
    print!("p {p}");
    println!("{n}");
    let i = 5;
    let m = if i ==5 {
        6
    } else {
        7
    };

    println!("{m}");

    let n: () = {
        let n = 76;
        println!("{n:?}");
    };

    println!("{n:?}");

    let name = "Hello".to_string();
    println!("hello {name}");
    let name = greet(name);
    greet(name);
}
