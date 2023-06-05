fn main() {
    let word = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("melon"),
    ];
    let v1 = filter_odd_length_strings(&word);
    let v2 = convert_to_uppercase(&word);

    println!("filter_odd_length_strings: {:?}", v1);
    println!("convert_to_uppercase: {:?}", v2)
}

fn filter_odd_length_strings(words: &Vec<String>) -> Vec<String> {
    words
        .into_iter()
        .filter(|w| w.len() % 2 == 1)
        .cloned()
        .collect()
}

fn convert_to_uppercase(words: &Vec<String>) -> Vec<String> {
    words.into_iter().map(|w| w.to_uppercase()).collect()
}

// [?] Trait를 쓰는기준? ex) Distance Trait을가지는 struct Point.
