

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let s3 = String::from("world");

    let len2 = calculate_length_new(&s3);

    println!("The length of '{}' is {}", s3, len2);

    let mut s4 = String::from("hello");

    change(&mut s4);

    println!("{}", s4);

    let word = String::from("hello world");

    //let first_word = first_word(&word);
    let sword = "hello world";
    let first_word = first_word(&sword);

    println!("{}", first_word);

}


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_new(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}