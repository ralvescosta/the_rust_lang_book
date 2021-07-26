use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;
fn main() {
    // Vec
    let a = [1, 2, 3];
    let mut v = Vec::<i32>::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 3];

    //Accessing element into a Vec

    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(v) => println!("The third element is {}", v),
        None => println!("There is no third element."),
    }
    // we cant have "a &mut" a and "&a" in the same time

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(2.2),
        SpreadSheetCell::Text(String::from("Text")),
    ];

    match &row[1] {
        SpreadSheetCell::Int(i) => println!("{}", i),
        _ => println!("Not a integer"),
    }

    ///////////////////////////////////
    // String are stored as a collection of utf-8 encoded bytes
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = format!("{}{}", s1, s2);

    // bytes
    for b in s3.bytes() {
        println!("{}", b);
    }

    // chars
    for c in s3.chars() {
        println!("{}", c);
    }

    for g in s3.graphemes(true) {
        println!("{}", g);
    }

    // HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);

    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(40);
}
