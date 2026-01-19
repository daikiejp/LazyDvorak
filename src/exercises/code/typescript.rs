pub fn get_typescript_code_tests() -> Vec<String> {
    vec![
        r#"function fibonacci(n: number): number {
  if (n <= 1) {
    return n;
  }
  return fibonacci(n - 1) + fibonacci(n - 2);
}"#,
        r#"function quicksort(arr: number[]): number[] {
  if (arr.length < 2) return arr;
  const pivot = arr[0];
  const less = arr.slice(1).filter(x => x <= pivot);
  const greater = arr.slice(1).filter(x => x > pivot);
  return [...quicksort(less), pivot, ...quicksort(greater)];
}"#,
        r#"function factorial(n: number): number {
  if (n <= 1) {
    return 1;
  }
  return n * factorial(n - 1);
}"#,
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}
