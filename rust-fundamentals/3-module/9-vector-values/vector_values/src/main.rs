fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize because it's used to index into a vector
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    get_item(3);

    // Retrieve a value at a specific index
    let third_value = vec[2];
    //println!("The third value in the vector is: {}", third_value);

    // Retrieve the last value
    let last_value = vec.last().unwrap();
    //println!("The last value in the vector is: {}", last_value);

    // Retrieve the first value using pattern matching
    // match vec.first() {
    //     Some(first_value) => println!("The first value in the vector is: {}", first_value),
    //     None => println!("The vector is empty!"),
    // }
}

/*
Rust uses usize for indexing because:

      - Memory safety: usize is designed to match the size of memory addresses on the platform (e.g., 32-bit or 64-bit). This ensures that indexing operations can safely represent valid memory offsets.
      - Performance: Since usize is the native integer type used for pointers and array indexing, it avoids unnecessary type conversions when working with memory addresses.
      - Consistency with pointer arithmetic: Rust ensures that array and slice indices align with pointer offsets, which are also usize.
*/