fn main() {
    let text = caesar_shift("ZaZ", 56);
    println!("{}", text);
    println!("{}", caesar_shift(&text, 22));
}

fn caesar_shift(message: &str, shift: u8) -> String {
    let bytes = String::from(message)
        .bytes()
        .map(|byte| match byte {
            b'a'..=b'z' => ((byte - b'a' + shift) % 26) + b'a',
            b'A'..=b'Z' => ((byte - b'A' + shift) % 26) + b'A',
            other => other,
        })
        .collect::<Vec<_>>();
    String::from_utf8(bytes)
        .expect("Caesar shift should not return invalid UTF-8")
}
