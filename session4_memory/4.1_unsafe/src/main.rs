fn main() {
    let my_vet: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    // my_vet[32];
    if let Some(value) = my_vet.get(2) {
        // Do something
    }
    let my_vec = Vec::<u32>::new();
    unsafe {
        println!("{}", my_vec.get_unchecked(2));
    }
}

// If you are using RustDoc, and the associated missing_docs warning then the documentation system will insist that any unsafe code be documented:

/// # Safety
/// This function calls into C code, and must be called with care. It is not thread-safe.
unsafe fn my_unsafe_function() {
    ...
}