pub fn run() {
    let data = vec![-1, -2, 4, 8, 10, 12, 15, 16, 18];
    let result = search(data, 18);

    println!("Picked: {}", result);
}

fn search(vector: Vec<i8>, value: i8) -> i8 {
    let mut min = 0;
    let mut max = vector.len() - 1;

    while min <= max {
        let divide = (min + max) / 2;

        println!("Min: {} | Max: {}", min, max);
        println!("Divide: {}", divide);

        if vector[divide] < value {
            min = divide + 1;
        } else if vector[divide] > value {
            max = divide - 1;
        } else {
            return divide as i8;
        }
    }

    return -1;
}
