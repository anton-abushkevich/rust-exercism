fn main() {
    let mut v = vec!();

    for _ in 0..2 {
        let s = String::from("0");
        v.push(s.as_str());
        println!("{:?}", v);
    }
}