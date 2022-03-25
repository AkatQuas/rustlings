// vec1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute the command `rustlings hint vec1` if you need hints.
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    let a = [10, 20, 30, 40]; // a plain array
    let mut v = vec![10, 20, 30]; // TODO: declare your vector here with the macro for vectors
    v.push(40);

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
