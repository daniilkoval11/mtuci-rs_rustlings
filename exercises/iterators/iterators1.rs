// iterators1.rs
//
//  Make me compile by filling in the `???`s
//
// When performing operations on elements within a collection, iterators are essential.
// This module helps you get familiar with the structure of using an iterator and
// how to go through elements within an iterable collection.
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a hint.

fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.into_iter();   // Step 1

    assert_eq!(my_iterable_fav_fruits.next().map(|x| x.to_string()), Some("banana".to_string()));     // Step 2
    assert_eq!(my_iterable_fav_fruits.next().map(|x| x.to_string()), Some("custard apple".to_string()));     // Step 3
    assert_eq!(my_iterable_fav_fruits.next().map(|x| x.to_string()), Some("avocado".to_string()));     // Step 4
    assert_eq!(my_iterable_fav_fruits.next().map(|x| x.to_string()), Some("peach".to_string()));     // Step 5
    assert_eq!(my_iterable_fav_fruits.next().map(|x| x.to_string()), Some("raspberry".to_string()));     // Step 6
    assert_eq!(my_iterable_fav_fruits.next(), None);     // Step 7
}
