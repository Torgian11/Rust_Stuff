use std::{collections::HashMap, io::Split, str::SplitWhitespace};

fn median_number(numbers: &[i32]) -> f32 {
    let v:Vec<i32> = numbers.to_vec();
    let v_length = v.len();

    if &v_length % 2 == 0 {
        let num_one = v[&v_length / 2] as f32;
        let num_two = v[(&v_length / 2)+ 1] as f32;
        let median_number = (num_one + num_two) / 2.0;
        println!("{median_number} of {num_one} and {num_two}");
        median_number
    } else {
        let median_number = v[v_length / 2] as f32;
        println!("{median_number}");
        median_number
    }
}

fn number_mode(numbers: &[i32]) -> &i32 {
    let mut mapped_numbers = HashMap::new();

    for num in numbers {
        let count = mapped_numbers.entry(num).or_insert(0);
        *count += 1;
    }
    
    let max_number = mapped_numbers.iter().max_by_key(|&(_num, count)| count).map(|(&num, _count)| num);

    return max_number.unwrap();
}

// first => irst-fay
// apple => apple-hay

fn pig_latin(words: String) -> String {
    if words.len() < 1 {
        return String::from("Empty");
    }

    let cloned_string = words.clone();

    let piggy_string = cloned_string.split(" ");
    let mut pig_latin_string = String::new();

    for s in piggy_string {
        let mut new_string = String::from(s);
        let first_letter = &new_string.chars().nth(0);

        match first_letter.unwrap() {
            'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U' => {
                new_string.push_str("-hay ");
            },
            _ => {
                new_string.replace_range(..1, "");
                new_string.push_str("-");
                new_string.push_str(String::from(first_letter.unwrap()).as_str());
                new_string.push_str("ay ");
            }
        }
        pig_latin_string.push_str(&new_string);
    }
    println!("{pig_latin_string:?}");
    return pig_latin_string;
}

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Red");
    let team_score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{team_score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name.clone(), field_value.clone());

    println!("{field_name}");
    println!("{field_value}");
    println!("{map:?}");

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(30);

    println!("{scores:?}");

    let text = "Here is some crazy text that is crazy";
    let mut map2 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map2:?}");

    let mut sorted_nums = [1,1,2,3,6];
    sorted_nums.sort();
    median_number(&sorted_nums);
    number_mode(&sorted_nums);
    pig_latin("Here is a string of words".to_string());

}
