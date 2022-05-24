mod timing_our_code;
mod time_complexity;
mod space_complexity;
mod logarithms;

fn main() {
    let test1 = timing_our_code::add_up_to1(100000000);
    let test2 = timing_our_code::add_up_to2(100000000);

    println!("test 1: {}, \n test 2: {}", test1, test2);

    // counting_operations::count_up_and_down(10);
    // counting_operations::_log_at_most_5(0);
}
