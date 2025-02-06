fn main() {
    let mut i = 0;
    while i < 5 { // while this condition is met
        println!("i = {}", i);
        i += 1; // shadowing
    }
}
