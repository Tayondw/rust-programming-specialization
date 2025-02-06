enum Shape {
    Circle(f64),
    Square(f64),
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0)];

    let total_area: f64 = shapes
        .iter() // Iterate over the shapes
        .map(|shape| { // |shape| is a closure that takes a reference to a Shape and returns a f64
            match shape {
                Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
                Shape::Square(length) => length * length,
            }
        })
        .sum();

    println!("Total area: {} sq. units", total_area);
}
