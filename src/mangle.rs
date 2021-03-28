fn proximity_map(letter: char) -> Vec<char> {
    match letter {
        'a' => vec!['q', 'w', 's', 'z'],
        'b' => vec!['v', 'n', 'g', 'h'],
        'c' => vec!['x', 'v', 'd', 'f'],
        'd' => vec!['e', 'r', 's', 'f', 'x', 'c'],
        'e' => vec!['w', 'r', 's', 'd'],
        'f' => vec!['r', 't', 'd', 'g', 'c', 'v'],
        'g' => vec!['t', 'y', 'f', 'h', 'v', 'b'],
        'h' => vec!['y', 'u', 'g', 'j', 'b', 'n'],
        'i' => vec!['u', 'o', 'j', 'k'],
        'j' => vec!['u', 'i', 'h', 'k', 'n', 'm'],
        'k' => vec!['i', 'o', 'j', 'l', 'm'],
        'l' => vec!['o', 'p', 'k'],
        'm' => vec!['j', 'k', 'n'],
        'n' => vec!['h', 'j', 'b', 'm'],
        'o' => vec!['i', 'p', 'k', 'l'],
        'p' => vec!['o', 'l'],
        'q' => vec!['w', 'a'],
        'r' => vec!['e', 't', 'd', 'f'],
        's' => vec!['w', 'e', 'a', 'd', 'z', 'x'],
        't' => vec!['r', 'y', 'f', 'g'],
        'u' => vec!['y', 'i', 'h', 'j'],
        'v' => vec!['f', 'g', 'c', 'b'],
        'w' => vec!['q', 'e', 'a', 's'],
        'x' => vec!['s', 'd', 'z', 'c'],
        'y' => vec!['t', 'u', 'g', 'h'],
        'z' => vec!['a', 's', 'x'],
        '-' => vec!['-', '_'],
        other => vec![other],
    }
}

pub fn replace_char_at_index(word: String, i: usize, c: char) -> String {
    word.char_indices()
        .map(|(j, s)| if j == i { c } else { s })
        .collect()
}

pub fn mangle_word(word: String) -> Vec<String> {
    let alternatives: Vec<Vec<char>> = word.chars().map(proximity_map).collect();
    alternatives
        .iter()
        .enumerate()
        .map(|(i, alt)| {
            alt.iter()
                .map(|c| replace_char_at_index(word.clone(), i, *c))
                .collect::<Vec<String>>()
        })
        .flatten()
        .collect()
}
