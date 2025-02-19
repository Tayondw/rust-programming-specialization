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

/*
Challenge Questions:
Modify the code to count the number of occurrences of each vowel (a, e, i, o, u) in the sentence. Print the count for each vowel individually.

Extend the program by implementing a function that takes a sentence as input and returns the longest word in the sentence. Invoke this function with the `sentence` variable and print the longest word.
*/

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();

    // Count occurrences of each vowel
    let mut vowel_counts = std::collections::HashMap::new(); // use HashMap<char, i32> to store count of vowels
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                *vowel_counts.entry(c).or_insert(0) += 1; // initializes the a count if the vowel is not in the map
                // the star is to dereference vowel counts once consumed or out of scope
            }
            _ => {
                continue;
            }
        }
    }

    // Print the vowel counts
    for vowel in ['a', 'e', 'i', 'o', 'u'] {
        println!("{}: {}", vowel, vowel_counts.get(&vowel).unwrap_or(&0)); // print the counts using this method to avoid None errors
    }

    // Find and print the longest word
    let longest_word = find_longest_word(&sentence);
    println!("Longest word: {}", longest_word);
}

// Function to find the longest word in a sentence
fn find_longest_word(sentence: &str) -> &str {
    sentence
        .split_whitespace() // Split into words
        .max_by_key(|word| word.len()) // Find the longest word - type of closure 
        .unwrap_or("") // Default to empty if no words
}
