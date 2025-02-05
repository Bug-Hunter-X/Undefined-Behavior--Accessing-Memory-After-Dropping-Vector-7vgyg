fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let ptr = vec.as_ptr();

    // Drop the vector. The memory pointed to by `ptr` is now invalid.
    drop(vec);

    // Accessing the memory pointed to by `ptr` is undefined behavior
    unsafe {
        println!("Value at ptr: {}", *ptr);
    }
}