pub fn run() {
    let mut data: Vec<i8> = vec![1, 1, 1, 1, 1, 2, 3];

    let result = count_unique_values(&mut data);
    println!("{result}");
}

fn count_unique_values(arr: &mut Vec<i8>) -> usize {
    if arr.len() == 0 {
        return 0;
    }

    let mut i = 0;

    for idx in 1..arr.len() {
        if arr[i] != arr[idx] {
            // 1. move forward
            i = i + 1;

            // 2. assign the unique value to the
            // because it's a duplicate of the previous.
            arr[i] = arr[idx];
        }
        println!("{:?}", arr);
    }

    i + 1
}
