#[path = "./timing_our_code.rs"]
mod timing_our_code;

fn main() {
    let test1 = timing_our_code::add_up_to1(100000000);
    let test2 = timing_our_code::add_up_to2(100000000);

    println!("test 1: {}, \n test 2: {}", test1, test2);
}
