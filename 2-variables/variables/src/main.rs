fn main() {
    let message = "Name: Alfredo, Weight: ";
    let weight = 190.0; // has to be a float to divide by 2.2, cannot be 190

    let kilos = weight / 2.2;
    println!("{}{}", message, weight);
    println!("{}{}", message, kilos);
}
