#![feature(unboxed_closures, fn_traits)]

mod edit_based;
mod error;
// use edit_based::hamming::Hamming;
use edit_based::hamming::hamming;

fn main() {
    let ss1 = "test";
    let ss2 = "test";
    let s1 = "test".to_string();
    let s2 = "test".to_string();
    let v1 = vec![1, 2, 4, 4];
    let v2 = vec![1, 3, 3, 4];

    dbg!(hamming(&v1, &v2));
    dbg!(hamming(1, 3));
    dbg!(hamming(&v1[..], &v2[..]));
    dbg!(hamming(ss1, ss2));
    dbg!(hamming(&s1, &s2));
    // dbg!(s1.chars().collect::<Vec<_>>());
    //     // let d = edit_based::hamming::hamming(&s1[..], &s2[..]);
    //     // let d = edit_based::hamming::hamming(&v1, &v2);
    //     println!("{:?}", v1.hamming(&v2));
    //     println!("{:?}", s1.hamming(&s2));
    //     println!("{:?}", ss1.hamming(&ss2));
    // }
}
