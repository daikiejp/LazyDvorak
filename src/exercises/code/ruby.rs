pub fn get_ruby_code_tests() -> Vec<String> {
    vec![
        r#"def fibonacci(n)
  return n if n <= 1
  fibonacci(n - 1) + fibonacci(n - 2)
end"#,
        r#"def quicksort(array)
  return array if array.length < 2
  pivot = array[0]
  less = array[1..].select { |x| x <= pivot }
  greater = array[1..].select { |x| x > pivot }
  quicksort(less) + [pivot] + quicksort(greater)
end"#,
        r#"def factorial(n)
  return 1 if n <= 1
  n * factorial(n - 1)
end"#,
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}
