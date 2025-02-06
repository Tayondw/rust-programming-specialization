fn main() {
    // the for loop using a range. Note you can use also `(1..10)` or `(1..=10)` 1..10 is exclusive of the last digit, 1..=10 is inclusive of the last digit
    // for i in 1..=10 {
    //     println!("i = {}", i);
    // }

    // for i in (1..=5).rev() { // reverse the order of printing
    //     println!("{}", i);
    // }

    let numbers = vec![1, 2, 3, 4, 5];
    for n in numbers { // iterating over a collection in this case a vector - similar to array in javascript
        println!("{}", n);
    }
}
