fn main() {

    let mut answer:u32 = 0;

    for x in 0..1000 {
        if x%5 == 0 || x%3 == 0 {
            answer = answer + x;
        }
    }

    println!("{}", answer);
}
