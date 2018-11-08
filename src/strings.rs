use std::fs::File;
use std::io::Read;

pub fn nth_word(s: &str, n: usize) -> &str {

    let mut words_checked: usize = 0;
    let mut start: usize = 0;
    let mut end: usize;

    for (i, &item) in s.as_bytes().iter().enumerate() {
        let at_end = i == s.len() - 1;
        if item == b' ' || at_end { 
            words_checked += 1;
            end = if at_end { i +1 } else { i };
            if words_checked == n { return &s[start..end] }
            start = end + 1;
        }
    }

    return ""
}

pub fn get_file(filename: &str) -> String {
    let mut contents = String::new();

    // UNWRAP?
    let mut file = File::open(filename).unwrap();

    // UNWRAP?
    file.read_to_string(&mut contents).unwrap();
    contents
}
