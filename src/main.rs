fn main() {
    let mut s = String::from("some words or whatever");

    let us = first_word(&mut s);

    println!("{}", us)
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}