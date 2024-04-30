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

    println!("Enter a command:");
    let mut departments:HashMap<String, Vec<String>> = HashMap::new();
    loop{
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).expect("Failed to read line");
        let parts: Vec<&str> = command.trim().split_whitespace().collect();
    
        if parts.len() < 3 {
            println!("Invalid command. Please try again.");
            continue;
        }

        let parts_head = parts[0].to_uppercase();
        let parts_second = parts[1].to_string();
        let parts_third = parts[2].to_uppercase().to_string();
        let parts_forth = parts[3].to_string();

        match parts_head.as_str() {
            "ADD" => {
                if parts_third == "TO" {
                    departments.entry(parts_forth).or_insert_with(Vec::new).push(parts_second);
                } else {
                    println!("Invalid command. Please try again.");
                    continue;
                }
            },
            "LIST" => {
                if parts_second == "ALL" {
                    let mut all_employees = Vec::new();
                    all_employees.extend(departments.values().flat_map(|v| v.clone()));
                    for employees in departments.values() {
                        all_employees.extend(employees.clone());
                    }
                    all_employees.sort();
                    all_employees.iter().for_each(|employee| println!("{}", employee));
                }else{
                    match departments.get(&parts_second) {
                        Some(employees) => {
                            let mut sorted_employees = employees.clone();
                            sorted_employees.sort();
                            println!("Employees in {} department:", parts_second);
                            for employee in sorted_employees {
                                println!("{}", employee);
                            }
                        },
                        None => {
                            println!("No such department.");
                            continue;
                        }
                    }
                }
            },
            "QUIT" | "EXIT" => break,
            _ => {
                println!("Invalid command. Please try again.");
                continue;
            }
        }
    }
}
