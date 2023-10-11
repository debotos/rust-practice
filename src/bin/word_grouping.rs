use std::collections::HashMap;

fn word_groupings(words: Vec<String>) -> Vec<Vec<String>> {
    let mut words_hash: HashMap<String, Vec<String>> = HashMap::new();

    let mut char_freq: Vec<i32> = vec![0; 26];

    for word in words {
        for c in word.to_lowercase().chars() {
            char_freq[c as usize - 'a' as usize] += 1;
        }

        let key: String = char_freq
            .into_iter()
            .map(|i: i32| i.to_string())
            .collect::<String>();
        words_hash.entry(key).or_insert(Vec::new()).push(word);

        char_freq = vec![0; 26];
    }

    for (key, value) in &words_hash {
        println!("Key # {:?} value {:?}", key, value);
    }

    words_hash.into_iter().map(|(_, v)| v).collect()
}

fn main() {
    let words: Vec<String> = vec![
        "the".to_string(),
        "teh".to_string(),
        "het".to_string(),
        "stupid".to_string(),
        "studpi".to_string(),
        "apple".to_string(),
        "appel".to_string(),
    ];

    let results = word_groupings(words);
    let input_word = String::from("teh");
    for i in results {
        if i.contains(&input_word) {
            println!("The group of the word is {:?}", i);
        }
    }
}
