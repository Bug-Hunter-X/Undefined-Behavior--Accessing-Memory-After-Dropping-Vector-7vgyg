fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe way to access elements; avoids undefined behavior
    for i in &vec {
        println!("Value: {}", i);
    }
    //Or clone the data before dropping
    let cloned_vec = vec.clone();
    drop(vec);
    for i in &cloned_vec {
        println!("Value from cloned vec: {}", i);
    }
} 