fn ownership() {
    let numbers = vec![1, 2, 3];
    let slice = &numbers[..]; // creates a slice of all elements in numbers, fixed in size so are immutable if the vec referenced is immutable
    // the .. means we want the full range of what is inside the vector

    println!("slice = {:?}", slice);
}

fn modifiable() {
    let mut numbers = vec![1, 2, 3];
    let slice = &mut numbers[..]; // creates a mutable slice of all elements in numbers
    slice[0] = 10;

    // This would fail!
    let other_slice = &numbers[..]; // since slice is mutable, we can't borrow numbers immutably while slice is still in use

    println!("slice = {:?}", slice);
}

fn main() {
    // slices and vectors are similar. But slices are immutable depending on how they are borrowed
    //ownership();
    modifiable();
}

// Use slices when:
// - you want to borrow a portion of a collection rather than the whole collection
// - you want to pass around a reference to a sequence of items without copying them
// - you want to access a subset of a collection without copying
// Use vectors when:
// - you need to dynamically grow or shrink your collection
// - you need to own the collection and transfer ownership to another scope
