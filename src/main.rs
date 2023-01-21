use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        num:u32,
        value:Chars,
    }

    for i in 1..=num - 1 {
        let mut last = 0;
        for l in 0..value.len() {
            if l + i as usize >= value.len() {
                break;
            }
            // println!("{} {} {} {}", i, l, value[l], value[l + i as usize]);
            if value[l] == value[l + i as usize] {
                break;
            }
            last = l + 1;
        }
        println!("{}", last);
    }
}
