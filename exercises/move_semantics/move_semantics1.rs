// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

fn main() {
    let vec0 = Vec::new();

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    let mut vec1 = fill_vec(vec0.clone());

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    println!("{} has length {} content `{:?}`", "vec", vec.len(), vec);
    vec
}

fn calculate_length(s: String) -> (String, usize) {
    let x = s;
    let length = x.len(); // len() returns the length of a String

    (x, length)
}
