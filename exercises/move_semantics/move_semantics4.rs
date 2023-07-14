// move_semantics4.rs
// Refactor this code so that instead of passing `vec0` into the `fill_vec` function,
// the Vector gets created in the function itself and passed back to the main
// function.
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand for a hint.



fn main() {
    let vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    let mut vec2 = vec1.clone(); // Create a mutable copy of vec1
    vec2.push(88);

    println!("{} has length {} content `{:?}`", "vec2", vec2.len(), vec2);
}

fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
