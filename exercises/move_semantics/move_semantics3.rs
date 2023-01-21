// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand for a hint.

fn main() {
    let x = 1;
    let mut y = x;

    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// The value is moved into this scope which is mutable just like 
// let x = 1; // variable x has value 1 and is imutable
// let mut y = x; // variable y now owns the value an is mutable
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> { 
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
