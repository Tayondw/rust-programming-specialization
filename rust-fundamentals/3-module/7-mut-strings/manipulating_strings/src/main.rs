fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string(); // a string slice but .to_string() turns into a String type
    // Use slicing to get the first three characters of the sentence
    //println!("{}", &sentence[0..4]);

    // Use slicing to get the first four characters of the sentence
    //println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence); // String Type
    //println!("{}", description);

    // iterate over the characters in the sentence
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
            _ => {
                continue;
            }
            // _ means a wildcard pattern that acts as a catch all for any values not explicitly handled by previous patterns
            // the wildcard (_) matches all other characters (consonants, spaces, punctuation, etc)
            // the continue means that if anything that is not a vowel then move to or loop to the next iteration without doing anything
            // thus the wildcard makes sure that every character is accounted for, which prevents a compiler error due to non-exhaustive pattern matching
        }
    }

    // Split and collect into a vector
    // split the words into whitespace and collect them into a vector that holds strings
    // similar to .split(" ") in JS b/c it splits and places them in an array
    //let words: Vec<&str> = sentence.split_whitespace().collect(); // this is the way of defining a variable and saying exactly what you are expecting
    let words = sentence.split(' ').collect::<Vec<&str>>(); // when you collect this - make it a vector of string slices - must be string slices because you are referencing a point in memory
    //println!("{:?}", words);

    // get the characters of the string, reverse the characters of each word in the sentence, then turn them into a string collection (like join)
    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);
}
