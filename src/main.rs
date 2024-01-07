fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    let mut sorted = false;
    let mut n = arr.len() - 1;
    println!("format {n} test");

    while !sorted {
        sorted = true;
        for i in 0..n {
            println!(); // prints just a newline
            println!("{i} index");

            if arr[i] > arr[i + i] {
                arr.swap(i, i + 1);
                sorted = false
            }
        }
        n -= 1;
    }
}

fn main() {
    println!("Hello");
    let mut ve1 = vec![6, 5, 4, 3, 2, 1];

    bubble_sort(&mut ve1);
}
