mod edit_based;

fn main() {
    let s1 = "test";
    let s2 = "test";
    let d = edit_based::hamming::hamming(s1.chars(), s2.chars());
    println!("{:?}", d);
    println!("{:?}", s1);
    println!("{:?}", s2);
}
