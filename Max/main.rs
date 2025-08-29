fn stalin_sort<T: PartialOrd + Copy>(arr: &[T]) -> Vec<T> {
    let mut result = Vec::new();
    if let Some(&first) = arr.first() {
        result.push(first);
        let mut last = first;

        for &item in &arr[1..] {
            if item >= last {
                result.push(item);
                last = item;
            }
        }
    }
    result
}

fn main() {
    let numbers = vec![5, 1, 2, 3, 0, 4, 7, 6, 8];
    let sorted = stalin_sort(&numbers);
    println!("Input: {:?}", numbers);
    println!("Stalin sorted: {:?}", sorted);
}
