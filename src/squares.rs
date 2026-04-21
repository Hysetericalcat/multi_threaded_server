pub fn square_iterator() -> Vec<i32> {
    let v = vec![1, 2, 3, -10, 11, -7, 8, 0, 9];
    v.iter()
        .filter(|x| **x >= 0)
        .map(|x| x * x)
        .filter(|x| *x > 10)
        .collect()
        //|x|->automatically infers types
}