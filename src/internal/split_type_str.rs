/// Strips the module path from every identifier in a `type_name`-style string,
/// keeping the `' '`, `','`, `'<'`, and `'>'` punctuation that separates them.
///
/// For example `a::Foo<b::Bar, c::Baz>` becomes `Foo<Bar, Baz>`.
pub fn split_type_str(string: &str) -> String {
    let mut result = String::new();
    let mut token_start = 0;

    for (index, ch) in string.char_indices() {
        if matches!(ch, ' ' | ',' | '<' | '>') {
            result.push_str(last_path_segment(&string[token_start..index]));
            result.push(ch);
            token_start = index + ch.len_utf8();
        }
    }

    result.push_str(last_path_segment(&string[token_start..]));
    result
}

/// Returns `token` with any leading `module::path::` prefix removed, i.e. the
/// portion after its final `::` separator.
fn last_path_segment(token: &str) -> &str {
    match token.rfind("::") {
        Some(index) => &token[index + "::".len()..],
        None => token,
    }
}
