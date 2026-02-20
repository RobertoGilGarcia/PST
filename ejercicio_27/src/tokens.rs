pub fn next_token(s: &str, start: usize) -> Option<(usize, usize)> {
    let mut primera = None;
    let mut ultima = None;

    for (i, ch) in s[start..].char_indices() {
        let inicio = start + i;

        if ch != ' ' && primera.is_none() {
            primera = Some(inicio);
            if "()".contains(ch) {
                ultima = Some(inicio);
                break;
            }
        }
        if ch == ' ' && primera.is_some() {
            ultima = Some(inicio - 1);
            break;
        }
    }

    match (primera, ultima) {
        (Some(start), Some(end)) => Some((start, end)),
        (Some(inicio), None) => Some((inicio, s.len() - 1)),
        _ => None,
    }
}
