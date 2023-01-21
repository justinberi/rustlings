// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

fn main() {
    let vec0 = Vec::new();
    // vec0.push(0);

    let mut vec1 = fill_vec(vec0.clone()); // This is a deep copy

    let mut vec2 = Vec::new();
    fill_vec2(&mut vec2);
    fill_vec2(&mut vec2);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    println!("{} has length {} content `{:?}`", "vec1", vec2.len(), vec2);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
// Same as below.
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    //let mut vec = vec;// 

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec2(vec: &mut Vec<i32>) {
    vec.push(33);
    vec.push(55);
    vec.push(66);
}