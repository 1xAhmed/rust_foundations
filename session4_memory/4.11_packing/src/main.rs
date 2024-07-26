struct OneByte {
    a: u8,
}

struct TwoBytes {
    a: u16,
}

// Rust insert one bit per structure, to avoid this, check ThreeBytes2, Not recommanded
struct ThreeBytes {
    a: u16,
    b: u8,
}

struct FourBytes {
    a: u32,
}

// Rust insert one bit per structure
// #[no_mangle]    Dont mangle the struct names
#[repr(packed)]     // Represent this as Packed
#[repr(C)]      // Make sure that your type is in the same order as C's
struct ThreeBytes2 {
    a: u16,
    b: u8,
}

fn main() {
    println!("{}", std::mem::size_of::<OneByte>());
    println!("{}", std::mem::size_of::<TwoBytes>());
    println!("{}", std::mem::size_of::<ThreeBytes>());
    println!("{}", std::mem::size_of::<FourBytes>());
}
