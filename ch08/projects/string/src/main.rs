fn main() {
    let mut s = String::new();

    let data = "initial contents";

    // the method also works on a literal directly;
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // push_str
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);
}
