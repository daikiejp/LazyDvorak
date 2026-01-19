pub fn get_rust_code_tests() -> Vec<String> {
    vec![
        r#"fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}"#,
        r#"fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = partition(arr);
    quicksort(&mut arr[0..pivot]);
    quicksort(&mut arr[pivot + 1..]);
}"#,
        r#"fn factorial(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    n * factorial(n - 1)
}"#,
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}
