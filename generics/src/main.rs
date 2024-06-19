fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest::<i32>(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 25, 100, 65];
    let result = largest::<i32>(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest::<char>(&char_list);
    println!("The largest char is {result}");
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
