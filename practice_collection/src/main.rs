use std::collections::HashMap;
fn main() {
    let mut int_list = vec![1, 2, 3, 4, 5, 1, 2, 2, 3, 3, 3];
    let total: i32 = int_list.iter().sum();
    let length: usize = int_list.len();
    let mean = total / length as i32;
    println!("Mean: {}", mean);

    int_list.sort();
    let median;
    if length % 2 == 1 {
        median = int_list[length / 2];
    }else{
        median = int_list[length / 2 - 1];
    }
    println!("Median: {}", median);

    let mut occurrences = HashMap::new();
    for &value in &int_list {
        *occurrences.entry(value).or_insert(0) += 1;
    }
    let mode = occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute mode for an empty list");
    print!("Mode: {}", mode);
}
