fn main() {
    let mut v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("The element is {third}");

    let next_third: Option<&i32> = v.get(2);
    match next_third {
        Some(next_third) => println!("And the next third is {next_third}"),
        None => println!("Nothing found")
    }

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    println!("{v:?}");

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(1.3),
        SpreadsheetCell::Text(String::from("bluey")),
    ];

    println!("{row:?}");
}
