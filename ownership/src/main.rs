
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calulate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}


fn calulate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}