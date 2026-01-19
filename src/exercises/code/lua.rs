pub fn get_lua_code_tests() -> Vec<String> {
    vec![
        r#"function fibonacci(n)
  if n <= 1 then
    return n
  end
  return fibonacci(n - 1) + fibonacci(n - 2)
end"#,
        r#"local function quicksort(arr)
  if #arr < 2 then return arr end
  local pivot = arr[1]
  local less, greater = {}, {}
  for i = 2, #arr do
    if arr[i] <= pivot then
      table.insert(less, arr[i])
    else
      table.insert(greater, arr[i])
    end
  end
  return quicksort(less) .. {pivot} .. quicksort(greater)
end"#,
        r#"local function factorial(n)
  if n <= 1 then
    return 1
  end
  return n * factorial(n - 1)
end"#,
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}
