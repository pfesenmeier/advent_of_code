pub fn parse_csv(input: &str) -> Vec<u32> {
    input
        .trim_end()
        .split(',')
        // .inspect(|f| println!("line: {}", f))
        .into_iter()
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect()
}

#[test]
fn test_parse() {
    assert_eq!(parse_csv("1,2,3"), vec![1, 2, 3]);
}
