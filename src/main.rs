fn main() {
    let mut s = String::from("some words or whatever");

    let us = first_word(&mut s);

    println!("{}", us);

    let user = User {
        name: String::from("denis"),
        age: 27,
    };

    let is_adult = user.is_adult();

    print!("{} is adult? {}", user.name, is_adult.to_string())
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    let s2;
    {
        s2 = User::some_other_function();
    }
    println!("{}", s2);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s2.len()
}

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

impl User {
    fn is_adult(&self) -> bool {
        return self.age >= 18;
    }

    fn some_other_function<'a>() -> &'a str {
        return "tutturu";
    }
}