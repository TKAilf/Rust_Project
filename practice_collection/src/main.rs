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
        .max_by_key(|&( _, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute mode for an empty list");
    println!("Mode: {}", mode);

    // ターミナルからの入力を受け取る
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut result = Vec::new();

    for word in input.trim().split_whitespace() {
        let first_letter = word.chars().next().unwrap().to_lowercase().to_string();
        if vowels.contains(&first_letter.chars().next().unwrap()) {
            result.push(format!("{}-hay", word));
        }else{
            result.push(format!("{}-{}ay", &word[1..], word.chars().next().unwrap()));
        }
    }
    println!("Pig Latin: {}", result.join(" "));

    println!("Add Name:");
    let mut input_name = String::new();
    std::io::stdin().read_line(&mut input_name).expect("Failed to read line");
    let input_name = input_name.trim().split_whitespace();
    let mut vec_name = Vec::new();
    vec_name.push(input_name);

    println!("Add Department:");
    let mut input_department = String::new();
    std::io::stdin().read_line(&mut input_department).expect("Failed to read line");
    let input_department = input_department.trim().split_whitespace();
    let mut vec_department = Vec::new();
    vec_department.push(input_department);

    if(vec_name.len() == vec_department.len()){
    }
}
