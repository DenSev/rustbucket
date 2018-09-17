use ::Sex::MALE;
use std::env;
use std::fmt::Display;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mut s = String::from("some words or whatever");

    let us = first_word(&mut s);

    println!("{}", us);

    let user = User {
        name: String::from("denis"),
        age: 27,
        sex: MALE
    };

    let is_adult = user.is_adult();

    println!("{} is adult? {}", user.name, is_adult.to_string());
    println!("{}", user);

    let number_list = vec![34, 50, 25, 100, 65];

    println!("{}", largest(&number_list));
}


fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
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

//#[derive(Debug)]
enum Sex {
    MALE,
    FEMALE
}

impl Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Sex::MALE => write!(f, "male"),
            Sex::FEMALE => write!(f, "female")
        }
    }
}

//#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    sex: Sex
}

impl User {
    fn is_adult(&self) -> bool {
        return self.age >= 18;
    }

    fn some_other_function<'a>() -> &'a str {
        return "tutturu";
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "name: {}\nage: {}\nsex: {}", &self.name, &self.age, &self.sex)
    }
}