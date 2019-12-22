#[test]
pub fn one_word() {
    assert_eq!(tokenize("hello"), Ok(vec!["hello".into()]));
}

#[test]
pub fn two_words() {
    assert_eq!(
        tokenize("hello world"),
        Ok(vec!["hello".into(), "world".into()])
    );
}

#[test]
pub fn one_double_quoted_word() {
    assert_eq!(tokenize("\"hello\""), Ok(vec!["hello".into()]));
}

#[test]
pub fn one_spaced_double_quoted_word() {
    assert_eq!(tokenize("\"hello world\""), Ok(vec!["hello world".into()]));
}

#[test]
pub fn two_double_quoted_words() {
    assert_eq!(tokenize("\"hello\" \"world\""), Ok(vec!["hello".into(), "world".into()]));
}

#[test]
pub fn one_single_quoted_word() {
    assert_eq!(tokenize("'hello'"), Ok(vec!["hello".into()]));
}

#[test]
pub fn one_spaced_single_quoted_word() {
    assert_eq!(tokenize("'hello world'"), Ok(vec!["hello world".into()]));
}

#[test]
pub fn two_single_quoted_words() {
    assert_eq!(tokenize("'hello' 'world'"), Ok(vec!["hello".into(), "world".into()]));
}

#[test]
pub fn nested_single_double_quote() {
    assert_eq!(tokenize("'hello \"friend\" world'"), Ok(vec!["hello \"friend\" world".into()]));
}

#[test]
pub fn nested_double_single_quote() {
    assert_eq!(tokenize("\"hello 'friend' world\""), Ok(vec!["hello 'friend' world".into()]));
}

#[test]
pub fn escaped_single_quote() {
    assert_eq!(tokenize("\\'"), Ok(vec!["'".into()]));
}

#[test]
pub fn escaped_double_quote() {
    assert_eq!(tokenize("\\\""), Ok(vec!["\"".into()]));
}

#[test]
pub fn interpolated_double_quoted() {
    assert_eq!(tokenize("hello\" friend \"bob"), Ok(vec!["hello friend bob".into()]));
}
