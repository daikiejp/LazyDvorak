pub fn get_python_code_tests() -> Vec<String> {
    vec![
        r#"def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)"#,
        r#"def quicksort(arr):
    if len(arr) < 2:
        return arr
    pivot = arr[0]
    less = [x for x in arr[1:] if x <= pivot]
    greater = [x for x in arr[1:] if x > pivot]
    return quicksort(less) + [pivot] + quicksort(greater)"#,
        r#"def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)"#,
        r#"def bubble_sort(arr):
    n = len(arr)
    for i in range(n):
        for j in range(0, n - i - 1):
            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
    return arr"#,
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}
