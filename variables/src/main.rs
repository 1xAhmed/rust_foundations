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
    print!("Hello {s}");
    s
}

fn greet_borrow(s: &String) {
    print!("{s}");
}

fn greet_borrow_mut(s: &mut String) {
    *s = format!("Hello {s}");
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
    
 
    let mut name = greet(name);
    greet_borrow_mut(&mut name);
    println!("hello {name}");
    greet_borrow(&name);
    println!("hello {name}");
    
}
