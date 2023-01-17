 use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32), 
    Float(f64), 
    Text(String),
}

fn main() {
    
    //Vec<T> => vector
    // s
    let mut v: Vec<i32> = Vec::new();

    //using macros
    let v1 = vec![1, 2, 3, 4];

    //updating
    for i in 1..=4 {
        v.push(i);
    }

    println!("{:?}, \n{:?}", v, v1);

    //getting an element. 

    //[]:  panics when it goes out of bound.
    let third = &v1[2];
    println!("The third element is: {}", third);

    // get(): return None if goes out of bound.
    let third: Option<&i32> = v1.get(10);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("Out of bound. The len of the vector is {}", v1.len()),
    }

    // iterating through a list. 

    // mutable reference iteration. 

    for i in &v {
        print!("{i} ");
    }
    println!("");

    // immutable reference iteration

    for i in &mut v {
        *i += 5;
        print!("{i} ");
    }
    println!("\n");

    let mut row = vec![
        SpreadsheetCell::Int(4), 
        SpreadsheetCell::Float(4.0),
        SpreadsheetCell::Text(String::from("blue")), 
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(90.0),
    ];

    for i in &row {
        println!("{:#?}", i);
    }

    // popping an element from a vector.
    while let Some(top) = row.pop() {
        println!("{:?}", top);
    }

    // strings type. 
    working_strings();

    // hash map. 

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // iterate
    for (key, value) in &scores {
        println!("{key}, {value}")
    }



}

fn working_strings() {

    // creating strings; 
    let mut s1 = String::new();
    let s2 = String::from("using from method");
    let s3 = "using to_string method".to_string();

    println!("\nEmpty {s1}, \n{s2}, \n{s3}");

    //updating a string; 
    s1.push_str("pushing to a string using push_str method.");
    println!("{s1}");

    // concatenating
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s2} {s3}");

    // indexing a string. 
    let hello = String::from("Здравствуйте");
    println!("{}", &hello[0..4]);

    // iterationg
    //chars
    for c in "Зд".chars() {
        println!("{c}");
    }

    //bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
