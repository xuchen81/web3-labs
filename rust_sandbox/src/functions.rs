pub fn run() {
    greeting("Hello", "Jane");

    // Bind function values to variables
    let get_sum = add(5, 4);
    println!("Sum: {}", get_sum);

    // Clousure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(3, 8));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to mee you!", greet, name)
}

fn add (n1: i32, n2: i32) -> i32 {
    n1 + n2
}
