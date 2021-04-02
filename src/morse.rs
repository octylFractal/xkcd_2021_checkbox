use once_cell::sync::Lazy;
use std::collections::HashMap;

static ASCII_TO_MORSE_TABLE: Lazy<HashMap<char, &str>> = Lazy::new(|| {
    let mut table = HashMap::new();

    table.insert('0', "-----");
    table.insert('1', ".----");
    table.insert('2', "..---");
    table.insert('3', "...--");
    table.insert('4', "....-");
    table.insert('5', ".....");
    table.insert('6', "-....");
    table.insert('7', "--...");
    table.insert('8', "---..");
    table.insert('9', "----.");
    table.insert('A', ".-");
    table.insert('B', "-...");
    table.insert('C', "-.-.");
    table.insert('D', "-..");
    table.insert('E', ".");
    table.insert('F', "..-.");
    table.insert('G', "--.");
    table.insert('H', "....");
    table.insert('I', "..");
    table.insert('J', ".---");
    table.insert('K', "-.-");
    table.insert('L', ".-..");
    table.insert('M', "--");
    table.insert('N', "-.");
    table.insert('O', "---");
    table.insert('P', ".--.");
    table.insert('Q', "--.-");
    table.insert('R', ".-.");
    table.insert('S', "...");
    table.insert('T', "-");
    table.insert('U', "..-");
    table.insert('V', "...-");
    table.insert('W', ".--");
    table.insert('X', "-..-");
    table.insert('Y', "-.--");
    table.insert('Z', "--..");
    table.insert('.', ".-.-.-");
    table.insert(',', "--..--");
    table.insert('?', "..--..");
    table.insert('\'', ".----.");
    table.insert('!', "-.-.--");
    table.insert('/', "-..-.");
    table.insert('(', "-.--.");
    table.insert(')', "-.--.-");
    table.insert('&', ".-...");
    table.insert(':', "---...");
    table.insert(';', "-.-.-.");
    table.insert('=', "-...-");
    table.insert('+', ".-.-.");
    table.insert('-', "-....-");
    table.insert('_', "..--.-");
    table.insert('"', ".-..-.");
    table.insert('$', "...-..-");
    table.insert('@', ".--.-.");

    table
});

static MORSE_TO_ASCII_TABLE : Lazy<HashMap<&str, char>> = Lazy::new(|| {
    let mut table = HashMap::with_capacity(ASCII_TO_MORSE_TABLE.len());

    for (k, v) in ASCII_TO_MORSE_TABLE.iter() {
        table.insert(*v, *k);
    }

    table
});

pub fn morse_to_ascii(morse: &str) -> String {
    // average morse code (not weighted by common characters) is 4 characters
    let mut ascii = String::with_capacity(morse.len() / 4);

    for part in morse.split(' ') {
        let char = MORSE_TO_ASCII_TABLE.get(&part)
            .unwrap_or_else(|| panic!("No ASCII value for {:?}", part));
        ascii.push(*char);
    }

    ascii
}

pub fn ascii_to_morse(ascii: &str) -> String {
    // average morse code (not weighted by common characters) is 4 characters
    // plus we know there is (n-1) spaces, so `n * 4 + (n - 1)` = `n * 4 + n - 1` = `n * 5 - 1`
    let mut morse = String::with_capacity(ascii.len() * 5 - 1);

    for part in ascii.chars().map(|c| c.to_ascii_uppercase()) {
        let char = ASCII_TO_MORSE_TABLE.get(&part)
            .unwrap_or_else(|| panic!("No Morse value for {:?}", part));
        morse += char;
        morse += " ";
    }

    morse.trim_end().to_string()
}
