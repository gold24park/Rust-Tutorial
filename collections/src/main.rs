use std::{
    collections::{hash_map::Entry, HashMap},
    io,
};

fn main() {
    // न म स् ते
    // bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // scalar values
    // ['न', 'म', 'स', '्', 'त', 'े']
    // grapheme clusters (문자소, 말이 어려운데, "letter"에 가까움)
    // ["न", "म", "स्", "ते"]

    // Expected: 30, 50
    let array = [20, 10, 30, 50, 50];
    let (median, mode) = exercise1(&array);
    println!("Exercise1: {}, {}", median, mode);

    // Expected: irst-fay, apple-hay
    let word1 = String::from("first");
    let word2 = String::from("apple");
    println!("Exercise2: {}, {}", exercise2(&word1), exercise2(&word2));

    let mut company: HashMap<String, String> = HashMap::new();
    let command1 = String::from("Add Sally to Engineering");
    let command2 = String::from("Add Amir to Sales");
    exercise3(&command1, &mut company);
    exercise3(&command2, &mut company);

    exercise3_answer();
}

// @ref: https://doc.rust-lang.org/book/ch08-03-hash-maps.html
fn exercise1_answer(numbers: &Vec<i32>) {
    let mut max = std::i32::MIN;
    let mut min = std::i32::MAX;
    let mut sum = 0;

    let mut mode_map: HashMap<i32, u32> = HashMap::new();

    for item in numbers {
        sum += *item;

        let val = mode_map.entry(*item).or_default();
        *val += 1;

        if *item < min {
            min = *item;
        }

        if *item > max {
            max = *item;
        }
    }

    let mut numbers = numbers.clone();
    numbers.sort();

    let len = numbers.len();
    let middle_position = len / 2;
    let mean: f64 = sum as f64 / len as f64;
    println!("mean: {}", mean);

    let medium = numbers.get(middle_position).unwrap();
    println!("middle_position: {}", middle_position);
    println!("medium: {}", medium);

    let mut mode = 0;
    let mut max_occurs = std::u32::MIN;

    for (k, v) in &mode_map {
        if *v > max_occurs {
            max_occurs = *v;
            mode = *k;
        }
    }
    println!("sorted numbers: {:?}", &numbers);
    println!("mode_map: {:#?}", &mode_map);
    println!("mode: {:?} (occurs {} times)", mode, max_occurs);
}

// Given a list of integers, returns median and mode of the list
fn exercise1(array: &[i32]) -> (i32, i32) {
    if array.len() == 0 {
        panic!("The array is empty.");
    }

    let mut list = array.to_vec();
    list.sort();
    let median = list.get(list.len() / 2).copied().unwrap_or(0);

    let mut max_count = 0;
    let mut mode = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in list {
        let count = map.entry(i).or_insert(0);
        *count += 1;

        if *count > max_count {
            max_count = *count;
            mode = i;
        }
    }

    (median, mode)
}

// Convert strings to pig latin
// first -> irst-fay
// apple -> apple-hay
fn exercise2_answer(word: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut char_iter = word.chars();
    let first_letter = char_iter.next().unwrap();

    if vowels.contains(&first_letter) {
        format!("{}-hay", &word)
    } else {
        let remaining: String = char_iter.take(word.len() - 1).collect();
        format!("{}-{}ay", remaining, word)
    }
}

fn exercise2(word: &str) -> String {
    let mut pig_string = String::new();

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut is_vowel = false;
    for char in word.chars() {
        is_vowel = vowels.contains(&char);
        break;
    }

    if is_vowel {
        pig_string = format!("{}-hay", word);
    } else {
        pig_string = format!("{}-{}ay", &word[1..], &word[..1])
    }

    pig_string
}

fn exercise3(cmd: &str, company: &mut HashMap<String, String>) {
    let mut words = cmd.split_whitespace();
    match words.next() {
        Some(action) => {
            if action != "Add" {
                panic!("Unknown command.")
            }
        }
        None => panic!("There's no input."),
    }
    let name = match words.next() {
        Some(name) => name,
        None => panic!("Name not found."),
    };

    match words.next() {
        Some(target_action) => {
            if target_action != "to" {
                panic!("Unknown target action.")
            }
        }
        None => panic!("There's no target action"),
    }

    let department = match words.next() {
        Some(department) => department,
        None => panic!("Department not found."),
    };

    company
        .entry(String::from(name))
        .or_insert(String::from(department));

    println!("=========================");
    for member in company.iter() {
        println!("{} | {}", member.0, member.1);
    }
    println!("=========================");
}

fn exercise3_answer() {
    let mut user_map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Plase enter a command: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Falied to read line");

        let input: Vec<&str> = input.trim().split(' ').collect();
        let operator = input.get(0).map(|x| x.to_lowercase());

        match operator.as_deref() {
            Some("add") => add_user(input.get(1).copied(), input.get(3).copied(), &mut user_map),
            Some("view") => match input.get(1) {
                Some(department) => list_users(department, &user_map),
                None => println!("Plase enter all or a departement!"),
            },
            Some("exit") => break,
            Some(_) | None => println!("Invalid command"),
        }
    }
}

fn add_user(
    name: Option<&str>,
    department: Option<&str>,
    user_map: &mut HashMap<String, Vec<String>>,
) {
    match (name, department) {
        (Some(name), Some(department)) => {
            let user_vec = user_map.entry(department.to_string()).or_insert(Vec::new());
            user_vec.push(name.to_string());
            user_vec.sort();
        }
        (None, Some(_)) => println!("Please enter a name!"),
        (Some(_), None) => println!("Please enter a department"),
        (None, None) => println!("Please enter all or a department!"),
    }
}

fn list_users(department: &str, user_map: &HashMap<String, Vec<String>>) {
    if department == "All" || department == "all" {
        for (key, value) in user_map {
            println!("Department - {}:", key);
            for user in value {
                println!(" •{}", user);
            }
        }
    } else {
        let users = user_map.get(department);
        match users {
            Some(department) => {
                for user in department {
                    println!(" •{}", user);
                }
            }
            None => println!("Department doesn't exist, or it's empty."),
        }
    }
}
