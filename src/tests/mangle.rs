use crate::mangle::*;

#[test]
fn replace_chars() {
    let res_ok = replace_char_at_index("ciao".to_owned(), 0, 'm');
    assert_eq!("miao".to_string(), res_ok);

    let res_noop = replace_char_at_index("ciao".to_owned(), 10, 't');
    assert_eq!("ciao".to_string(), res_noop);
}

#[test]
fn mangle_a_word() {
    let word = "ciao".to_string();
    let mut expected_result: Vec<String> = vec![
        "diao", "fiao", "xiao", "viao", "cuao", "coao", "cjao", "ckao", "ciqo", "ciwo", "ciso",
        "cizo", "ciai", "ciap", "ciak", "cial",
    ]
    .iter()
    .map(|s| String::from(*s))
    .collect();
    expected_result.sort();

    let mut mangled = mangle_word(word);
    mangled.sort();

    assert_eq!(mangled, expected_result);
}
