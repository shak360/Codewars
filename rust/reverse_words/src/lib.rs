pub fn reverse_words(str: &str) -> String {
    let mut total_collector = Vec::new();
    let mut mini_collector = Vec::new();
    for (i, c) in str.chars().enumerate() {
        if c.is_whitespace() {
            if !mini_collector.is_empty() {
                mini_collector.reverse();
                total_collector.append(&mut mini_collector);
                mini_collector.truncate(0);
            }
            total_collector.push(c);
        } else {
            mini_collector.push(c);
        }

        if i == str.len() - 1 {
            mini_collector.reverse();
            total_collector.append(&mut mini_collector);
            mini_collector.truncate(0)
        }
    }
    total_collector.into_iter().collect::<String>()
}

pub fn reverse_words2(str: &str) -> String {
    /*

    The following one-liner in the function only works because of
    the following trick:

    ```Rust
    let x = "    a  b c".to_string();
    let d: Vec<_> = x.split(' ').collect();
    assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
    ```

    Note that the string has 4 spaces before a, 2 spaces before b,
    and 1 space before c. However, when split, the resulting vector
    has 4 spaces, then a, then 1 space, then b, then 0 spaces, then c.

    Contiguous separators can lead to possibly surprising behavior
    when whitespace is used as the separator. This trick allows this
    to work on sentences with an arbitrary number of spaces in between
    words.

    */

    str.split(' ')
        .map(|word| word.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}
