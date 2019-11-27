use std::vec::Vec;

fn main() {

    let mut values = Vec::new();

    for x in 0..1000 {
        if x%5 == 0 || x%3 == 0 {
            values.push(x);
        }
    }

    let mut answer:u32 = 0;

    for y in values {
        answer = answer + y;
    }

    println!("{}", answer);

}
